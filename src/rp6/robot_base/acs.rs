use super::{port::*, Pin, RobotBase};

impl RobotBase {
    /// Disable the ACS of the robot.
    pub fn set_acs_power_off() {
        ACS_Pwr::set_input();
        ACS_Pwr::set_low();
        ACS_PwrH::set_input();
        ACS_PwrH::set_low();
        ACS_L::set_low();
        ACS_R::set_low();
    }

    /// Set the ACS of the robot to low power.
    pub fn set_acs_power_low() {
        ACS_Pwr::set_output();
        ACS_Pwr::set_high();
        ACS_PwrH::set_input();
        ACS_PwrH::set_low();
    }

    /// Set the ACS of the robot to medium power.
    pub fn set_acs_power_medium() {
        ACS_Pwr::set_input();
        ACS_Pwr::set_low();
        ACS_PwrH::set_output();
        ACS_PwrH::set_high();
    }

    /// Set the ACS of the robot to high power.
    pub fn set_acs_power_high() {
        ACS_Pwr::set_output();
        ACS_Pwr::set_high();
        ACS_PwrH::set_output();
        ACS_PwrH::set_high();
    }
}
