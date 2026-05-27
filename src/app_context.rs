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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_adc_type_ads1115() {
        let adc_type = get_adc_type("ADS1115");
        assert_eq!(adc_type, AdcSupported::ADS1115);
    }

    #[test]
    fn test_get_adc_type_lowercase() {
        let adc_type = get_adc_type("ads1115");
        assert_eq!(adc_type, AdcSupported::ADS1115);
    }

    #[test]
    fn test_get_adc_type_mixed_case() {
        let adc_type = get_adc_type("AdS1115");
        assert_eq!(adc_type, AdcSupported::ADS1115);
    }

    #[test]
    fn test_get_adc_type_unknown() {
        let adc_type = get_adc_type("UNKNOWN_ADC");
        assert_eq!(adc_type, AdcSupported::Unknown);
    }

    #[test]
    fn test_get_adc_type_empty() {
        let adc_type = get_adc_type("");
        assert_eq!(adc_type, AdcSupported::Unknown);
    }

    #[test]
    fn test_get_thermometer_type_lps331ap() {
        let therm_type = get_thermometer_type("LPS331AP");
        assert_eq!(therm_type, ThermometerSupported::LPS331AP);
    }

    #[test]
    fn test_get_thermometer_type_lowercase() {
        let therm_type = get_thermometer_type("lps331ap");
        assert_eq!(therm_type, ThermometerSupported::LPS331AP);
    }

    #[test]
    fn test_get_thermometer_type_mixed_case() {
        let therm_type = get_thermometer_type("Lps331Ap");
        assert_eq!(therm_type, ThermometerSupported::LPS331AP);
    }

    #[test]
    fn test_get_thermometer_type_unknown() {
        let therm_type = get_thermometer_type("UNKNOWN_SENSOR");
        assert_eq!(therm_type, ThermometerSupported::Unknown);
    }

    #[test]
    fn test_get_thermometer_type_empty() {
        let therm_type = get_thermometer_type("");
        assert_eq!(therm_type, ThermometerSupported::Unknown);
    }

    #[test]
    fn test_app_context_loading_from_file() {
        let config_path = "tests/fixtures/test_config.json".to_string();
        let context = AppContext::new(config_path);

        // Verify I2C device path
        assert_eq!(context.i2c_dev_path, "/dev/i2c-1");

        // Verify ADC configuration
        assert_eq!(context.adc_config.adc_address, 72);
        assert_eq!(context.adc_config.adc_type, AdcSupported::ADS1115);
        assert!(context.adc_config.registers.contains_key("Conversion"));
        assert!(context.adc_config.registers.contains_key("Config"));
        assert_eq!(context.adc_config.registers["Conversion"], 0);
        assert_eq!(context.adc_config.registers["Config"], 1);

        // Verify thermometer configuration
        assert_eq!(context.thermometer_config.thermometer_type, ThermometerSupported::LPS331AP);
        assert_eq!(context.thermometer_config.device_data.address, 93);
        assert!(context.thermometer_config.device_data.registers.contains_key("CtrlReg1"));
        assert!(context.thermometer_config.device_data.registers.contains_key("WhoAmI"));
    }

    #[test]
    fn test_app_context_adc_registers_values() {
        let config_path = "tests/fixtures/test_config.json".to_string();
        let context = AppContext::new(config_path);

        assert!(context.adc_config.registers_values.contains_key("Config"));
        let config_values = &context.adc_config.registers_values["Config"];
        assert_eq!(config_values.len(), 2);
        assert_eq!(config_values[0], 131);
        assert_eq!(config_values[1], 131);
    }

    #[test]
    fn test_app_context_thermometer_registers_values() {
        let config_path = "tests/fixtures/test_config.json".to_string();
        let context = AppContext::new(config_path);

        assert!(context.thermometer_config.device_data.registers_values.contains_key("CtrlReg1"));
        let ctrl_values = &context.thermometer_config.device_data.registers_values["CtrlReg1"];
        assert_eq!(ctrl_values.len(), 1);
        assert_eq!(ctrl_values[0], 224);
    }
}
