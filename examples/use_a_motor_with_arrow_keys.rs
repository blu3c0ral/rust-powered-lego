#![allow(clippy::single_match)]

use std::process::exit;
use std::str::FromStr;

use crossterm::event;
use rust_powered_lego::{MotorType, HubType};
use rust_powered_lego::lego::consts::{Profile, EndState, TechnicHubPorts};
use rust_powered_lego::lego::message_parameters::StartupAndCompletionInfo;
use rust_powered_lego::{hub::Hub, ports::Motor};
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
use anyhow::{Result};

async fn activate_motor_forward<T>(motor: &T)
where
    T: MotorType 
{
        _ = motor.start_speed(
            // 1% to 100% of speed
            100,
            // Not using power greater than max_power
            100,
            // The profiles are controlling how quickly the motor will get to the speed
            Profile::AccDec,
            // Do it now and don't care about what the motor has to say about that
            StartupAndCompletionInfo::ExecuteImmediatelyAndNoAction,
        ).await;
}

async fn activate_motor_backward<T>(motor: &T)
where
    T: MotorType 
{
        _ = motor.start_speed(
            // -1% to -100% of speed
            -100,
            // Not using power greater than max_power
            100,
            // The profiles are controlling how quickly the motor will get to the speed
            Profile::AccDec,
            // Do it now and don't care about what the motor has to say about that
            StartupAndCompletionInfo::ExecuteImmediatelyAndNoAction,
        ).await;
}

async fn release_motor<T>(motor: &T)
where
    T: MotorType,
{
    motor.stop_motor(
        // When the key is released - the motor has to be powerless, but not stopped
        EndState::FLOAT, 
        Profile::AccDec, 
        StartupAndCompletionInfo::ExecuteImmediatelyAndNoAction
    ).await;
}

async fn controller(/*motor: impl MotorType, */ hub: impl HubType,  mut event_rx: UnboundedReceiver<Option<Event<'_, ()>>>)
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
                                ..} => return , //*control_flow = ControlFlow::Exit,                  
                    _ => (),
                }
            },
            None => continue,
        }
    }
}

fn start_event_loop(event_tx: UnboundedSender<Option<Event<()>>>) {
    let mut event_loop = EventLoop::new();

    event_loop.set_device_event_filter(DeviceEventFilter::Never);

    event_loop.run_return(move |event, _, control_flow| {
        control_flow.set_wait();
        event_tx.send(event.to_static()).unwrap();
    });
}

//#[tokio::main]
fn main() -> Result<()> {
    
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let (event_tx, event_rx) = mpsc::unbounded_channel();

    runtime.block_on(async {
        let hub_mac_address = "90:84:2b:4e:5b:96";
        let hub = get_hub(hub_mac_address).await.unwrap();
        runtime.spawn(async move {
            controller(hub, event_rx).await;
        });
    });

    start_event_loop(event_tx);

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
