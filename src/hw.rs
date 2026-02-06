use crate::app_context::{AdcSupported, AppContext};
use crate::hw::i2c_mgmt::I2cDevice;
use String;

mod i2c_mgmt;
mod adc;
mod thermometer;

pub struct Hw {
    app_context: AppContext,
    i2c: I2cDevice,
    adc: Box<dyn adc::Adc>,
    thermometer: Box<dyn thermometer::Thermometer>,
}

impl Hw {
    pub fn new(context: AppContext) -> Hw {
        if context.adc_config.adc_type == AdcSupported::Unknown {
            panic!("Unsupported ADC type in configuration");
        }
        Hw {
            app_context: context.clone(),
            i2c: I2cDevice::new(context.i2c_dev_path),
            adc: Box::new(adc::Ads1115::new( // todo: support other ADCs
                context.adc_config.adc_address,
                context.adc_config.registers_values.clone(),
            )),
            thermometer: Box::new(thermometer::Lps331ap::new( // todo: support other thermometers
                context.thermometer_config.device_data.address,
                context.thermometer_config.device_data.registers_values.clone(),
            )),
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        println!("Initializing hardware components");
        match self.thermometer.initialize(&mut self.i2c) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Thermometer initialization failed: {}", e)),
        }
    }
    
    pub fn i2c_status(&self) -> String {
        match self.i2c.functionality() {
            Ok(functionality) => format!("{} | {}", self.i2c.dev_path(), functionality),
            Err(error) => {
                error.to_string()
            }
        }
    }

    pub fn adc_status(&mut self) -> String {
        let adc_conf = &self.app_context.adc_config;
        const REG_LEN: usize = 2;

        if !adc_conf.registers.contains_key("Config") {
            println!("Config register not found");
            return String::from("Error: ADC Config register not defined");
        }

        println!("Retrieving ADC status from address {:x}", adc_conf.adc_address);

        match self
            .i2c
            .get_register(adc_conf.adc_address, adc_conf.registers["Config"], REG_LEN)
        {
            Ok(status_reg) => {
                let mut status_hex = String::from("0x");
                for byte in status_reg.iter() {
                    status_hex.push_str(&format!("{:X}", byte));
                }
                println!("ADC status: {:}", status_hex);
                status_hex
            },
            Err(error) => {
                println!("ADC status read error: {}", error);
                error.to_string()
            }
        }
    }
    
    pub fn read_adc_value(&mut self, converted: bool, channel: u8) -> Result<u16, String> {
        let raw_bytes_result = self.adc.read_val(&mut self.i2c, channel);
        match raw_bytes_result {
            Ok(raw_bytes) => {
                let raw = u16::from_be_bytes([raw_bytes[0], raw_bytes[1]]);
                if converted {
                    return Ok(self.adc.raw_to_voltage(raw));
                }
                Ok(raw)
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub fn read_humidity(&mut self, channel: u8) -> Result<u8, String> {
        let voltage = self.read_adc_value(true, channel);
        match voltage {
            Ok(voltage_mv) => {
                // Placeholder conversion logic
                let v = voltage_mv as f32;
                let humidity = ((3300.0 - v) / 3300.0 * 100.0) as u8;
                Ok(humidity)
            }
            Err(err) => Err(err),
        }
    }
    
    pub fn read_temperature(&mut self) -> Result<i16, String> {
        self.thermometer.read_temperature(&mut self.i2c)
    }
}
