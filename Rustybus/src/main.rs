use std::env;
use std::net::IpAddr;
use std::str::FromStr;
use tokio::runtime::Runtime;
use tokio_modbus::prelude::*;
use tokio_modbus::tcp::TcpMaster;

const DEFAULT_PORT: u16 = 502;

fn main() {
    // Parse the target IP address from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run -- <IP_ADDRESS> <UNIT_ID>");
        return;
    }
    let target_ip = match IpAddr::from_str(&args[1]) {
        Ok(ip) => ip,
        Err(_) => {
            println!("Invalid IP address");
            return;
        }
    };
    let unit_id = match u8::from_str(&args[2]) {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid unit ID");
            return;
        }
    };

    // Create a Tokio runtime
    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    // Write values to Modbus registers
    rt.block_on(async {
        let addr = SocketAddr::from((target_ip, DEFAULT_PORT));
        let mut ctx = TcpMaster::connect(addr).await.expect("Failed to connect");

        let address = 0; // Address of the register to write to
        let value: u16 = 1234; // Value to write

        match ctx.write_single_register(unit_id, address, value).await {
            Ok(_) => {
                println!("Value written successfully! IP: {}, Unit ID: {}, Address: {}, Value: {}", target_ip, unit_id, address, value);
                // Additional actions can be performed here if needed
            }
            Err(err) => {
                println!("Failed to write value: {:?}", err);
            }
        }

        println!("Write operation complete.");
    });
}
