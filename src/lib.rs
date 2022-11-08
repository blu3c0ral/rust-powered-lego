//! This crate aims to control Lego hub and its various peripherals with the Rust programming language.
//! 
//! ConnectionManager is the object to start with.
//! Ask the CM for a Hub, with its name or address.
//! Then, kindly ask the Hub to get a port of specific type (sorry, currently only the motor).
//! 
//! This crate is far from completion - more types and functionality will be added in the future.
//! 
//! Below are the main traits. More located at lego/mod.rs
//! 

use anyhow::Result;
use async_trait::async_trait;

use hub::{
    PortInfoReply, 
    TechnicHubPorts
};
use lego::message_parameters::{
    PortInformationType, 
    PortModeInformationType,
    PortOutputCommandParams
};
use ports::Motor;

pub mod connection_manager;
pub mod hub;
pub mod lego;
pub mod error;
pub mod ports;


/* Hubs type */

#[async_trait]
pub trait HubType {
    
    async fn shut_down_hub(&self) -> Result<()>;

    async fn get_port_information(
        &self, 
        port_id: u8, 
        info_type: PortInformationType
    ) -> Result<PortInfoReply>;

    async fn get_mode_information(
        &self, 
        port_id: TechnicHubPorts, 
        mode_id: u8, 
        info_type: PortModeInformationType
    ) -> Result<Vec<u8>>;

    async fn send_output_command(&self, subcommand: PortOutputCommandParams)-> Result<Vec<u8>>;

    async fn get_motor(&self, port_id: u8) -> Result<Motor>;
}



/* Ports type */

#[async_trait]
pub trait MotorType {
    
    async fn set_acceleration_time(
        &self, 
        time: i16,
        feedback: bool,
    ) -> Result<Vec<u8>>;

    async fn set_deceleration_time(
        &self,
        time: i16,
        feedback: bool,
    ) -> Result<Vec<u8>>;

    async fn start_speed(
        &self, 
        speed: i8, 
        max_power: i8, 
        use_profile: bool, 
        feedback: bool
    ) -> Result<Vec<u8>>;

    async fn stop_motor(&self, use_profile: bool, feedback: bool) -> Result<Vec<u8>>;

    async fn set_abs_position(&self, position: i32, feedback: bool) -> Result<Vec<u8>>;

    async fn go_to_aps_position(
        &self, 
        abs_pos: i32,
        speed: i8,
        max_power: i8,
        end_state: i8,
        use_profile: bool,
        feedback: bool
    ) -> Result<Vec<u8>>;

    async fn start_speed_for_deg (
        &self, 
        degrees: i32,
        speed: i8,
        max_power: i8,
        end_state: i8,
        use_profile: bool,
        feedback: bool
    ) -> Result<Vec<u8>>;
}