// This example shows how to connect to the hub and activate the motor
//
//
// 

use std::{
    str::FromStr, 
    time::Duration
};

use tokio::time;

use anyhow::Result;
use btleplug::api::BDAddr;

use rust_powered_lego::{
    connection_manager::ConnectionManager, 
    HubType,
    MotorType,
    lego::message_parameters::{
        StartupAndCompletionInfo,
    },
    lego::{
        consts::{
            TechnicHubPorts,
            EndState, 
            Profile,
        },
    },
};


#[tokio::main]
async fn main() -> Result<()> {
    // Hub "MAC" address can be found in several ways. 
    // Connect it to a computer and continue from there...
    let hub_mac_address = "90:84:2b:4e:5b:96";
    let port_id = TechnicHubPorts::B;

    // Converting the MAC string to btleplug::api::BDAddr type
    let address = BDAddr::from_str(hub_mac_address)?;

    // The ConnectionManager connects stuff - so ask it for the hub...
    let cm = ConnectionManager::new();

    // It is possible to use the name of the hub or its MAC address. That's why it's Option<>
    // Here, only address is implemented
    let hub = cm.get_hub(None, Some(address), 5).await?;

    // Ask to get the motor object (pay attention to the port_id)
    let motor = hub.get_motor(port_id as u8).await?;

    // Initiate the motor with power
    _ = motor.start_power(100, StartupAndCompletionInfo::ExecuteImmediatelyAndNoAction).await?;

    // Let it hang there for 3 seconds
    time::sleep(Duration::from_secs(3)).await;

    // And stop
    _ = motor.stop_motor(EndState::FLOAT, Profile::AccDec, StartupAndCompletionInfo::ExecuteImmediatelyAndNoAction).await?;

    Ok(())
}