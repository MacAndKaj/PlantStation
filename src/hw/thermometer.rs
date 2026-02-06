use std::collections::HashMap;
use crate::hw::i2c_mgmt::I2cDevice;

pub trait Thermometer {
    fn initialize(&self, i2c: &mut I2cDevice) -> Result<(), String>;
    fn read_temperature(&self, i2c: &mut I2cDevice) -> Result<i16, String>;
}

pub struct Lps331ap {
    address: u16,
    ctrl_reg1_value: u8,
}

impl Lps331ap {
    const WHO_AM_I: u8 = 0x0F;
    const WHO_AM_I_VALUE: u8 = 0xBB;
    const CTRL_REG1: u8 = 0x20;
    const TEMP_OUT_L: u8 = 0x2B;
    const TEMP_OUT_H: u8 = 0x2C;
    pub fn new(addr: u16, init_config: HashMap<String, Vec<u8>>) -> Self {
        let ctrl_reg1 = init_config.get("CtrlReg1").expect("CTRL_REG1 not found in init_config");
        if ctrl_reg1.len() != 1 {
            panic!("CTRL_REG1 value must be 1 byte for LPS331AP");
        }
        Lps331ap {
            address: addr,
            ctrl_reg1_value: *ctrl_reg1.first().unwrap(),
        }
    }
}

impl Thermometer for Lps331ap {
    fn initialize(&self, i2c: &mut I2cDevice) -> Result<(), String> {
        println!("Initializing LPS331AP thermometer");

        let who_am_i = match i2c.get_register(self.address, Self::WHO_AM_I, 1) {
            Ok(val) => val[0],
            Err(e) => return Err(format!("Failed to read WHO_AM_I from LPS331AP: {}", e)),
        };

        if who_am_i != Self::WHO_AM_I_VALUE {
            return Err(format!(
                "LPS331AP WHO_AM_I mismatch: expected {:X}, got {:X}",
                Self::WHO_AM_I_VALUE, who_am_i
            ));
        }

        match i2c.write_register(
            self.address,
            Self::CTRL_REG1,
            &Vec::from([self.ctrl_reg1_value])
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to write CTRL_REG1 to LPS331AP: {}", e)),
        }
    }

    fn read_temperature(&self, i2c: &mut I2cDevice) -> Result<i16, String> {
        println!("Reading temperature from LPS331AP");

        let temp_out_l = match i2c.get_register(self.address, Self::TEMP_OUT_L, 1) {
            Ok(val) => val[0],
            Err(e) => return Err(format!("Failed to read TEMP_OUT_L from LPS331AP: {}", e)),
        };
        let temp_out_h = match i2c.get_register(self.address, Self::TEMP_OUT_H, 1) {
            Ok(val) => val[0],
            Err(e) => return Err(format!("Failed to read TEMP_OUT_H from LPS331AP: {}", e)),
        };

        let raw_temp = ((temp_out_h as i16) << 8) | (temp_out_l as i16);
        let temperature_c = (raw_temp as f32 / 480.0) + 42.5;
        Ok(temperature_c.round() as i16)
    }
}