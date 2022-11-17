use num_derive::FromPrimitive;


//
// Parameters structs for (some of the) MessageTypes is located in message_parameters.rs file
//
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum MessageTypes {                             //  Comm.	    Rply2   Notes
    HubProperties                       = 0x01,     //  Down + Up   0x01    Set or retrieve standard Hub Property information
    HubActions                          = 0x02,     //  Down + Up	0x02	Perform actions on connected hub
    HubAlerts                           = 0x03,     //  Down + Up	0x03	Subscribe or retrieve Hub alerts
    HubAttachedIO                       = 0x04,     //  Up	        N/A	    Transmitted upon Hub detection of attached I/O
    GenericErrorMessages                = 0x05,     //  Up	        N/A	    Generic Error Messages from the Hub
    HWNetWorkCommands                   = 0x08,     //  Down + Up	0x08	Commands used for H/W Networks
    FWUpdateGoIntoBootMode              = 0x10,     //  Down	    N/A	    Set the Hub in a special Boot Loader mode
    FWUpdateLockMemory                  = 0x11,     //  Down	    N/A	    Locks the memory
    FWUpdateLockStatusRequest           = 0x12,     //  Down	    N/A	    Request the Memory Locking State
    FWLockStatus                        = 0x13,     //  Up	        0x12	Answer to the F/W Lock Status Request
    
    PortInformationRequest              = 0x21,     //	Down    N/A	    Request Port information
    PortModeInformationRequest          = 0x22,     //	Down	N/A	    Request Port Mode information
    PortInputFormatSetupSingle          = 0x41,     //	Down	N/A	    Setup input format for single mode
    PortInputFormatSetupCombinedMode    = 0x42,     //	Down	 	    Setup input format for multiple modes (CombinedMode)
    PortInformation                     = 0x43,     //	Up	    0x21	N/A
    PortModeInformation                 = 0x44,     //	Up	    0x22	N/A
    PortValueSingle                     = 0x45,     //	Up	    0x21	Value update related to single Port Mode
    PortValueCombinedMode               = 0x46,     //	Up	    0x21	Value update related to multiple Port Modes in combination (CombinedMode)
    PortInputFormatSingle               = 0x47,     //	Up	    0x41	N/A
    PortInputFormatCombinedMode         = 0x48,     //	Up	    0x42	N/A
    VirtualPortSetup                    = 0x61,     //	Down	N/A	    Used to control synchronization between synchronizable ports.
    PortOutputCommand                   = 0x81,     //	Down	N/A	    Used to execute Port Output commands
    PortOutputCommandFeedback           = 0x82,     //	Up	    0x81	Provides feedback on completed Port Output commands
}




/***************************************/
/*********** Subcommand Types **********/
/***************************************/

//
// Parameters structs for (some of the) SubcommandType is located in message_parameters.rs file
//
#[derive(Clone, Copy)]
pub enum SubcommandType {
    StartPowerSync              = 0x02,
    SetAccTime                  = 0x05,
    SetDecTime                  = 0x06,
    StartSpeed                  = 0x07,
    StartSpeedSync              = 0x08,
    StartSpeedForTime           = 0x09,
    StartSpeedForTimeSync       = 0x0a,
    StartSpeedForDegrees        = 0x0b,
    StartSpeedForDegreesSync    = 0x0c,
    GotoAbsolutePosition        = 0x0d,
    GotoAbsolutePositionSync    = 0x0e,

    WriteDirectModeData         = 0x51,

    // (TODO) This list is not complete
}