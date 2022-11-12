use core::result::Result::Ok;
use std::pin::Pin;
use async_trait::async_trait;
use btleplug::api::ValueNotification;
use num_traits::FromPrimitive;

//use anyhow::Ok;
use anyhow::bail;
use btleplug::platform::Peripheral;

use anyhow::Result;
use byteorder::ByteOrder;
use byteorder::LittleEndian;
use tokio_stream::Stream;

use crate::HubType;
use crate::lego::{
    Communicator,
    MessageTypes,
};
use crate::lego::{
    message_parameters:: {
        HubActionsParams,
        HubActionsTypes,
        PortInformationType,
        PortInformationRequestParams,
        PortModeInformationType,
        PortModeInformationRequestParams,
        PortInputFormatSetupSingleParams,
        PortOutputCommandParams
    },
    consts::{
        PortType,
        PortInfoModeReplyCapabilities,
    },
};
use crate::ports::Motor;

pub struct Hub {
    communicator: Communicator,
}

impl Hub {
    pub async fn new(p: Peripheral) -> Result<Self> {
        let communicator = Communicator::new(p).await?;
        Ok(Self { communicator })
    }

    async fn get_port_info(&self, port_id: u8, information_type: PortInformationType) -> Result<Vec<u8>> {
        self.communicator.send_message(
            MessageTypes::PortInformationRequest,
            PortInformationRequestParams {
                port_id: port_id as u8,
                information_type: information_type,
            }
        ).await?;
        self.communicator.read_message().await
    }
}

#[async_trait]
impl HubType for Hub {

    async fn shut_down_hub(&self) -> Result<()> {
        self.communicator.send_message(
            MessageTypes::HubActions,
            HubActionsParams {
                action_type: HubActionsTypes::Shutdown,
            }
        ).await
    }

    async fn get_notification(&self) -> Result<Pin<Box<dyn Stream<Item = ValueNotification> + Send>>> {
        //self.communicator.subscribe_for_notifications().await?;
        Ok(self.communicator.get_notification_stream().await?)
    }

    async fn get_port_info_value(
        &self, 
        port_id: u8,
    ) -> Result<PortInfoValueReply> {
        let msg = self.get_port_info(
            port_id, 
            PortInformationType::PortValue).await?;
        Ok(PortInfoValueReply {port_type: FromPrimitive::from_u8(msg[5])})
    }

    async fn get_port_info_raw_value(
        &self,
        port_id: u8
    ) -> Result<i32> {
        let msg = self.get_port_info(port_id, PortInformationType::PortValue).await?;
        match msg[0] {
            0x05 => Ok(i32::from_u8(msg[4]).unwrap()),
            0x06 => Ok(i32::from_u16(u16::from_le_bytes(msg[4..6].try_into().unwrap())).unwrap()),
            0x08 => Ok(i32::from_le_bytes(msg[4..8].try_into().unwrap())),
            _ => bail!("Such port value reply is not currently supported.")
        }
    }

    async fn get_port_info_mode(
        &self, 
        port_id: u8,
    ) -> Result<PortInfoModeReply> {
        let msg = self.get_port_info(
            port_id, 
            PortInformationType::ModeInfo).await?;
        Ok(PortInfoModeReply {
            port_id:            msg[3].clone(), 
            info_type:          msg[4].clone(), 
            capabilities:       parse_capabilities(msg[5]), 
            total_mode_count:   msg[6].clone(), 
            input_modes:        parse_io_modes(LittleEndian::read_u16(&msg[7..9])), 
            output_modes:       parse_io_modes(LittleEndian::read_u16(&msg[9..11])),
        })
    }

    async fn get_port_info_combinations(
        &self, 
        port_id: u8,
    ) -> Result<PortInfoCombinationsReply> {
        let msg = self.get_port_info(
            port_id, 
            PortInformationType::PossibleModeCombinations).await?;
        Ok(PortInfoCombinationsReply {data: msg})
    }    

    async fn get_mode_information(
        &self, 
        port_id: u8, 
        mode_id: u8, 
        info_type: PortModeInformationType
    ) -> Result<Vec<u8>>
    {
        self.communicator.send_message(
            MessageTypes::PortModeInformationRequest,
            PortModeInformationRequestParams {
                port_id: port_id as u8,
                mode_id: mode_id,
                information_type: info_type,
            }
        ).await?;
        self.communicator.read_message().await
    }

    async fn setup_port_input_format(
        &self,
        port_id:                u8,
        mode_id:                u8,
        delta:                  u32,
        enable_notifications:   bool,
    ) -> Result<()> {
        self.communicator.send_message(
            MessageTypes::PortInputFormatSetupSingle,
            PortInputFormatSetupSingleParams {
                port_id:                port_id,
                mode_id:                mode_id,
                delta:                  delta,
                enable_notifications:   enable_notifications,
            }
        ).await?;
        _ = self.communicator.read_message().await?;
        Ok(())
    }

    async fn send_output_command(&self, subcommand: PortOutputCommandParams)-> Result<Vec<u8>> {
        self.communicator.send_message(
            MessageTypes::PortOutputCommand,
            subcommand
        ).await?;
        self.communicator.read_message().await
    }

    async fn get_motor(&self, port_id: u8) -> Result<Motor> {
        // (TODO) Make sure this is really a motor!!
        Ok(Motor {
            hub: self,
            port_id: port_id as u8
        })
    }



}

// (TODO) Not fully implemented yet
#[derive(Debug)]
pub struct PortInfoValueReply {
    pub port_type: Option<PortType>,
}

// (TODO) Not fully implemented yet
#[derive(Debug)]
pub struct PortInfoCombinationsReply {
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct PortRawValueReply {
    pub port_id:    u8,
    pub value:      i32,
}

#[derive(Debug)]
pub struct PortInfoModeReply {
    pub port_id:            u8,
    pub info_type:          u8,
    pub capabilities:       Vec<PortInfoModeReplyCapabilities>,
    pub total_mode_count:   u8,
    pub input_modes:        Vec<u8>,
    pub output_modes:       Vec<u8>,
}

fn parse_capabilities(capabilities: u8) -> Vec<PortInfoModeReplyCapabilities> {
    let mut res: Vec<PortInfoModeReplyCapabilities> = Vec::new();
    if capabilities & 0x1 == 0x1 {
        res.push(PortInfoModeReplyCapabilities::Output);
    }
    if capabilities >> 1 & 0x1 == 0x1 {
        res.push(PortInfoModeReplyCapabilities::Input);
    }
    if capabilities >> 1 & 0x1 == 0x1 {
        res.push(PortInfoModeReplyCapabilities::LogicalCombinable);
    }
    if capabilities >> 1 & 0x1 == 0x1 {
        res.push(PortInfoModeReplyCapabilities::LogicalSynchronizable);
    } 
    res
}

fn parse_io_modes(mut modes: u16) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for i in 0..16 as u8  {
        if modes & 0x1 == 0x1 {
            res.push(i);
        }
        modes >>= 1;
    }
    res
}