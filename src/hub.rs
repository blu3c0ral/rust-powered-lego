use core::result::Result::Ok;
use async_trait::async_trait;
use num_traits::FromPrimitive;

//use anyhow::Ok;
use anyhow::bail;
use btleplug::platform::Peripheral;

use anyhow::Result;
use byteorder::ByteOrder;
use byteorder::LittleEndian;

use crate::HubType;
use crate::lego::{
    Communicator,
    MessageTypes,
    check_for_lego_error,
};
use crate::lego::message_parameters:: {
    HubActionsParams,
    HubActionsTypes,
    PortInformationType,
    PortInformationRequestParams,
    PortModeInformationType,
    PortModeInformationRequestParams,
    PortOutputCommandParams
};
use crate::ports::{Motor, PortType, MOTOR_TYPES};


// (TODO) Implement device checking of these values
pub enum HubTypes {
    TechnicHub,         // # item: 88012
    HubHub,             // # item: 88009
}

#[allow(dead_code)]
enum HubTypesSystemId {
    TechnicHubSystemId  = 0b1000000,
    HubHubSystemId      = 0b1000001,
}


/***************************************/
/***************** Hub *****************/
/***************************************/

pub struct Hub {
    communicator: Communicator,
}

impl Hub {
    pub async fn new(p: Peripheral) -> Result<Self> {
        let communicator = Communicator::new(p).await?;
        Ok(Self { communicator })
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

    async fn get_port_information(
        &self, 
        port_id: u8, 
        info_type: PortInformationType
    ) -> Result<PortInfoReply> 
    {
        self.communicator.send_message(
            MessageTypes::PortInformationRequest,
            PortInformationRequestParams {
                port_id: port_id as u8,
                information_type: info_type,
            }
        ).await?;
        let msg = self.communicator.read_message().await?;
        match check_for_lego_error(&msg) {
            Ok(_) => {
                parse_port_info_reply(msg)
            }
            Err(e) => bail!(e)
        }
    }

    async fn get_mode_information(
        &self, 
        port_id: TechnicHubPorts, 
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


    async fn send_output_command(&self, subcommand: PortOutputCommandParams)-> Result<Vec<u8>> {
        self.communicator.send_message(
            MessageTypes::PortOutputCommand,
            subcommand
        ).await?;
        self.communicator.read_message().await
    }

    async fn get_motor(&self, port_id: u8) -> Result<Motor> {
        let reply = self.get_port_information(
            port_id, 
            PortInformationType::PortValue
        ).await?;

        match reply {
            PortInfoReply::PortInfoValueReplyParsed(val) => {
                if val.port_type.is_some() {
                    if !MOTOR_TYPES.contains(&(val.port_type.unwrap())) {
                        bail!("Got port type not compatible with a motor type")
                    };
                };
            }
            _ => bail!("Got port type not compatible with a motor type")
        }

        Ok(Motor {
            hub: self,
            port_id
        })


    }



}

// This function parse an info request reply.
// The function make a sanity check and then make sense out of the reply
// into a formatted String.
// (TODO) Parse PortValue and PossibleModeCombinations info replies.
// (TODO) Export some struct out of the reply containing the values.
//        Right now, there is nothing to do with this values...
//
// reply: Vec<u8> is the entire upstream message
pub fn parse_port_info_reply(reply: Vec<u8>) -> Result<PortInfoReply> {
    /* Sanity check */
    if !((reply[2] == 0x43) || (reply[2] == 0x45) || (reply[2] == 0x46)) {
        bail!("[Error] The submitted reply is not valid reply for port information request")
    }

    match reply[4] {
        0x0 => Ok(
            PortInfoReply::PortInfoValueReplyParsed(PortInfoValueReply {port_type: FromPrimitive::from_u8(reply[5])})
        ),
        0x1 => {
            Ok(PortInfoReply::PortInfoModeReplyParsed(
                PortInfoModeReply {
                    port_id:            reply[3].clone(), 
                    info_type:          reply[4].clone(), 
                    capabilities:       parse_capabilities(reply[5]), 
                    total_mode_count:   reply[6].clone(), 
                    input_modes:        parse_io_modes(LittleEndian::read_u16(&reply[7..9])), 
                    output_modes:       parse_io_modes(LittleEndian::read_u16(&reply[9..11])),
                }
            ))
        },
        0x2 => Ok(
            PortInfoReply::PortInfoCombinationsReplyParsed(PortInfoCombinationsReply {data: reply})
        ),
        _   => bail!("[Error] Information type not supported")
    }
}

#[derive(Debug)]
pub enum PortInfoReply {
    PortInfoValueReplyParsed(PortInfoValueReply),
    PortInfoModeReplyParsed(PortInfoModeReply),
    PortInfoCombinationsReplyParsed(PortInfoCombinationsReply),
}

// (TODO) Not fully implemented yet
#[derive(Debug)]
pub struct PortInfoValueReply {
    port_type: Option<PortType>,
}

// (TODO) Not fully implemented yet
#[derive(Debug)]
pub struct PortInfoCombinationsReply {
    data: Vec<u8>,
}

#[derive(Debug)]
pub struct PortInfoModeReply {
    port_id:            u8,
    info_type:          u8,
    capabilities:       Vec<PortInfoModeReplyCapabilities>,
    total_mode_count:   u8,
    input_modes:        Vec<u8>,
    output_modes:       Vec<u8>,
}

#[derive(Debug)]
enum PortInfoModeReplyCapabilities {
    Output                  = 0x0,  // Output (seen from Hub)
    Input                   = 0x1,  // Input (seen from Hub)
    LogicalCombinable       = 0x2,  // Logical Combinable
    LogicalSynchronizable   = 0x3,  // Logical Synchronizable
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



/***************************************/
/************* Technic Hub *************/
/***************************************/


/* Below consts are taken from https://github.com/corneliusmunz/legoino/blob/master/src/Lpf2HubConst.h */
/* Same values are in https://github.com/sciguy16/lego-powered-up/blob/main/lego-powered-up/src/hubs.rs */
#[derive(Clone, Copy)]
pub enum TechnicHubPorts {
    A               = 0x00,
    B               = 0x01,
    C               = 0x02,
    D               = 0x03,
    LED             = 0x32,
    CURRENT         = 0x3B,
    VOLTAGE         = 0x3C,
    ACCELEROMETER   = 0x61,
    GYRO            = 0x62,
    TILT            = 0x63,
}