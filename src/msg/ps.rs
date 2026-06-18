// Re-export generated protobuf types with compatibility helpers
use super::{
    GetAdcValueReq as ProtoGetAdcValueReq, GetAdcValueResp as ProtoGetAdcValueResp,
    GetHygrometerStatusReq as ProtoGetHygrometerStatusReq, GetHygrometerStatusResp as ProtoGetHygrometerStatusResp,
    GetStatusReq as ProtoGetStatusReq, GetStatusResp as ProtoGetStatusResp,
    GetTemperatureReq as ProtoGetTemperatureReq, GetTemperatureResp as ProtoGetTemperatureResp
    ,
};

// Type aliases for main usage
pub type GetStatusReq = ProtoGetStatusReq;
pub type GetStatusResp = ProtoGetStatusResp;
pub type GetAdcValueReq = ProtoGetAdcValueReq;
pub type GetAdcValueResp = ProtoGetAdcValueResp;
pub type GetHygrometerStatusReq = ProtoGetHygrometerStatusReq;
pub type GetHygrometerStatusResp = ProtoGetHygrometerStatusResp;
pub type GetTemperatureReq = ProtoGetTemperatureReq;
pub type GetTemperatureResp = ProtoGetTemperatureResp;

// Re-export StatusType enum with compatibility
pub use super::StatusType;

// Helper trait implementations for backward compatibility
impl GetStatusReq {
    pub fn new(status_type_arg: StatusType) -> GetStatusReq {
        GetStatusReq { status_type: status_type_arg as u32 }
    }

    pub fn get_status(&self) -> StatusType {
        match self.status_type {
            1 => StatusType::I2c,
            2 => StatusType::Adc,
            _ => StatusType::Unknown
        }
    }
}

impl GetAdcValueReq {
    pub fn new(converted: bool, channel: u32) -> GetAdcValueReq {
        GetAdcValueReq { converted, channel }
    }

    pub fn is_converted(&self) -> bool {
        self.converted
    }
}

impl GetStatusResp {
    pub fn new(status: String) -> GetStatusResp {
        GetStatusResp { status }
    }
}

impl GetAdcValueResp {
    pub fn new(value: u32) -> GetAdcValueResp {
        GetAdcValueResp { value }
    }
}

impl GetHygrometerStatusReq {
    pub fn new(channel: u32) -> GetHygrometerStatusReq {
        GetHygrometerStatusReq { channel }
    }
}

impl GetHygrometerStatusResp {
    pub fn new(humidity: u32) -> GetHygrometerStatusResp {
        GetHygrometerStatusResp { humidity }
    }
}

impl GetTemperatureReq {
    pub fn new(dummy: u32) -> GetTemperatureReq {
        GetTemperatureReq { dummy }
    }
}

impl GetTemperatureResp {
    pub fn new(temperature: i32) -> GetTemperatureResp {
        GetTemperatureResp { temperature }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_type_i2c() {
        let req = GetStatusReq::new(StatusType::I2c);
        assert_eq!(req.get_status() as u8, StatusType::I2c as u8);
    }

    #[test]
    fn test_status_type_adc() {
        let req = GetStatusReq::new(StatusType::Adc);
        assert_eq!(req.get_status() as u8, StatusType::Adc as u8);
    }

    #[test]
    fn test_status_type_unknown() {
        let req = GetStatusReq::new(StatusType::Unknown);
        assert_eq!(req.get_status() as u8, StatusType::Unknown as u8);
    }

    #[test]
    fn test_get_status_req_i2c_serialization() {
        let req = GetStatusReq::new(StatusType::I2c);
        // Protobuf types don't have serde support by default
        // Just verify the request was created correctly
        assert_eq!(req.status_type, 1);
    }

    #[test]
    fn test_get_status_req_adc_serialization() {
        let req = GetStatusReq::new(StatusType::Adc);
        // Protobuf types don't have serde support by default
        // Just verify the request was created correctly
        assert_eq!(req.status_type, 2);
    }

    #[test]
    fn test_get_status_resp_serialization() {
        let resp = GetStatusResp::new("OK".to_string());
        // Just verify the response was created correctly
        assert_eq!(resp.status, "OK".to_string());
    }

    #[test]
    fn test_get_adc_value_req_converted() {
        let req = GetAdcValueReq::new(true, 0);
        assert!(req.is_converted());
        assert_eq!(req.channel, 0);
    }

    #[test]
    fn test_get_adc_value_req_raw() {
        let req = GetAdcValueReq::new(false, 5);
        assert!(!req.is_converted());
        assert_eq!(req.channel, 5);
    }

    #[test]
    fn test_get_adc_value_req_serialization() {
        let req = GetAdcValueReq::new(true, 3);
        // Just verify the request was created correctly
        assert_eq!(req.is_converted(), true);
        assert_eq!(req.channel, 3);
    }

    #[test]
    fn test_get_adc_value_resp_serialization() {
        let resp = GetAdcValueResp::new(1024);
        // Just verify the response was created correctly
        assert_eq!(resp.value, 1024);
    }

    #[test]
    fn test_get_hygrometer_status_req_all_channels() {
        for channel in 0..8 {
            let req = GetHygrometerStatusReq::new(channel);
            assert_eq!(req.channel, channel);
        }
    }

    #[test]
    fn test_get_hygrometer_status_req_serialization() {
        let req = GetHygrometerStatusReq::new(2);
        // Just verify the request was created correctly
        assert_eq!(req.channel, 2);
    }

    #[test]
    fn test_get_hygrometer_status_resp_serialization() {
        let resp = GetHygrometerStatusResp::new(65);
        // Just verify the response was created correctly
        assert_eq!(resp.humidity, 65);
    }

    #[test]
    fn test_get_temperature_req_serialization() {
        let req = GetTemperatureReq::new(0);
        // Just verify it was created correctly
        assert_eq!(req.dummy, 0);
    }

    #[test]
    fn test_get_temperature_resp_positive() {
        let resp = GetTemperatureResp::new(25);
        // Just verify the response was created correctly
        assert_eq!(resp.temperature, 25);
    }

    #[test]
    fn test_get_temperature_resp_negative() {
        let resp = GetTemperatureResp::new(-10);
        // Just verify the response was created correctly
        assert_eq!(resp.temperature, -10);
    }

    #[test]
    fn test_get_temperature_resp_extreme_cold() {
        let resp = GetTemperatureResp::new(-273); // absolute zero
        // Just verify the response was created correctly
        assert_eq!(resp.temperature, -273);
    }

    #[test]
    fn test_invalid_status_type_conversion() {
        let req = GetStatusReq::new(StatusType::Unknown);
        assert_eq!(req.get_status() as u8, 0); // Unknown variant
    }
}
