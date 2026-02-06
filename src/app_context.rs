use json;
use std::fs;
use std::collections::HashMap;
use json::JsonValue;

#[derive(Clone, Debug, PartialEq)]
pub enum AdcSupported {
    Unknown,
    ADS1115,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ThermometerSupported {
    Unknown,
    LPS331AP,
}

#[derive(Clone)]
pub struct DeviceData {
    pub address: u16,
    pub registers: HashMap<String, u8>,
    pub registers_values: HashMap<String, Vec<u8>>,
}

#[derive(Clone)]
pub struct AdcConfig {
    pub adc_address: u16,
    pub adc_type: AdcSupported,
    pub registers: HashMap<String, u8>,
    pub registers_values: HashMap<String, Vec<u8>>,
}

#[derive(Clone)]
pub struct ThermometerConfig {
    pub thermometer_type: ThermometerSupported,
    pub device_data: DeviceData,
}

#[derive(Clone)]
pub struct AppContext {
    pub i2c_dev_path: String,
    pub adc_config: AdcConfig,
    pub thermometer_config: ThermometerConfig,
}

fn get_adc_type(type_str: &str) -> AdcSupported {
    match type_str.to_uppercase().as_str() {
        "ADS1115" => AdcSupported::ADS1115,
        _ => AdcSupported::Unknown,
    }
}

fn get_thermometer_type(type_str: &str) -> ThermometerSupported {
    match type_str.to_uppercase().as_str() {
        "LPS331AP" => ThermometerSupported::LPS331AP,
        _ => ThermometerSupported::Unknown,
    }
}

fn get_registers(reg_values: &JsonValue) -> HashMap<String, u8> {
    let mut ret = HashMap::new();
    for (key, value) in  reg_values.entries(){
        ret.insert(key.to_string(), value.as_u8().unwrap());
    }
    println!("Registers: {:?}", ret);
    ret
}

fn get_registers_values(reg_values: &JsonValue) -> HashMap<String, Vec<u8>> {
    let mut ret = HashMap::new();
    for (key, value) in  reg_values.entries(){
        let mut bytes_vec: Vec<u8> = Vec::new();
        for byte in value.members() {
            bytes_vec.push(byte.as_u8().unwrap());
        }
        ret.insert(key.to_string(), bytes_vec);
    }
    println!("Registers values: {:?}", ret);
    ret
}

impl AppContext {
    pub fn new(config_path: String) -> AppContext {
        let file: String = fs::read_to_string(&config_path).unwrap();
        let parsed = json::parse(&file).unwrap();
        AppContext {
            i2c_dev_path: parsed["i2cdev"].as_str().unwrap().to_string(),
            adc_config: AdcConfig {
                adc_address: parsed["adc"]["i2c_address"].as_u16().unwrap(),
                adc_type: get_adc_type(parsed["adc"]["type"].as_str().unwrap()),
                registers: get_registers(&parsed["adc"]["registers"]),
                registers_values: get_registers_values(&parsed["adc"]["registers_values"]),
            },
            thermometer_config: ThermometerConfig {
                thermometer_type: get_thermometer_type(parsed["thermometer"]["type"].as_str().unwrap()),
                device_data: DeviceData {
                    address: parsed["thermometer"]["i2c_address"].as_u16().unwrap(),
                    registers : get_registers(&parsed["thermometer"]["registers"]),
                    registers_values : get_registers_values(&parsed["thermometer"]["registers_values"]),
                }
            },
        }
    }
}
