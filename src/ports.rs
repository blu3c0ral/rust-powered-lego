// Dealing with all ports types and actions
use anyhow::{Result, Ok};
use async_trait::async_trait;


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
            StartPowerPayload,
        }, 
        SubcommandType, 
        consts::{
            PortType,
            Profile,
            MotorModes,
            EndState,
        },
    }, MotorType, HubType
};


pub const MOTOR_TYPES: [PortType; 3] = [
    PortType::TechnicLargeLinearMotor,
    PortType::TechnicXlargeLinearMotor,
    PortType::TrainMotor,
];


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

    // wdm = write direct mode
    fn get_wdm_ouput_command_params(
        &self,
        mode : MotorModes,
        payload: WriteDirectModeDataCommands,
        start_up_info: StartupAndCompletionInfo
    ) -> PortOutputCommandParams {
        self.get_ouput_command_params(
            SubcommandType::WriteDirectModeData,
            SubcommandPayload::WriteDirectModeData(
                WriteDirectModeDataPayload {
                    mode : mode as u8,
                    payload: payload,
                    
                }
            ),
            start_up_info
        )
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

    async fn start_power(
        &self, 
        power: i8, 
        start_up_info: StartupAndCompletionInfo
    ) -> Result<Vec<u8>> {        
        self.hub.send_output_command(
            self.get_wdm_ouput_command_params(
                MotorModes::Power, 
                WriteDirectModeDataCommands::StartPower(
                    StartPowerPayload {
                        power,
                    },
                ), 
                start_up_info
            )
        ).await
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
        end_state: EndState,
        use_profile: Profile, 
        start_up_info: StartupAndCompletionInfo
    ) -> Result<Vec<u8>> {
        // Below distinction is dictated by the Docs
        match end_state {
            EndState::HOLD => self.start_speed(0, 0, use_profile, start_up_info).await,
            _ => {
                self.start_power(end_state as i8, start_up_info).await
            }
        }
    }

    async fn set_abs_position(
        &self, 
        position: i32, 
        start_up_info: StartupAndCompletionInfo
    ) -> Result<Vec<u8>> {        
        self.hub.send_output_command(
            self.get_wdm_ouput_command_params(
                MotorModes::Pos, 
                WriteDirectModeDataCommands::SetAbsolutePosition(
                    SetAbsolutePositionPayload {
                        position
                    },
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