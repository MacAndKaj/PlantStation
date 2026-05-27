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
