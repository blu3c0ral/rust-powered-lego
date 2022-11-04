// Dealing with all ports types and actions
use anyhow::Result;
use num_derive::FromPrimitive;



use crate::{
    hub::Hub, 
    lego::{
        message_parameters::{
            StartupAndCompletionInfo, 
            SubcommandPayload, 
            PortOutputCommandParams, 
            StartSpeedPayload
        }, 
        SubcommandType
    }
};

/* Below consts are taken from https://github.com/corneliusmunz/legoino/blob/master/src/Lpf2HubConst.h */
#[derive(Debug, FromPrimitive)]
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
        feedback: bool
    ) -> PortOutputCommandParams {
        PortOutputCommandParams {
            port_id: self.port_id,
            start_up_info: 
                if feedback 
                    {StartupAndCompletionInfo::BufferAndFeedback} 
                else 
                    {StartupAndCompletionInfo::BufferAndNoAction},
            subcommand_id: subcommand_id,
            payload: payload

        }
    }

    pub async fn start_speed(
        &self, 
        speed: i8, 
        max_power: i8, 
        use_profile: bool, 
        feedback: bool
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
                feedback
        )).await
    }

    pub async fn stop_motor(&self, use_profile: bool, feedback: bool) -> Result<Vec<u8>> {
        self.start_speed(0, 100, use_profile, feedback).await
    }

    

}


