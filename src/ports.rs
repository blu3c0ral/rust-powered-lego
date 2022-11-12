// Dealing with all ports types and actions
use anyhow::{Result, Ok};
use async_trait::async_trait;
use num_derive::FromPrimitive;


use crate::{
    hub::Hub, 
    lego::{
        message_parameters::{
            StartupAndCompletionInfo, 
            SubcommandPayload, 
            PortOutputCommandParams,
            SetAccTimePayload,
            SetDecTimePayload,
            StartSpeedPayload,
            StartSpeedForDegreesPayload,
            GotoAbsolutePositionPayload,
            WriteDirectModeDataPayload,
            SetAbsolutePositionPayload,
            WriteDirectModeDataCommands,
        }, 
        SubcommandType
    }, MotorType, HubType
};



/* Below consts are taken from https://github.com/corneliusmunz/legoino/blob/master/src/Lpf2HubConst.h */
#[derive(Debug, FromPrimitive, PartialEq)]
pub enum PortType {
    UnknownDevice                       = 0,
    SimpleMediumLinearMotor             = 1,
    TrainMotor                          = 2,
    Light                               = 8,
    VoltageSensor                       = 20,
    CurrentSensor                       = 21,
    PiezoBuzzer                         = 22,
    HubLed                              = 23,
    TiltSensor                          = 34,
    MotionSensor                        = 35,
    ColorDistanceSensor                 = 37,
    MediumLinearMotor                   = 38,
    MoveHubMediumLinearMotor            = 39,
    MoveHubTiltSensor                   = 40,
    DuploTrainBaseMotor                 = 41,
    DuploTrainBaseSpeaker               = 42,
    DuploTrainBaseColorSensor           = 43,
    DuploTrainBaseSpeedometer           = 44,
    TechnicLargeLinearMotor             = 46,   // Technic Control+
    TechnicXlargeLinearMotor            = 47,   // Technic Control+
    TechnicMediumAngularMotor           = 48,   // Spike Prime
    TechnicLargeAngularMotor            = 49,   // Spike Prime
    TechnicMediumHubGestSensor          = 54,
    RemoteControlButton                 = 55,
    RemoteControlRssi                   = 56,
    TechnicMediumHubAccelerometer       = 57,
    TechnicMediumHubGyroSensor          = 58,
    TechnicMediumHubTiltSensor          = 59,
    TechnicMediumHubTemperatureSensor   = 60,
    TechnicColorSensor                  = 61,   // Spike Prime
    TechnicDistanceSensor               = 62,   // Spike Prime
    TechnicForceSensor                  = 63,   // Spike Prime
    MarioHubGestureSensor               = 71,   // https://github.com/bricklife/LEGO-Mario-Reveng
    MarioHubBarcodeSensor               = 73,   // https://github.com/bricklife/LEGO-Mario-Reveng
    MarioHubPantSensor                  = 74,   // https://github.com/bricklife/LEGO-Mario-Reveng
    TechnicMediumAngularMotorGrey       = 75,   // Mindstorms
    TechnicLargeAngularMotorGrey        = 76    // Mindstorms
}

pub const MOTOR_TYPES: [PortType; 3] = [
    PortType::TechnicLargeLinearMotor,
    PortType::TechnicXlargeLinearMotor,
    PortType::TrainMotor,
];



pub enum Profile {
    Acc     = 0x01,     // 0b 0000 0001
    Dec     = 0x02,     // 0b 0000 0010
    AccDec  = 0x03,     // 0b 0000 0011
}


pub enum EndState {
    FLOAT   = 0x00, // Another word for an inactive port. I.e. NO power power supplied to a motor (high impedance).
    HOLD    = 0x7e, // = 126. When the motor is stopped (no rotation/movement), but the driver continues to keep the current position by actively.
    BRAKE   = 0x7f, // = 127. When the motor is shorted through the motordriver.
}

pub struct Motor<'a> {
    pub hub:        &'a Hub,
    pub port_id:    u8,
}

impl<'a> Motor<'a> {
    pub fn new(hub: &'a Hub, port_id: u8) -> Result<Self> {
        Ok(
            Self {
                hub,
                port_id,
            }
        )
    }

    fn get_ouput_command_params(
        &self,
        subcommand_id: SubcommandType,
        payload: SubcommandPayload,
        start_up_info: StartupAndCompletionInfo
    ) -> PortOutputCommandParams {
        PortOutputCommandParams {
            port_id: self.port_id,
            start_up_info: start_up_info,
            subcommand_id: subcommand_id,
            payload: payload

        }
    }
}

#[async_trait]
impl<'a> MotorType for Motor<'a> {

    async fn set_acceleration_time(
        &self, 
        time: i16,
        start_up_info: StartupAndCompletionInfo,
    ) -> Result<Vec<u8>> {
        self.hub.send_output_command(
            self.get_ouput_command_params(
                SubcommandType::SetAccTime,
                SubcommandPayload::SetAccTime(
                    SetAccTimePayload {
                        time,
                    }
                ),
                start_up_info
        )).await
    }

    async fn set_deceleration_time(
        &self,
        time: i16,
        start_up_info: StartupAndCompletionInfo,
    ) -> Result<Vec<u8>> {
        self.hub.send_output_command(
            self.get_ouput_command_params(
                SubcommandType::SetDecTime,
                SubcommandPayload::SetDecTime(
                    SetDecTimePayload {
                        time,
                    }
                ),
                start_up_info
        )).await
    }

    async fn start_speed(
        &self, 
        speed: i8, 
        max_power: i8, 
        use_profile: Profile, 
        start_up_info: StartupAndCompletionInfo
    ) -> Result<Vec<u8>> {
        self.hub.send_output_command(
            self.get_ouput_command_params(
                SubcommandType::StartSpeed,
                SubcommandPayload::StartSpeed(
                    StartSpeedPayload {
                        speed,
                        max_power,
                        use_profile
                    }
                ),
                start_up_info
        )).await
    }

    async fn stop_motor(
        &self, 
        use_profile: Profile, 
        start_up_info: StartupAndCompletionInfo
    ) -> Result<Vec<u8>> {
        self.start_speed(0, 0, use_profile, start_up_info).await
    }

    async fn set_abs_position(
        &self, 
        position: i32, 
        start_up_info: StartupAndCompletionInfo
    ) -> Result<Vec<u8>> {
        self.hub.send_output_command(
            self.get_ouput_command_params(
                SubcommandType::WriteDirectModeData,
                SubcommandPayload::WriteDirectModeData(
                    WriteDirectModeDataPayload {
                        mode : MotorModes::Pos as u8,
                        payload: WriteDirectModeDataCommands::SetAbsolutePosition(
                            SetAbsolutePositionPayload {
                                position
                            },
                        )
                    }
                ),
                start_up_info
            )
        ).await
    }

    async fn go_to_abs_position(
        &self, 
        abs_pos: i32,
        speed: i8,
        max_power: i8,
        end_state: EndState,
        use_profile: Profile,
        start_up_info: StartupAndCompletionInfo
    ) -> Result<Vec<u8>> {
        self.hub.send_output_command(
            self.get_ouput_command_params(
                SubcommandType::GotoAbsolutePosition,
                SubcommandPayload::GotoAbsolutePosition(
                    GotoAbsolutePositionPayload {
                        abs_pos,
                        speed,
                        max_power,
                        end_state,
                        use_profile,
                    }
                ),
                start_up_info
        )).await
    }

    async fn start_speed_for_deg (
        &self, 
        degrees: i32,
        speed: i8,
        max_power: i8,
        end_state: EndState,
        use_profile: Profile,
        start_up_info: StartupAndCompletionInfo
    ) -> Result<Vec<u8>> {
        self.hub.send_output_command(
            self.get_ouput_command_params(
                SubcommandType::StartSpeedForDegrees,
                SubcommandPayload::StartSpeedForDegrees(
                    StartSpeedForDegreesPayload {
                        degrees,
                        speed,
                        max_power,
                        end_state,
                        use_profile,
                    }
                ),
                start_up_info
        )).await
    }

}


// Below values are empirical. No official documentation has been found.
pub enum MotorModes {
    Power   = 0x00,
    Speed   = 0x01,
    Pos     = 0x02,
    Apos    = 0x03,
    Load    = 0x04,
    Calib   = 0x05,
}