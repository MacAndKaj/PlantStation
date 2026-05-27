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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pga_to_fsr_6_144v() {
        assert_eq!(pga_to_fsr(0b000), 6.144);
    }

    #[test]
    fn test_pga_to_fsr_4_096v() {
        assert_eq!(pga_to_fsr(0b001), 4.096);
    }

    #[test]
    fn test_pga_to_fsr_2_048v() {
        assert_eq!(pga_to_fsr(0b010), 2.048);
    }

    #[test]
    fn test_pga_to_fsr_1_024v() {
        assert_eq!(pga_to_fsr(0b011), 1.024);
    }

    #[test]
    fn test_pga_to_fsr_0_512v() {
        assert_eq!(pga_to_fsr(0b100), 0.512);
    }

    #[test]
    fn test_pga_to_fsr_0_256v() {
        assert_eq!(pga_to_fsr(0b101), 0.256);
    }

    #[test]
    fn test_pga_to_fsr_invalid_defaults_to_2_048() {
        assert_eq!(pga_to_fsr(0b110), 2.048);
        assert_eq!(pga_to_fsr(0b111), 2.048);
        assert_eq!(pga_to_fsr(255), 2.048);
    }

    #[test]
    fn test_config_for_mux_channel_0() {
        let base_config = vec![0b10001111, 0x83]; // Example config
        let result = config_for_mux(0, &base_config);
        assert_eq!(result[0], 0b10001111); // Channel 0 << 4 = 0b0000_0000
    }

    #[test]
    fn test_config_for_mux_channel_1() {
        let base_config = vec![0b10001111, 0x83];
        let result = config_for_mux(1, &base_config);
        assert_eq!(result[0], 0b10011111); // Channel 1 << 4 = 0b0001_0000
    }

    #[test]
    fn test_config_for_mux_channel_7() {
        let base_config = vec![0b10001111, 0x83];
        let result = config_for_mux(7, &base_config);
        assert_eq!(result[0], 0b11111111); // Channel 7 << 4 = 0b0111_0000
    }

    #[test]
    fn test_config_for_mux_preserves_other_bits() {
        let base_config = vec![0b10001111, 0x83];
        let result = config_for_mux(5, &base_config);
        // Original: 0b10001111, with channel 5 = 0b10101111
        // Mask: 0b10001111 (keep bits 0-3 and 7)
        // Result: (0b10001111 & 0b10001111) | (5 << 4) = 0b10101111
        assert_eq!(result[0] & 0b00001111, 0b1111); // lower 4 bits preserved
        assert_eq!(result[0] & 0b10000000, 0b10000000); // bit 7 preserved
    }

    #[test]
    fn test_config_for_mux_preserves_second_byte() {
        let base_config = vec![0b10001111, 0xAB];
        let result = config_for_mux(3, &base_config);
        assert_eq!(result[1], 0xAB); // Second byte unchanged
    }

    #[test]
    fn test_raw_to_voltage_zero() {
        let init_config = std::collections::HashMap::new();
        let mut test_config = init_config.clone();
        test_config.insert("Config".to_string(), vec![0x83, 0x83]);

        let ads = Ads1115::new(0x48, test_config);
        let voltage = ads.raw_to_voltage(0);
        assert_eq!(voltage, 0);
    }

    #[test]
    fn test_raw_to_voltage_max_positive() {
        let init_config = std::collections::HashMap::new();
        let mut test_config = init_config.clone();
        test_config.insert("Config".to_string(), vec![0x83, 0x83]);

        let ads = Ads1115::new(0x48, test_config);
        let voltage = ads.raw_to_voltage(32767); // Max positive value
        // With PGA=1 (4.096V FSR), expected ~4096mV
        assert!(voltage > 4000 && voltage <= 4096);
    }

    #[test]
    fn test_raw_to_voltage_half_scale() {
        let init_config = std::collections::HashMap::new();
        let mut test_config = init_config.clone();
        test_config.insert("Config".to_string(), vec![0x83, 0x83]);

        let ads = Ads1115::new(0x48, test_config);
        let voltage = ads.raw_to_voltage(16384); // Half-scale
        // Expected ~2048mV
        assert!(voltage > 1900 && voltage < 2100);
    }
}
