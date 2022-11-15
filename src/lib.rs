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

use std::pin::Pin;

use anyhow::Result;
use async_trait::async_trait;

use btleplug::api::ValueNotification;
use hub::{
    PortInfoValueReply, PortInfoModeReply, PortInfoCombinationsReply
};
use lego::{
    message_parameters::{
        PortModeInformationType,
        PortOutputCommandParams, 
        StartupAndCompletionInfo,
    },
    consts:: {
        EndState, 
        Profile,
    }
};
use ports::{
    Motor,
};
use tokio_stream::Stream;

pub mod connection_manager;
pub mod hub;
pub mod lego;
pub mod error;
pub mod ports;


/* Hubs type */

#[async_trait]
pub trait HubType {
    
    async fn shut_down_hub(&self) -> Result<()>;

    async fn get_notification(&self) -> Result<Pin<Box<dyn Stream<Item = ValueNotification> + Send>>>;

    async fn get_port_info_value(
        &self, 
        port_id: u8,
    ) -> Result<PortInfoValueReply>;

    async fn get_port_info_raw_value(
        &self,
        port_id: u8
    ) -> Result<i32>;

    async fn get_port_info_mode(
        &self, 
        port_id: u8,
    ) -> Result<PortInfoModeReply>;

    async fn get_port_info_combinations(
        &self, 
        port_id: u8,
    ) -> Result<PortInfoCombinationsReply>;

    async fn get_mode_information(
        &self, 
        port_id: u8, 
        mode_id: u8, 
        info_type: PortModeInformationType
    ) -> Result<Vec<u8>>;

    async fn setup_port_input_format(
        &self,
        port_id:                u8,
        mode_id:                u8,
        delta:                  u32,
        enable_notifications:   bool,
    ) -> Result<()>;

    async fn send_output_command(&self, subcommand: PortOutputCommandParams)-> Result<Vec<u8>>;

    async fn get_motor(&self, port_id: u8) -> Result<Motor>;
}



/* Ports type */

#[async_trait]
pub trait MotorType {
    
    async fn set_acceleration_time(
        &self, 
        time: i16,
        start_up_info: StartupAndCompletionInfo,
    ) -> Result<Vec<u8>>;

    async fn set_deceleration_time(
        &self,
        time: i16,
        start_up_info: StartupAndCompletionInfo,
    ) -> Result<Vec<u8>>;

    async fn start_power(
        &self, 
        power: i8, 
        start_up_info: StartupAndCompletionInfo
    ) -> Result<Vec<u8>>;

    async fn start_speed(
        &self, 
        speed: i8, 
        max_power: i8, 
        use_profile: Profile, 
        start_up_info: StartupAndCompletionInfo,
    ) -> Result<Vec<u8>>;

    async fn stop_motor(
        &self,
        end_state: EndState,
        use_profile: Profile, 
        start_up_info: StartupAndCompletionInfo
    ) -> Result<Vec<u8>>;

    async fn set_abs_position(
        &self, 
        position: i32, 
        start_up_info: StartupAndCompletionInfo
    ) -> Result<Vec<u8>>;

    async fn go_to_abs_position(
        &self, 
        abs_pos: i32,
        speed: i8,
        max_power: i8,
        end_state: EndState,
        use_profile: Profile,
        start_up_info: StartupAndCompletionInfo,
    ) -> Result<Vec<u8>>;

    async fn start_speed_for_deg (
        &self, 
        degrees: i32,
        speed: i8,
        max_power: i8,
        end_state: EndState,
        use_profile: Profile,
        start_up_info: StartupAndCompletionInfo,
    ) -> Result<Vec<u8>>;
}