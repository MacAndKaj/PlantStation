pub mod ps;

#[repr(u8)]
pub enum MessageId {
    Unknown = 0,
    GetStatusReq,
    GetStatusResp,
    GetAdcValueReq,
    GetAdcValueResp,
    GetHygrometerStatusReq,
    GetHygrometerStatusResp,
    GetTemperatureReq,
    GetTemperatureResp,
}
