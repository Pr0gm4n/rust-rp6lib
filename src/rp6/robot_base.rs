/// Struct managing all actions regarding the robot's base
pub struct RobotBase {}

impl RobotBase {
    pub fn init() {}

    /// Set the LEDs on the `RobotBase` to the least significant 6 bits of the provided value
    pub fn set_leds(_value: u8) {
        // TODO: set value
        Self::update_leds();
    }

    fn update_leds() {}
}
