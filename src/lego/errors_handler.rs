use std::io::Error;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use super::MessageTypes;

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

pub fn parse_lego_error(msg: Vec<u8>) -> Result<String, Error> {
    let slc = msg;
    let err_cmd = slc[0];
    let err_code = slc[1];

    let err: Option<LegoErrorTypes> = FromPrimitive::from_u8(err_code);
    let cmd: Option<MessageTypes> = FromPrimitive::from_u8(err_cmd);

    let mut cmd_str: String = "UnknownCommand".to_string();
    cmd.map(|x| { cmd_str = format!("{:?}", x)});

    let mut err_str: String = "UnknownError".to_string();
    err.map(|x| { err_str = format!("{:?}", x)});

    Ok(format!("[Error] On command {}: {}", cmd_str, err_str))
}

