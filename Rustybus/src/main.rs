use td::error::Error;
use std::time::Duration;

use tokio::time::sleep;
use tokio_modbus::prelude::*;
use tokio_modbus::rtu::SerialPort;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the Modbus PLC via serial port
    let mut ctx = ModbusContext::new_rtu(
        SerialPort::new("/dev/ttyUSB0", 19200, 'N', 8, 1)?,
        ModbusMode::RTU,
    );

    // Set the slave ID of the PLC you want to communicate with
    ctx.set_slave(1);

    // Read holding registers from the PLC
    let start_address = 0;
    let num_registers = 10;
    let response = ctx.read_holding_registers(start_address, num_registers).await?;

    // Print the received data
    for (i, value) in response.iter().enumerate() {
        println!("Register {}: {}", start_address + i as u16, value);
    }

    // Write a value to a holding register in the PLC
    let register_address = 100;
    let value_to_write = 42;
    ctx.write_single_register(register_address, value_to_write).await?;

    // Sleep for 1 second
    sleep(Duration::from_secs(1)).await;

    // Read the value from the previously written holding register
    let response = ctx.read_holding_registers(register_address, 1).await?;
    let read_value = response[0];
    println!("Register {}: {}", register_address, read_value);

    Ok(())
}s
