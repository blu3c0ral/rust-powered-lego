// Util functions not directly related to specific device

use anyhow::Ok;
use anyhow::Result;
use anyhow::bail;

use byteorder::{ByteOrder, LittleEndian};



/* Begin: parse_port_info_reply */

// This function parse an info request reply.
// The function make a sanity check and then make sense out of the reply
// into a formatted String.
// (TODO) Parse PortValue and PossibleModeCombinations info replies.
// (TODO) Export some struct out of the reply containing the values.
//        Right now, there is nothing to do with this values...
//
// reply: Vec<u8> is the entire upstream message

// Port Information: [11, 0, 67, 0, 1, 15, 6, 30, 0, 31, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0]

pub fn parse_port_info_reply(reply: Vec<u8>) -> Result<PortInfoReply> {
    /* Sanity check */
    if !((reply[2] == 0x43) || (reply[2] == 0x45) || (reply[2] == 0x46)) {
        bail!("[Error] The submitted reply is not valid reply for port information request")
    }

    match reply[4] {
        0x0 => Ok(
            PortInfoReply::PortInfoValueReplyParsed(PortInfoValueReply {data: reply})
        ),
        0x1 => {
            Ok(PortInfoReply::PortInfoModeReplyParsed(
                PortInfoModeReply {
                    port_id:            reply[3], 
                    info_type:          reply[4], 
                    capabilities:       parse_capabilities(reply[5]), 
                    total_mode_count:   reply[6], 
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

#[derive(Debug)]
pub struct PortInfoValueReply {
    data: Vec<u8>,
}

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

/* End: parse_port_info_reply */