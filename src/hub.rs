use btleplug::platform::Peripheral;

use anyhow::Result;

use crate::lego::Communicator;
use crate::lego::MessageTypes;
use crate::lego::message_parameters;

pub enum HubTypes {
    TechnicHub,         // # item: 88012
    HubHub,             // # item: 88009
}

enum HubTypesSystemId {
    TechnicHubSystemId  = 0b1000000,
    HubHubSystemId      = 0b1000001,
}



/***************************************/
/************* Technic Hub *************/
/***************************************/


/* Below consts are taken from https://github.com/corneliusmunz/legoino/blob/master/src/Lpf2HubConst.h */
/* Same values are in https://github.com/sciguy16/lego-powered-up/blob/main/lego-powered-up/src/hubs.rs */
pub enum TechnicHubPortsId {
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

pub struct TechnicHub {
    communicator: Communicator,
}


impl TechnicHub {
    pub async fn new(p: Peripheral) -> Result<Self> {
        let communicator = Communicator::new(p).await?;
        Ok(Self { communicator })
    }

    pub async fn shut_down_hub(&self) -> Result<()> {
        self.communicator.send_message(
            MessageTypes::HubActions,
            message_parameters::HubActionsParams {
                action_type: message_parameters::HubActionsTypes::Shutdown,
            }
        ).await
    }

    pub async fn get_port_information(&self, port_id: u8, info_type: message_parameters::PortInformationType) -> Result<Vec<u8>> {
        self.communicator.send_message(
            MessageTypes::PortInformationRequest,
            message_parameters::PortInformationRequestParams {
                port_id,
                information_type: info_type,
            }
        ).await?;
        self.communicator.read_message().await
    }

}

