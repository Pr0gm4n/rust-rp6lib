//! Adapted from `<https://github.com/Rahix/av-device/blob/main/src/interrupt.rs>` for the Robby RP6 (ATmega32)

mod vector {
    pub fn lookup_vector(interrupt: &str) -> Option<usize> {
        match interrupt {
            "RESET" => Some(0),
            "INT0" => Some(1),
            "INT1" => Some(2),
            "INT2" => Some(3),
            "TIMER2_COMP" => Some(4),
            "TIMER2_OVF" => Some(5),
            "TIMER1_CAPT" => Some(6),
            "TIMER1_COMPA" => Some(7),
            "TIMER1_COMPB" => Some(8),
            "TIMER1_OVF" => Some(9),
            "TIMER0_COMP" => Some(10),
            "TIMER0_OVF" => Some(11),
            "SPI_STC" => Some(12),
            "USART_RXC" => Some(13),
            "USART_UDRE" => Some(14),
            "USART_TXC" => Some(15),
            "ADC" => Some(16),
            "EE_RDY" => Some(17),
            "ANA_COMP" => Some(18),
            "TWI" => Some(19),
            "SPM_RDY" => Some(20),
            _ => None,
        }
    }
}

use syn::spanned::Spanned;

/// Allows to define the entry point of the program by annotating a function with this macro.
#[proc_macro_attribute]
pub fn entry(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut f = syn::parse_macro_input!(input as syn::ItemFn);

    // check the function signature
    let valid_signature = f.sig.constness.is_none()
        && f.vis == syn::Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.inputs.is_empty()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
            syn::ReturnType::Default => false,
            syn::ReturnType::Type(_, ref ty) => matches!(**ty, syn::Type::Never(_)),
        };

    if !valid_signature {
        return syn::parse::Error::new(
            f.span(),
            "`#[entry]` function must have signature `[unsafe] fn() -> !`",
        )
        .to_compile_error()
        .into();
    }

    if !args.is_empty() {
        return syn::parse::Error::new(
            proc_macro2::Span::call_site(),
            "This attribute accepts no arguments",
        )
        .to_compile_error()
        .into();
    }

    let (statics, stmts) = match extract_static_muts(f.block.stmts) {
        Err(e) => return e.to_compile_error().into(),
        Ok(x) => x,
    };

    // Rename the function so it is not callable
    f.sig.ident = syn::Ident::new(
        &format!("__rp6_{}", f.sig.ident),
        proc_macro2::Span::call_site(),
    );
    f.sig.inputs.extend(statics.iter().map(|statik| {
        let ident = &statik.ident;
        let ty = &statik.ty;
        let attrs = &statik.attrs;

        // Note that we use an explicit `'static` lifetime for the entry point arguments. This makes
        // it more flexible, and is sound here, since the entry will not be called again, ever.
        syn::parse::<syn::FnArg>(
            quote::quote!(#[allow(non_snake_case)] #(#attrs)* #ident: &'static mut #ty).into(),
        )
        .unwrap()
    }));
    f.block.stmts = stmts;

    let tramp_ident = syn::Ident::new(
        &format!("{}_trampoline", f.sig.ident),
        proc_macro2::Span::call_site(),
    );
    let ident = &f.sig.ident;

    let resource_args = statics
        .iter()
        .map(|statik| {
            let (ref cfgs, ref attrs) = extract_cfgs(statik.attrs.clone());
            let ident = &statik.ident;
            let ty = &statik.ty;
            let expr = &statik.expr;
            quote::quote! {
                #(#cfgs)*
                {
                    #(#attrs)*
                    static mut #ident: #ty = #expr;
                    &mut #ident
                }
            }
        })
        .collect::<Vec<_>>();

    quote::quote! (
        #[cfg(not(any(doc, target_arch = "avr")))]
        compile_error!(
            "Ensure that you are using an AVR target! You may need to change \
       directories or pass a --target flag to cargo. See
       https://github.com/Rahix/avr-device/pull/41 for more details."
        );

        #[doc(hidden)]
        #[export_name = "main"]
        pub unsafe extern "C" fn #tramp_ident() {
            #ident(
                #(#resource_args),*
            )
        }

        #[doc(hidden)]
        #f
    )
    .into()
}

/// Allows to define an interrupt service routine (ISR) by annotating a function with this macro.
/// By convention, the function must be named like the handled interrupt.
///
/// For example:
/// ```rust
/// #[interrupt]
/// fn INT0() {
///     ...
/// }
/// ```
///
/// The available interrupts on the RP6 (ATmega32) are:
/// ```
/// RESET
/// INT0
/// INT1
/// INT2
/// TIMER2_COMP
/// TIMER2_OVF
/// TIMER1_CAPT
/// TIMER1_COMPA
/// TIMER1_COMPB
/// TIMER1_OVF
/// TIMER0_COMP
/// TIMER0_OVF
/// SPI_STC
/// USART_RXC
/// USART_UDRE
/// USART_TXC
/// ADC
/// EE_RDY
/// ANA_COMP
/// TWI
/// SPM_RDY
#[proc_macro_attribute]
pub fn interrupt(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut f: syn::ItemFn =
        syn::parse(input).expect("`#[interrupt]` must be applied to a function");

    let fspan = f.span();
    let ident = f.sig.ident.clone();
    let ident_s = ident.to_string();

    let valid_signature = f.sig.constness.is_none()
        && f.vis == syn::Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.inputs.is_empty()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
            syn::ReturnType::Default => true,
            syn::ReturnType::Type(_, ref ty) => match **ty {
                syn::Type::Tuple(ref tuple) => tuple.elems.is_empty(),
                syn::Type::Never(..) => true,
                _ => false,
            },
        };

    if !valid_signature {
        return syn::parse::Error::new(
            fspan,
            "`#[interrupt]` handlers must have signature `[unsafe] fn() [-> !]`",
        )
        .to_compile_error()
        .into();
    }

    let (statics, stmts) = match extract_static_muts(f.block.stmts.iter().cloned()) {
        Err(e) => return e.to_compile_error().into(),
        Ok(x) => x,
    };

    f.sig.ident = syn::Ident::new(
        &format!("__rp6_{}", f.sig.ident),
        proc_macro2::Span::call_site(),
    );
    f.sig.inputs.extend(statics.iter().map(|statik| {
        let ident = &statik.ident;
        let ty = &statik.ty;
        let attrs = &statik.attrs;
        syn::parse::<syn::FnArg>(
            quote::quote!(#[allow(non_snake_case)] #(#attrs)* #ident: &mut #ty).into(),
        )
        .unwrap()
    }));
    f.block.stmts = stmts;

    let tramp_ident = syn::Ident::new(
        &format!("{}_trampoline", f.sig.ident),
        proc_macro2::Span::call_site(),
    );
    let ident = &f.sig.ident;

    let resource_args = statics
        .iter()
        .map(|statik| {
            let (ref cfgs, ref attrs) = extract_cfgs(statik.attrs.clone());
            let ident = &statik.ident;
            let ty = &statik.ty;
            let expr = &statik.expr;
            quote::quote! {
                #(#cfgs)*
                {
                    #(#attrs)*
                    static mut #ident: #ty = #expr;
                    &mut #ident
                }
            }
        })
        .collect::<Vec<_>>();

    let vect = if let Some(v) = vector::lookup_vector(&ident_s) {
        v
    } else {
        return syn::parse::Error::new(
            proc_macro2::Span::call_site(),
            &format!("Interrupt `{}` unknown", ident_s),
        )
        .to_compile_error()
        .into();
    };
    let vector = format!("__vector_{}", vect);
    let vector_ident = syn::Ident::new(&vector, proc_macro2::Span::call_site());
    let vector_ident_s = vector_ident.to_string();

    quote::quote! (
        #[doc(hidden)]
        #[export_name = #vector_ident_s]
        pub unsafe extern "avr-interrupt" fn #tramp_ident() {
            #ident(
                #(#resource_args),*
            )
        }

        #[doc(hidden)]
        #f
    )
    .into()
}

/// Extracts `static mut` vars from the beginning of the given statements
fn extract_static_muts(
    stmts: impl IntoIterator<Item = syn::Stmt>,
) -> Result<(Vec<syn::ItemStatic>, Vec<syn::Stmt>), syn::parse::Error> {
    let mut istmts = stmts.into_iter();

    let mut seen = std::collections::HashSet::new();
    let mut statics = vec![];
    let mut stmts = vec![];
    while let Some(stmt) = istmts.next() {
        match stmt {
            syn::Stmt::Item(syn::Item::Static(var)) => {
                if var.mutability.is_some() {
                    if seen.contains(&var.ident) {
                        return Err(syn::parse::Error::new(
                            var.ident.span(),
                            format!("the name `{}` is defined multiple times", var.ident),
                        ));
                    }

                    seen.insert(var.ident.clone());
                    statics.push(var);
                } else {
                    stmts.push(syn::Stmt::Item(syn::Item::Static(var)));
                }
            }
            _ => {
                stmts.push(stmt);
                break;
            }
        }
    }

    stmts.extend(istmts);

    Ok((statics, stmts))
}

fn extract_cfgs(attrs: Vec<syn::Attribute>) -> (Vec<syn::Attribute>, Vec<syn::Attribute>) {
    let mut cfgs = vec![];
    let mut not_cfgs = vec![];

    for attr in attrs {
        if eq(&attr, "cfg") {
            cfgs.push(attr);
        } else {
            not_cfgs.push(attr);
        }
    }

    (cfgs, not_cfgs)
}

/// Returns `true` if `attr.path` matches `name`
fn eq(attr: &syn::Attribute, name: &str) -> bool {
    attr.style == syn::AttrStyle::Outer && attr.path.is_ident(name)
}
