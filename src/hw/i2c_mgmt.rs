extern crate i2c_linux;

use i2c_linux::I2c;
use std::fs::File;

pub struct I2cDevice {
    dev_path: String,
    dev: I2c<File>,
}

impl I2cDevice {
    pub fn new(device_path: String) -> I2cDevice {
        I2cDevice {
            dev_path: device_path.clone(),
            dev: I2c::from_path(device_path).expect("Failed to create i2c device"),
        }
    }

    pub fn dev_path(&self) -> &String {
        &self.dev_path
    }

    pub fn functionality(&self) -> Result<String, String> {
        let ret = self.dev.i2c_functionality();
        println!("I2C functionality: {:?}", ret);
        match ret {
            Ok(ret_func) => Ok(format!("{:?}", ret_func).to_string()),
            Err(error) => Err("I2C functionality error".to_string() + &error.to_string()),
        }
    }

    pub fn write_register(
        &mut self,
        slave_address: u16,
        register: u8,
        value: &Vec<u8>,
    ) -> Result<(), String> {
        self.dev
            .smbus_set_slave_address(slave_address, false)
            .expect("Error during setting slave address");

        println!(
            "Writing value {:?} to register {} at slave address {:x}",
            value, register, slave_address
        );
        match self.dev.i2c_write_block_data(register, value.as_slice()) {
            Ok(_) => Ok(()),
            Err(_) => {
                println!("Write failed for register {}", register);
                Err("Write failed for ".to_string() + &register.to_string())
            }
        }
    }

    pub fn get_register(
        &mut self,
        slave_address: u16,
        register: u8,
        num_of_bytes: usize,
    ) -> Result<Vec<u8>, String> {
        self.dev
            .smbus_set_slave_address(slave_address, false)
            .expect("Error during setting slave address");
        let mut buffer: [u8; 4] = [0; 4];

        println!(
            "Reading {} bytes from register {} at slave address {:x}",
            num_of_bytes, register, slave_address
        );
        match self.dev.i2c_read_block_data(register, &mut buffer) {
            Ok(size) => {
                if size < num_of_bytes {
                    println!("Read {} bytes, expected {}", size, num_of_bytes);
                    return Err(
                        "Read less bytes than expected for ".to_string() + &register.to_string()
                    );
                }
                Ok(buffer[0..num_of_bytes].to_vec())
            }
            Err(e) => {
                println!("Read failed for register {}, with I2C error: {}", register, e);
                Err("Read failed for ".to_string() + &register.to_string())
            }
        }
    }

    pub fn get_byte_from_register(
        &mut self,
        slave_address: u16,
        register: u8,
    ) -> Result<u8, String> {
        self.dev
            .smbus_set_slave_address(slave_address, false)
            .expect("Error during setting slave address");

        println!("Reading 1 byte from register {} at slave address {:x}", register, slave_address);
        match self.dev.smbus_read_byte_data(register) {
            Ok(byte) => Ok(byte),
            Err(e) => {
                println!("Read failed for register {}, with I2C error: {}", register, e);
                Err("Read failed for ".to_string() + &register.to_string())
            }
        }
    }
}
