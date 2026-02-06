use std::collections::HashMap;
use crate::hw::i2c_mgmt::I2cDevice;

use bit_vec::BitVec;

pub trait Adc {
    fn read_val(&self, i2c: &mut I2cDevice, channel: u8) -> Result<Vec<u8>, String>;
    fn raw_to_voltage(&self, raw_val: u16) -> u16;
}

pub struct Ads1115 {
    address: u16,
    config_value: Vec<u8>,
}

fn pga_to_fsr(pga: u8) -> f32 {
    println!("PGA setting: {}", pga);
    match pga {
        0b000 => 6.144,
        0b001 => 4.096,
        0b010 => 2.048,
        0b011 => 1.024,
        0b100 => 0.512,
        0b101 => 0.256,
        _ => 2.048, // Default to ±2.048V
    }
}

fn config_for_mux(channel: u8, base_config: &Vec<u8>) -> Vec<u8> {
    let mut config = base_config.clone();
    config[0] = (config[0] & 0b10001111) | (channel << 4);
    config
}

impl Ads1115 {
    const CONFIG_REGISTER: u8 = 0x01;
    const CONVERSION_REGISTER: u8 = 0x00;
    pub fn new(addr: u16, init_config: HashMap<String, Vec<u8>>) -> Self {
        let config = init_config.get("Config").expect("Config register not found in init_config");
        if config.len() != 2 {
            panic!("Config value must be 2 bytes for ADS1115");
        }
        Ads1115 {
            address: addr,
            config_value: config.clone(),
        }
    }
}

impl Adc for Ads1115 {
    fn read_val(&self, i2c: &mut I2cDevice, channel: u8) -> Result<Vec<u8>, String> {
        if channel > 7 {
            return Err("Invalid channel for ADS1115".to_string());
        }
        println!("Reading from ADS1115");
        let config_value = config_for_mux(channel, &self.config_value);
        match i2c.write_register(self.address, Self::CONFIG_REGISTER, config_value.as_ref()) {
            Ok(_) => (),
            Err(e) => return Err(format!("Failed to write config to ADS1115: {}", e)),
        }
        i2c.get_register(self.address, Self::CONVERSION_REGISTER, 2)
    }

    fn raw_to_voltage(&self, raw_val: u16) -> u16 {
        let mut pga_bitset = BitVec::from_bytes(&[0x00]);
        pga_bitset.set(5, (self.config_value[0] & 0x08) != 0);
        pga_bitset.set(6,(self.config_value[0] & 0x04)  != 0);
        pga_bitset.set(7, (self.config_value[0] & 0x02)  != 0);
        let val_f = (raw_val as f32 / 32767.) * pga_to_fsr(pga_bitset.to_bytes()[0]);
        (val_f * 1000.) as u16 // Return in millivolts
    }
}