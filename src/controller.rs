use std::fs::File;
use std::io::Write;
use std::net::{SocketAddr, UdpSocket};
use clap::{Parser, arg};
use String;
use crate::msg::ps::StatusType;

mod msg;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(long, default_value = "127.0.0.1")]
    ps_ip: String,

    #[arg(long, default_value = "8080")]
    ps_port: u16,

    #[arg(long, default_value = "192.168.1.55")]
    ctrl_addr: String,

    #[arg(long, default_value = "8080")]
    ctrl_port: u16,
}

fn build_get_status_req(status_type: StatusType) -> Vec<u8> {
    let req = msg::ps::GetStatusReq::new(status_type);
    println!("{:#?}", req);
    let mut ret = vec![msg::MessageId::GetStatusReq as u8];
    ret.append(bincode::serialize(&req).unwrap().as_mut());
    ret
}

fn run_status(sock: &UdpSocket) {
    let encoded = build_get_status_req(msg::ps::StatusType::ADC);

    println!("{:?}", encoded);
    let len = sock.send(encoded.as_slice()).unwrap();
    println!("{:?} bytes sent", len);
    let mut buf = [0; 1024];
    let len = sock.recv(&mut buf).unwrap();

    // println!("{:?}", &buf[..len]);
    let resp: msg::ps::GetStatusResp = bincode::deserialize(&buf[1..len]).unwrap();
    println!("{:?}", resp);

}

fn run_get_adc_value(sock: &UdpSocket, converted: bool) {
    let req = msg::ps::GetAdcValueReq::new(converted, 0);
    let mut encoded = vec![msg::MessageId::GetAdcValueReq as u8];
    encoded.append(bincode::serialize(&req).unwrap().as_mut());

    println!("{:?}", encoded);
    let len = sock.send(encoded.as_slice()).unwrap();
    println!("{:?} bytes sent", len);
    let mut buf = [0; 10];
    let len = sock.recv(&mut buf).unwrap();

    // println!("{:?}", &buf[..len]);
    let resp: msg::ps::GetAdcValueResp = bincode::deserialize(&buf[1..len]).unwrap();
    println!("{:?}", resp);
}

fn run_get_higrometer_status(sock: &UdpSocket) -> Result<u8, String> {
    let req = msg::ps::GetHygrometerStatusReq::new(0);
    let mut encoded = vec![msg::MessageId::GetHygrometerStatusReq as u8];
    encoded.append(bincode::serialize(&req).unwrap().as_mut());

    println!("{:?}", encoded);
    let len = sock.send(encoded.as_slice()).unwrap();
    println!("{:?} bytes sent", len);
    let mut buf = [0; 10];
    let len = sock.recv(&mut buf).unwrap();

    if len == 0 {
        return Err("No response received".to_string());
    }
    else if len == 1 {
        return Err("Error response received".to_string());
    }
    // println!("{:?}", &buf[..len]);
    let resp: msg::ps::GetHygrometerStatusResp = bincode::deserialize(&buf[1..len]).unwrap();
    println!("{:?}", resp);
    Ok(resp.humidity)
}

fn run_get_temperature(sock: &UdpSocket) {
    let req = msg::ps::GetTemperatureReq::new(0);
    let mut encoded = vec![msg::MessageId::GetTemperatureReq as u8];
    encoded.append(bincode::serialize(&req).unwrap().as_mut());

    println!("{:?}", encoded);
    let len = sock.send(encoded.as_slice()).unwrap();
    println!("{:?} bytes sent", len);
    let mut buf = [0; 10];
    let len = sock.recv(&mut buf).unwrap();

    if len == 0 {
        println!("No response received");
        return;
    }
    else if len == 1 {
        println!("Error response received");
        return;
    }
    // println!("{:?}", &buf[..len]);
    let resp: msg::ps::GetTemperatureResp = bincode::deserialize(&buf[1..len]).unwrap();
    println!("{:?}", resp);
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let ps_addr = SocketAddr::new(args.ps_ip.parse().unwrap(), args.ps_port);
    let ctrl_addr = SocketAddr::new(args.ctrl_addr.parse().unwrap(), args.ctrl_port);
    let sock = UdpSocket::bind(ctrl_addr).expect("Failed to bind UDP socket");
    sock.connect(ps_addr).expect("Failed to connect to UDP socket");
    println!("Controller created on {}", ps_addr);

    // run_get_adc_value(&sock, true);
    run_get_temperature(&sock);
    let _ = run_get_higrometer_status(&sock);

    // let mut file = File::create("/home/admin/RustroverProjects/PlantStation/controller.log").unwrap();
    // loop {
    //     let humidity = run_get_higrometer_status(&sock).unwrap_or(0);
    //     let timestamp = chrono::Utc::now().to_rfc3339();
    //     // write humidity to file with newline
    //     writeln!(file, "{}, {}",timestamp, humidity)?;
    //     file.flush()?;
    //     std::thread::sleep(std::time::Duration::from_secs(1));
    // }
    Ok(())
}
