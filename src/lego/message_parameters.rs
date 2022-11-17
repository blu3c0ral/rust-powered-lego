// Long list of structs representing each command / message + valid values of its parameters
// Simple command is transfered as is. Complicated command needs encoding.
// See message_types for list of these commands / messages.

use crate::lego::consts::{
    EndState, 
    Profile
};

use super::message_types::SubcommandType;


// The communicator is expected to send command as [u8]
// Since each command has different way of definition and payload,
// this trait aims to standardized the serialization process.
pub trait Serialized {
    fn serialize(&self) -> Vec<u8>;
}

/***************************************/
/************ HubProperties ************/
/***************************************/

pub struct HubPropertiesParams { 
    pub property:           HubPropertiesProperties,
    pub operation:          HubPropertiesOperations,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum HubPropertiesProperties {
    AdvertisingName                 = 0x01, // Advertising Name
    Button                          = 0x02,	// Button
    FWVersion                       = 0x03,	// FW Version
    HWVersion                       = 0x04,	// HW Version
    RSSI                            = 0x05,	// RSSI
    BatteryVoltage                  = 0x06,	// Battery Voltage
    BatteryType                     = 0x07,	// Battery Type
    ManufacturerName                = 0x08,	// Manufacturer Name
    RadioFirmwareVersion            = 0x09,	// Radio Firmware Version
    LEGOWirelessProtocolVersion     = 0x0A,	// LEGO Wireless Protocol Version
    SystemTypeID                    = 0x0B,	// System Type ID
    HWNetworkID                     = 0x0C,	// H/W Network ID
    PrimaryMACAddress               = 0x0D,	// Primary MAC Address
    SecondaryMACAddress             = 0x0E,	// Secondary MAC Address
    HardwareNetworkFamily           = 0x0F,	// Hardware Network Family
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum HubPropertiesOperations {
    Set             = 0x01, // Set              (Downstream)
    EnableUpdates   = 0x02, // Enable Updates   (Downstream)
    DisableUpdates  = 0x03, // Disable Updates  (Downstream)
    Reset           = 0x04, // Reset            (Downstream)
    RequestUpdate   = 0x05, // Request Update   (Downstream)
    Update          = 0x06, // Update           (Upstream)
}

impl Serialized for HubPropertiesParams {
    fn serialize(&self) -> Vec<u8> {
        vec![self.property as u8, self.operation as u8]
    }
}


/***************************************/
/************* HubActions **************/
/***************************************/

pub struct HubActionsParams {
    pub action_type:        HubActionsTypes,
}

#[derive(Clone, Copy)]
pub enum HubActionsTypes {
    SwitchOffHub            = 0x01, // Switch Off Hub
    Disconnect              = 0x02,	// Disconnect
    VCCPortControlOn        = 0x03,	// VCC Port Control On
    VCCPortControlOff       = 0x04,	// VCC Port Control Off
    ActivateBUSYIndication  = 0x05,	// Activate BUSY Indication (Shown byRGB. Actual RGB settings preserved).
    ResetBUSYIndication     = 0x06,	// Reset BUSY Indication (RGB shows the previously preserve RGB settings).
    Shutdown                = 0x2f,	// Shutdown the Hub without any up-stream information send. Used for fast power down in production. Suggested for PRODUCTION USE ONLY!

    /* Only Upstream */
    HubWillSwitchOff        = 0x30, // Hub Will Switch Off
    HubWillDisconnect       = 0x31,	// Hub Will Disconnect
    HubWillGoIntoBootMode   = 0x32,	// Hub Will Go Into Boot Mode
}

impl Serialized for HubActionsParams {
    fn serialize(&self) -> Vec<u8> {
        vec![self.action_type as u8]
    }
}



/***************************************/
/******* PortInformationRequest ********/
/***************************************/

pub struct PortInformationRequestParams {
    pub port_id:            u8,
    pub information_type:   PortInformationType,
}

#[derive(Clone, Copy)]
pub enum PortInformationType {
    PortValue                   = 0x00, // Port Value
    ModeInfo                    = 0x01, // Mode Info
    PossibleModeCombinations    = 0x02, //Possible Mode Combinations. Should only be used if the “Logical Combinable”-bit is set in the (MODE INFO Capabilities byte). I.e. in the Port Information 0x43
}

impl Serialized for PortInformationRequestParams {
    fn serialize(&self) -> Vec<u8> {
        vec![self.port_id, self.information_type as u8]
    }
}


/***************************************/
/***** PortModeInformationRequest ******/
/***************************************/

pub struct PortModeInformationRequestParams {
    pub port_id:            u8,
    pub mode_id:            u8,
    pub information_type:   PortModeInformationType,
}

#[derive(Clone, Copy)]
pub enum PortModeInformationType {
    Name            = 0x00,    // NAME	                                Name of the mode
    Raw             = 0x01,    // RAW	                                The raw range
    Pct             = 0x02,    // PCT	                                The percent range
    Si              = 0x03,    // SI	                                The SI value range
    Symbol          = 0x04,    // SYMBOL	                            The standard name of value
    Mapping         = 0x05,    // MAPPING	 
    Internal        = 0x06,    // Used internally
    MotorBias       = 0x07,    // Motor Bias (0-100%)
    CapabilityBits  = 0x08,    // Capability bits (6 bytes total)
    ValueFormat     = 0x80,    // VALUE FORMAT	Value encoding
}

impl Serialized for PortModeInformationRequestParams {
    fn serialize(&self) -> Vec<u8> {
        vec![self.port_id, self.mode_id, self.information_type as u8]
    }
}


/***************************************/
/***** PortInputFormatSetupSingle ******/
/***************************************/

pub struct PortInputFormatSetupSingleParams {
    pub port_id:                u8,
    pub mode_id:                u8,
    pub delta:                  u32,
    pub enable_notifications:   bool,
}

impl Serialized for PortInputFormatSetupSingleParams {
    fn serialize(&self) -> Vec<u8> {
        let mut data = vec![self.port_id, self.mode_id];
        data.append(&mut Vec::from(self.delta.to_le_bytes()));
        data.push(if self.enable_notifications {1} else {0});
        data
    }
}


/***************************************/
/********** PortOutputCommand **********/
/***************************************/

pub struct PortOutputCommandParams {
    pub port_id:        u8,
    pub start_up_info:  StartupAndCompletionInfo,
    pub subcommand_id:  SubcommandType,
    pub payload:        SubcommandPayload,
}

impl Serialized for PortOutputCommandParams {
    fn serialize(&self) -> Vec<u8> {
        let mut res = vec![self.port_id, self.start_up_info as u8, self.subcommand_id as u8];
        res.append(self.payload.serialize().as_mut());
        res
    }
}

#[derive(Clone, Copy)]
pub enum StartupAndCompletionInfo {
    BufferAndNoAction               = 0b00000000,
    BufferAndFeedback               = 0b00000001,
    ExecuteImmediatelyAndNoAction   = 0b00010000,
    ExecuteImmediatelyAndFeedback   = 0b00010001,
}





////////////////////////////////////////////////////////////////////////////////
/******************************************************************************/
/*************************** Subcommands Parameters ***************************/
/*************************** Subcommands Parameters ***************************/
/*************************** Subcommands Parameters ***************************/
/******************************************************************************/
////////////////////////////////////////////////////////////////////////////////



pub enum SubcommandPayload {
    SetAccTime(SetAccTimePayload),
    SetDecTime(SetDecTimePayload),
    StartSpeed(StartSpeedPayload),
    StartSpeedForDegrees(StartSpeedForDegreesPayload),
    GotoAbsolutePosition(GotoAbsolutePositionPayload),
    WriteDirectModeData(WriteDirectModeDataPayload),
}

impl Serialized for SubcommandPayload {
    fn serialize(&self) -> Vec<u8> {
        match self {
            SubcommandPayload::SetAccTime(payload) => {
                payload.serialize()
            },
            SubcommandPayload::SetDecTime(payload) => {
                payload.serialize()
            },
            SubcommandPayload::StartSpeed(payload) => {
                payload.serialize()
            },
            SubcommandPayload::StartSpeedForDegrees(payload) => {
                payload.serialize()
            },
            SubcommandPayload::GotoAbsolutePosition(payload) => {
                payload.serialize()
            },
            SubcommandPayload::WriteDirectModeData(payload) => {
                payload.serialize()
            },
        }
    }
}


/***************************************/
/************* SetAccTime **************/
/***************************************/

pub struct SetAccTimePayload {
    pub time:   i16,
}

impl Serialized for SetAccTimePayload {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::from(self.time.to_le_bytes());
        data.append(vec![
            Profile::Acc as u8,
        ].as_mut());
        data
    }
}


/***************************************/
/************* SetAccTime **************/
/***************************************/

pub struct SetDecTimePayload {
    pub time:   i16,
}

impl Serialized for SetDecTimePayload {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::from(self.time.to_le_bytes());
        data.append(vec![
            Profile::Dec as u8,
        ].as_mut());
        data
    }
}


/***************************************/
/************* StartSpeed **************/
/***************************************/

pub struct StartSpeedPayload {
    pub speed:          i8,
    pub max_power:      i8,
    pub use_profile:    Profile,
}

impl Serialized for StartSpeedPayload {
    fn serialize(&self) -> Vec<u8> {
        vec![
            self.speed.to_le_bytes()[0], 
            self.max_power.to_le_bytes()[0], 
            self.use_profile as u8
            ]
    }
}


/***************************************/
/******** StartSpeedForDegrees *********/
/***************************************/

pub struct StartSpeedForDegreesPayload {
    pub degrees:        i32,
    pub speed:          i8,
    pub max_power:      i8,
    pub end_state:      EndState,
    pub use_profile:    Profile,
}

impl Serialized for StartSpeedForDegreesPayload {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::from(self.degrees.to_le_bytes());
        data.append(vec![
            self.speed.to_le_bytes()[0],
            self.max_power.to_le_bytes()[0],
            (self.end_state as u8).to_le_bytes()[0],
            self.use_profile as u8,
        ].as_mut());
        data
    }
}


/***************************************/
/******** GotoAbsolutePosition *********/
/***************************************/

pub struct GotoAbsolutePositionPayload {
    pub abs_pos:        i32,        // Degrees
    pub speed:          i8,
    pub max_power:      i8,
    pub end_state:      EndState,
    pub use_profile:    Profile,
}

impl Serialized for GotoAbsolutePositionPayload {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::from(self.abs_pos.to_le_bytes());
        data.append(vec![
            self.speed.to_le_bytes()[0],
            self.max_power.to_le_bytes()[0],
            (self.end_state as u8).to_le_bytes()[0],
            self.use_profile as u8,
        ].as_mut());
        data
    }
}


/***************************************/
/********* WriteDirectModeData *********/
/***************************************/

pub struct WriteDirectModeDataPayload {
    pub mode:       u8,
    pub payload:    WriteDirectModeDataCommands,
}

impl Serialized for WriteDirectModeDataPayload {
    fn serialize(&self) -> Vec<u8> {
        let mut data = vec![self.mode];
        data.append(self.payload.serialize().as_mut());
        data
    }
}

pub enum WriteDirectModeDataCommands {
    StartPower(StartPowerPayload),
    SetAbsolutePosition(SetAbsolutePositionPayload),
}

impl Serialized for WriteDirectModeDataCommands {
    fn serialize(&self) -> Vec<u8> {
        match self {
            WriteDirectModeDataCommands::StartPower(payload) => {
                payload.serialize()
            },
            WriteDirectModeDataCommands::SetAbsolutePosition(payload) => {
                payload.serialize()
            },
        }
    }
}


/************* WriteDirectModeDataCommands *************/


/***************************************/
/************* StartPower **************/
/***************************************/

pub struct StartPowerPayload {
    pub power: i8,
}

impl Serialized for StartPowerPayload {
    fn serialize(&self) -> Vec<u8> {
        Vec::from(self.power.to_le_bytes())
    }
}


/***************************************/
/********* SetAbsolutePosition *********/
/***************************************/

pub struct SetAbsolutePositionPayload {
    pub position: i32,
}

impl Serialized for SetAbsolutePositionPayload {
    fn serialize(&self) -> Vec<u8> {
        Vec::from(self.position.to_le_bytes())
    }
}