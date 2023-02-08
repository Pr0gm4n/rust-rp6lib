window.SIDEBAR_ITEMS = {"fn":[["without_interrupts","Executes a closure, disabling interrupts until its completion. Introduces a `CriticalSection` that allows to access shared data structures via the guards provided in the `mutex` module."]],"mod":[["mutex","Module to allow safe access of shared data structures within a `CriticalSection`."]],"static":[["CRITICAL_SECTION_COUNTER","Atomic counter of critical sections to avoid problems when `without_interrupts` is used in nested function calls."]],"struct":[["CriticalSection","Helper struct that automatically restores interrupts on drop. The wrapped `PhantomData` creates a private field to ensure that this struct cannot be initialized from outside of this module without using its `unsafe` initializer function `new`. The recommended use to enter a `CriticalSection` is to pass a closure to `without_interrupts`."]]};