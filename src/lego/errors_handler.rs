use num_traits::FromPrimitive;
use anyhow::{Result, Ok, bail};

use super::MessageTypes;

use crate::lego::consts::LegoErrorTypes;

fn parse_lego_error(msg: &Vec<u8>) -> Result<String> {
    let slc = msg;
    let err_cmd = slc[3];
    let err_code = slc[4];

    let err: Option<LegoErrorTypes> = FromPrimitive::from_u8(err_code);
    let cmd: Option<MessageTypes> = FromPrimitive::from_u8(err_cmd);

    let mut cmd_str: String = "UnknownCommand".to_string();
    cmd.map(|x| { cmd_str = format!("{:?}", x)});

    let mut err_str: String = "UnknownError".to_string();
    err.map(|x| { err_str = format!("{:?}", x)});

    Ok(format!("[Error] On command {}: {}", cmd_str, err_str))
}

pub fn check_for_lego_error(msg: &Vec<u8>) -> Result<()> {
    if msg.len() < 3 {
        bail!("[Error] Not a valid message")
    }
    if msg[2] == 0x05 {
        bail!(parse_lego_error(msg).unwrap())
    }
    Ok(())
}

