use num_derive::FromPrimitive;



/***************************************/
/********* Hub Related Consts **********/
/***************************************/

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



/***************************************/
/********* Port Related Consts *********/
/***************************************/

/* Below consts are taken from https://github.com/corneliusmunz/legoino/blob/master/src/Lpf2HubConst.h */
#[derive(Debug, FromPrimitive, PartialEq, Clone, Copy)]
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

#[derive(Clone, Copy)]
pub enum Profile {
    Acc     = 0x01,     // 0b 0000 0001
    Dec     = 0x02,     // 0b 0000 0010
    AccDec  = 0x03,     // 0b 0000 0011
}

#[derive(Clone, Copy)]
pub enum EndState {
    FLOAT   = 0x00, // Another word for an inactive port. I.e. NO power power supplied to a motor (high impedance).
    HOLD    = 0x7e, // = 126. When the motor is stopped (no rotation/movement), but the driver continues to keep the current position by actively.
    BRAKE   = 0x7f, // = 127. When the motor is shorted through the motordriver.
}

#[derive(Clone, Copy)]
// Below values are empirical. No official documentation has been found.
pub enum MotorModes {
    Power   = 0x00,
    Speed   = 0x01,
    Pos     = 0x02,
    Apos    = 0x03,
    Load    = 0x04,
    Calib   = 0x05,
}

#[derive(Debug, Clone, Copy)]
pub enum PortInfoModeReplyCapabilities {
    Output                  = 0x0,  // Output (seen from Hub)
    Input                   = 0x1,  // Input (seen from Hub)
    LogicalCombinable       = 0x2,  // Logical Combinable
    LogicalSynchronizable   = 0x3,  // Logical Synchronizable
}



/***************************************/
/*************** Generl ****************/
/***************************************/

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum LegoErrorTypes {
    Ack                     = 0x01,     //  ACK,
    Mack                    = 0x02,     //  MACK
    BufferOverflow          = 0x03,     //  Buffer Overflow
    Timeout                 = 0x04,     //  Timeout
    CommandNotRecognized    = 0x05,     //  Command NOT recognized
    InvalidUse              = 0x06,     //  Invalid use (e.g. parameter error(s)
    Overcurrent             = 0x07,     //  Overcurrent
    InternalError           = 0x08,     //  Internal ERROR
}


/* Below Color consts are taken from https://github.com/corneliusmunz/legoino/blob/master/src/Lpf2HubConst.h */
#[derive(Clone, Copy)]
pub enum Color {
    Black       = 0,
    Pink        = 1,
    Purple      = 2,
    Blue        = 3,
    LightBlue   = 4,
    Cyan        = 5,
    Green       = 6,
    Yellow      = 7,
    Orange      = 8,
    Red         = 9,
    White       = 10,
    None        = 255
}