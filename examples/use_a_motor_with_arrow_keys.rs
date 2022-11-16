/// This example shows how to use this crate to control a motor using the Up & Down keys.

use std::str::FromStr;

use rust_powered_lego::{MotorType, HubType};
use rust_powered_lego::lego::consts::{Profile, EndState, TechnicHubPorts};
use rust_powered_lego::lego::message_parameters::StartupAndCompletionInfo;
use rust_powered_lego::hub::Hub;
use rust_powered_lego::connection_manager::ConnectionManager;
use btleplug::api::BDAddr;
use tokio::sync::mpsc::{self, UnboundedSender, UnboundedReceiver};
use tokio::runtime::Builder;
use winit::event::{VirtualKeyCode, ElementState};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::{
    event::{Event, KeyboardInput, DeviceEvent},
    event_loop::{EventLoop, DeviceEventFilter}
};
use anyhow::Result;


/// When a user press the Up key - the vehicle should move forward
async fn activate_motor_forward<T>(motor: &T)
where
    T: MotorType 
{
        _ = motor.start_speed(
            // 1% to 100% of speed
            100,
            // Not using power greater than max_power
            // 1% - 100%. See calibrating_steering for reasons to use less than 100%.
            100,
            // The profiles are controlling how quickly the motor will get to the speed
            Profile::AccDec,
            // Do it now and don't care about what the motor has to say about that
            StartupAndCompletionInfo::ExecuteImmediatelyAndNoAction,
        ).await;
}

/// When a user press the Down key - the vehicle should move backward
async fn activate_motor_backward<T>(motor: &T)
where
    T: MotorType 
{
        _ = motor.start_speed(
            // -1% to -100% of speed
            -100,
            100,
            Profile::AccDec,
            StartupAndCompletionInfo::ExecuteImmediatelyAndNoAction,
        ).await;
}

/// When a user release any key that moved the vehicle - the motor should stop
async fn release_motor<T>(motor: &T)
where
    T: MotorType,
{
    _ = motor.stop_motor(
        // When the key is released - the motor has to be powerless, but not stopped
        EndState::FLOAT, 
        Profile::AccDec, 
        StartupAndCompletionInfo::ExecuteImmediatelyAndNoAction
    ).await;
}

/// controller function gets the raw events from the event loop and decides what kind of command to execute
/// See below as to how to create the hub itself.
/// The hub itself is moved here because the motor has a reference to it. So, in order to avoid lifetime issues,
/// both lives together.
/// event_rx is the recieving end of the mpsc channel
async fn controller(hub: impl HubType,  mut event_rx: UnboundedReceiver<Option<Event<'_, ()>>>)
{
    let motor = hub.get_motor(TechnicHubPorts::A as u8).await.unwrap();
    let mut up_pressed = false;
    let mut down_pressed = false;
    loop {
        let maybe_event = event_rx.recv().await.unwrap();
        
        match maybe_event {
            Some(event) => {
                match &event {
                    Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Up),
                                state: ElementState::Pressed, 
                                ..}), 
                                ..} => {if !up_pressed {up_pressed = true; activate_motor_forward(&motor).await;}}
                    Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Up),
                                state: ElementState::Released, 
                                ..}), 
                                ..} => {up_pressed = false; release_motor(&motor).await;}
                    Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Down),
                                state: ElementState::Pressed, 
                                ..}), 
                                ..} => {if !down_pressed {down_pressed = true; activate_motor_backward(&motor).await;}}
                    Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Down),
                                state: ElementState::Released, 
                                ..}), 
                                ..} => {down_pressed = false; release_motor(&motor).await;}
                    Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..}), 
                                ..} => return,          
                    _ => (),
                }
            },
            None => continue,
        }
    }
}

/// This is the winit crate event_loop.
/// event_tx is a sender of the mpsc channel.
/// Each event is being sent to the event_rx resides in the controller
fn start_event_loop(event_tx: UnboundedSender<Option<Event<()>>>) {
    let mut event_loop = EventLoop::new();

    event_loop.set_device_event_filter(DeviceEventFilter::Never);

    event_loop.run_return(move |event, _, control_flow| {
        control_flow.set_wait();
        event_tx.send(event.to_static()).unwrap();
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

fn main() -> Result<()> {
    
    // btleplug is async crate so a runtime is needed.
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    // Creating the channel between the event_loop to the controller
    let (event_tx, event_rx) = mpsc::unbounded_channel();

    // Starting the runtime
    runtime.block_on(async {
        let hub_mac_address = "90:84:2b:4e:5b:96";
        let hub = get_hub(hub_mac_address).await.unwrap();
        // Seperating the controller (async function) from the event_loop (sync function)
        runtime.spawn(async move {
            controller(hub, event_rx).await;
        });
    });

    // That's it!
    start_event_loop(event_tx);

    Ok(())
}
