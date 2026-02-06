mod app_context;
mod hw;
mod msg;

use std::net::{SocketAddr, UdpSocket};
use log::{error, info};
use crate::msg::MessageId;
use String;
use clap::{Parser, arg};
use msg::ps::StatusType;
use crate::app_context::AppContext;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "127.0.0.1")]
    ip: String,

    #[arg(short, long, default_value = "8080")]
    port: u16,

    #[arg(long)]
    hw_config: String,
}

fn route(msg_id: u8, buffer: &[u8], ps_hw: &mut hw::Hw) -> Vec<u8> {
    info!("Routing message id {}", msg_id);
    const GET_STATUS_MSG_ID: u8 = MessageId::GetStatusReq as u8;
    const GET_ADC_VALUE_MSG_ID: u8 = MessageId::GetAdcValueReq as u8;
    const GET_HYGROMETER_STATUS_MSG_ID: u8 = MessageId::GetHygrometerStatusReq as u8;
    const GET_TEMPERATURE_MSG_ID: u8 = MessageId::GetTemperatureReq as u8;

    match msg_id {
        GET_STATUS_MSG_ID => {
            match bincode::deserialize::<msg::ps::GetStatusReq>(buffer) {
                Ok(msg) => handle_get_status_req(&msg, ps_hw),
                Err(e) => {
                    error!("{}", e);
                    Vec::new()
                }
            }
        },
        GET_ADC_VALUE_MSG_ID => {
            match bincode::deserialize::<msg::ps::GetAdcValueReq>(buffer) {
                Ok(msg) => handle_get_adc_value_req(&msg, ps_hw),
                Err(e) => {
                    error!("{}", e);
                    Vec::new()
                }
            }
        },
        GET_HYGROMETER_STATUS_MSG_ID => {
            match bincode::deserialize::<msg::ps::GetHygrometerStatusReq>(buffer) {
                Ok(msg) => handle_get_higrometer_status_req(&msg, ps_hw),
                Err(e) => {
                    error!("GetHygrometerStatusReq error: {}", e);
                    Vec::new()
                }
            }
        },
        GET_TEMPERATURE_MSG_ID => {
            match bincode::deserialize::<msg::ps::GetTemperatureReq>(buffer) {
                Ok(_) => handle_get_temperature_req(ps_hw),
                Err(e) => {
                    error!("GetTemperatureReq error: {}", e);
                    Vec::new()
                }
            }
        },
        _ => {
            info!("Received unknown opcode {}", msg_id);
            Vec::new()
        }
    }
}

fn handle_get_status_req(req: &msg::ps::GetStatusReq, plantstation_hw: &mut hw::Hw) -> Vec<u8> {
    info!("Handling GetStatusReq: {:?}", req);
    let mut resp = msg::ps::GetStatusResp::new(String::new());
    match req.get_status() {
        StatusType::I2C => resp.status = plantstation_hw.i2c_status(),
        StatusType::ADC => resp.status = plantstation_hw.adc_status(),
        StatusType::Unknown => resp.status = String::from("Unknown"),
    }

    let mut out = bincode::serialize(&resp).unwrap();
    out.insert(0, MessageId::GetStatusResp as u8);
    out
}

fn handle_get_adc_value_req(req: &msg::ps::GetAdcValueReq, plantstation_hw: &mut hw::Hw) -> Vec<u8> {
    info!("Handling GetAdcValueReq: {:?}", req);
    let mut resp = msg::ps::GetAdcValueResp::new(0);
    match plantstation_hw.read_adc_value(req.is_converted(), req.channel) {
        Ok(val) => {
            resp.value = val;
        }
        Err(e) => {
            error!("Error reading ADC value: {}", e);
        }
    }

    let mut out = bincode::serialize(&resp).unwrap();
    out.insert(0, MessageId::GetAdcValueResp as u8);
    out
}

fn handle_get_higrometer_status_req(req: &msg::ps::GetHygrometerStatusReq, plantstation_hw: &mut hw::Hw) -> Vec<u8> {
    info!("Handling GetHygrometerStatusReq: {:?}", req);
    let mut resp = msg::ps::GetHygrometerStatusResp::new(0);

    resp.humidity = plantstation_hw.read_humidity(req.channel).unwrap_or_else(|_| 0);

    let mut out = bincode::serialize(&resp).unwrap_or_else(|_| {
        error!("Error serializing GetHygrometerStatusResp");
        Vec::from([0])
    });
    out.insert(0, MessageId::GetHygrometerStatusResp as u8);
    out
}

fn handle_get_temperature_req(plantstation_hw: &mut hw::Hw) -> Vec<u8> {
    info!("Handling GetTemperatureReq");
    let mut resp = msg::ps::GetTemperatureResp::new(-273);
    resp.temperature = plantstation_hw.read_temperature().unwrap_or(-273);

    let mut out = bincode::serialize(&resp).unwrap_or_else(|_| {
        info!("Error serializing GetTemperatureResp");
        Vec::from([0])
    });
    out.insert(0, MessageId::GetTemperatureResp as u8);
    out
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut hw = hw::Hw::new(AppContext::new(args.hw_config));
    hw.initialize().expect("HW initialization failed");

    let addr = SocketAddr::new(args.ip.parse().unwrap(), args.port);
    let sock = UdpSocket::bind(addr).expect("Failed to bind UDP socket");
    let mut buf = [0; 1024];

    info!("Listening on {}", addr);
    loop {
        let (len, src_addr) = sock.recv_from(&mut buf)?;
        info!("{:?} bytes received from {:?}", len, src_addr);
        info!("{:?}", &buf[..len]);

        let resp = route(buf[0], &buf[1..len], &mut hw);
        sock.send_to(resp.as_slice(), src_addr)?;
    }
}
