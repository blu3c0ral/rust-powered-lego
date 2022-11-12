// This example function calibrates the range of a steering motor and preset the middle (0)
// It is assumed that it is easy to steer the motor and there are physical barriers in the extremes.
//
//
// 

use std::{
    str::FromStr, 
    time::Duration
};

use tokio::time;

use anyhow::{Result};
use btleplug::api::BDAddr;
use rust_powered_lego::{
    hub::{
        Hub
    },
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
            MotorModes,
        },
    },
};


#[tokio::main]
async fn main() -> Result<()> {
    // Hub "MAC" address can be found in several ways. 
    // Connect it to a computer and continue from there...
    let hub_mac_address = "90:84:2b:4e:5b:96";
    let port_id = TechnicHubPorts::B;
    let hub = get_hub(hub_mac_address).await?;
    let motor = hub.get_motor(port_id as u8).await?;

    //Setting notifications. It's important - otherwise the port value won't update
    _ = hub.setup_port_input_format(port_id as u8, MotorModes::Pos as u8, 1, true).await?;

    let mut width = get_degree_width_avg(&hub, port_id as u8, &motor).await?;
    
    // Just in case, let's do it again!
    width = (width + get_degree_width_avg(&hub, port_id as u8, &motor).await?) / 2;

    // No good reason - to be just below the real extreme
    if width & 1 == 1 {
        width -= 1;
    } else {
        width -= 2;
    }

    // Set width to half of it
    width /= 2;

    move_motor_to_extreme(&motor, 1).await?;

    _ = motor.set_abs_position(width as i32, StartupAndCompletionInfo::BufferAndNoAction).await?;

    println!("Done! Calibrated!");

    // Checking what we've done.
    // Please, check if the wheels in the end, are at the middle...
    motor.go_to_abs_position(
        -(width as i32),
        10, 
        15, 
        EndState::HOLD, 
        Profile::AccDec, 
        StartupAndCompletionInfo::ExecuteImmediatelyAndNoAction
    ).await?;

    time::sleep(Duration::from_secs(5)).await;

    motor.go_to_abs_position(
        0,
        10, 
        15, 
        EndState::HOLD, 
        Profile::AccDec, 
        StartupAndCompletionInfo::ExecuteImmediatelyAndNoAction
    ).await?;

    time::sleep(Duration::from_secs(5)).await;

    Ok(())
}

async fn get_hub(address:  &str) -> Result<Hub> {
    
    // Converting the MAC string to btleplug::api::BDAddr type
    let address = BDAddr::from_str(address)?;

    // The ConnectionManager connects stuff - so ask it for the hub...
    let cm = ConnectionManager::new();

    // It is possible to use the name of the hub or its MAC address. That's why it's Option<>
    // Here, only address is implemented
    let hub = cm.get_hub(None, Some(address), 5).await?;
    
    // Great! Let's get on with this...
    Ok(hub)
}


// This is only in an example. So, be aware of setting up the notification prior to this function
// Otherwise it is all in vein...
// The point here is to move to one side, set it as zero, 
// then move to the other side and measure how much it moved
async fn get_degree_width_avg<T, U>(hub: &U, port_id: u8, motor: &T) -> Result<u32> 
where
    T: MotorType,
    U: HubType,
{
    move_motor_to_extreme(motor, 1).await?;
    
    _ = motor.set_abs_position(0, StartupAndCompletionInfo::BufferAndNoAction).await?;
    
    move_motor_to_extreme(motor, -1).await?;
    
    let mut width = hub.get_port_info_raw_value(port_id).await?.abs() as u32;
    
    _ = motor.set_abs_position(0, StartupAndCompletionInfo::BufferAndNoAction).await?;
    
    move_motor_to_extreme(motor, 1).await?;
    
    width += width;
    
    Ok(width / 2)
}

async fn move_motor_to_extreme<T>(motor: &T, sign: i8) -> Result<()> 
where
    T: MotorType,
{
    let speed  = sign.signum() * 10;
    let max_power = 15;
    let end_state = EndState::HOLD;
    let use_profile = Profile::AccDec;
    let start_up_info = StartupAndCompletionInfo::ExecuteImmediatelyAndNoAction;
    let degrees: i32 = 200;
    let sleep_time = 5;
    
    motor.start_speed_for_deg(
        degrees, 
        speed, 
        max_power, 
        end_state, 
        use_profile, 
        start_up_info).await?;
    
    time::sleep(Duration::from_secs(sleep_time)).await;

    Ok(())
}