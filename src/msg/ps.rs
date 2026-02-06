use serde::{Serialize, Deserialize};
use derive_new::new;

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
pub enum StatusType {
    Unknown,
    I2C,
    ADC
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetStatusReq {
    status_type: u8,
}

impl GetStatusReq {
    pub fn new(status_type_arg: StatusType) -> GetStatusReq {
        GetStatusReq { status_type: status_type_arg as u8 }
    }

    pub fn get_status(&self) -> StatusType {
        match self.status_type {
            1 => StatusType::I2C,
            2 => StatusType::ADC,
            _ => StatusType::Unknown
        }
    }
}

#[derive(Serialize, Deserialize, Debug, new)]
pub struct GetStatusResp {
    pub status: String
}


#[derive(Serialize, Deserialize, Debug, new)]
pub struct GetAdcValueReq {
    converted: bool,
    pub(crate) channel: u8, // mux bitmap 0=0b000, 1=0b001,...., 7=0b111
}
impl GetAdcValueReq {
    pub fn is_converted(&self) -> bool {
        self.converted
    }
}

#[derive(Serialize, Deserialize, Debug, new)]
pub struct GetAdcValueResp {
    pub value: u16,
}

#[derive(Serialize, Deserialize, Debug, new)]
pub struct GetHygrometerStatusReq {
    pub channel: u8, // mux bitmap 0=0b000, 1=0b001,...., 7=0b111
}

#[derive(Serialize, Deserialize, Debug, new)]
pub struct GetHygrometerStatusResp {
    pub humidity: u8,
}

#[derive(Serialize, Deserialize, Debug, new)]
pub struct GetTemperatureReq {
    dummy: u8,
}

#[derive(Serialize, Deserialize, Debug, new)]
pub struct GetTemperatureResp {
    pub temperature: i16,
}
