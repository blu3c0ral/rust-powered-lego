// Long list of structs representing each command / message + valid values of its parameters
// Simple command is transfered as is. Complicated command needs encoding.
// See message_types for list of these commands / messages.

use num_traits::ToPrimitive;

use super::message_types::SubcommandType;

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

pub enum PortModeInformationType {
    Name            = 0x00,    // NAME	Name of the mode
    Raw             = 0x01,    // RAW	The raw range
    Pct             = 0x02,    // PCT	The percent range
    Si              = 0x03,    // SI	The SI value range
    Symbol          = 0x04,    // SYMBOL	The standard name of value
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
/********** PortOutputCommand **********/
/***************************************/

pub struct PortOutputCommandParams {
    pub port_id: u8,
    pub start_up_info: StartupAndCompletionInfo,
    pub subcommand_id: SubcommandType,
    pub payload: SubcommandPayload,
}

impl Serialized for PortOutputCommandParams {
    fn serialize(&self) -> Vec<u8> {
        let mut res = vec![self.port_id, self.start_up_info as u8, self.subcommand_id as u8];
        res.append(self.payload.serialize().as_mut());
        res
    }
}

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
    StartSpeed(StartSpeedPayload),
    GotoAbsolutePosition(GotoAbsolutePositionPayload)
}

impl Serialized for SubcommandPayload {
    fn serialize(&self) -> Vec<u8> {
        match self {
            SubcommandPayload::StartSpeed(payload) => {
                payload.serialize()
            },
            SubcommandPayload::GotoAbsolutePosition(payload) => {
                payload.serialize()
            }
        }
    }
}

pub struct StartSpeedPayload {
    pub speed: i8,
    pub max_power: i8,
    pub use_profile: bool,
}

impl Serialized for StartSpeedPayload {
    fn serialize(&self) -> Vec<u8> {
        vec![
            self.speed.to_be_bytes()[0], 
            self.max_power.to_be_bytes()[0], 
            self.use_profile as u8
            ]
    }
}

pub struct GotoAbsolutePositionPayload {
    abs_pos: i32,
    speed: i8,
    max_power: i8,
    end_state: i8,
    use_profile: bool,
}

impl Serialized for GotoAbsolutePositionPayload {
    fn serialize(&self) -> Vec<u8> {
        vec![
            
            ]
    }
}