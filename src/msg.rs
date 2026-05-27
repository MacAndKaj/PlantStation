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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_id_values() {
        assert_eq!(MessageId::Unknown as u8, 0);
        assert_eq!(MessageId::GetStatusReq as u8, 1);
        assert_eq!(MessageId::GetStatusResp as u8, 2);
        assert_eq!(MessageId::GetAdcValueReq as u8, 3);
        assert_eq!(MessageId::GetAdcValueResp as u8, 4);
        assert_eq!(MessageId::GetHygrometerStatusReq as u8, 5);
        assert_eq!(MessageId::GetHygrometerStatusResp as u8, 6);
        assert_eq!(MessageId::GetTemperatureReq as u8, 7);
        assert_eq!(MessageId::GetTemperatureResp as u8, 8);
    }

    #[test]
    fn test_message_id_casting() {
        let id = MessageId::GetStatusReq as u8;
        assert_eq!(id, 1);

        let resp_id = MessageId::GetStatusResp as u8;
        assert_eq!(resp_id, 2);
    }
}
