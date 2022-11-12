#![allow(clippy::single_match)]

use std::str::FromStr;

use rust_powered_lego::hub::Hub;
use rust_powered_lego::connection_manager::ConnectionManager;
use btleplug::api::BDAddr;
use winit::{
    event::{Event, KeyboardInput, DeviceEvent},
    event_loop::{EventLoop, DeviceEventFilter}
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {

    let hub_mac_address = "90:84:2b:4e:5b:96";
    _ = get_hub(hub_mac_address).await?;

    let event_loop = EventLoop::new();

    event_loop.set_device_event_filter(DeviceEventFilter::Never);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match &event {
            Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {..}), ..} => {
                println!("Event: {:?}", &event);
            }
            _ => (),
        }
    });
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
