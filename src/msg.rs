pub mod ps;

// Include generated protobuf code
include!(concat!(env!("OUT_DIR"), "/plantstation.msg.rs"));

#[repr(u8)]
pub enum MessageId {
    Unknown = 0,
    GetStatusReq = 1,
    GetStatusResp = 2,
    GetAdcValueReq = 3,
    GetAdcValueResp = 4,
    GetHygrometerStatusReq = 5,
    GetHygrometerStatusResp = 6,
    GetTemperatureReq = 7,
    GetTemperatureResp = 8,
}


