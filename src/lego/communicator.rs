use btleplug::api::{Peripheral as _, Characteristic, Service, WriteType};
use btleplug::platform::{Peripheral};


use super::{MessageTypes, message_parameters::Serialized};

pub const MAX_MESSAGE_SIZE: usize = 130;

#[derive(Clone, Debug)]
pub enum RawMessageSlice {
    CommonMessageHeaderSlice(Vec<u8>),
    CommandSpecificMessageSlice(Vec<u8>),
}



pub struct CommonMessageHeader {}

impl CommonMessageHeader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_header(&self, msg_type: MessageTypes) -> RawMessageSlice {
        RawMessageSlice::CommonMessageHeaderSlice(vec![0x0, 0x0, msg_type as u8])
    }
}




pub struct Communicator<'a> {
    peripheral: &'a Peripheral,
    characteristic: Characteristic,
}

impl<'a> Communicator<'a> {
    pub async fn new(peripheral: &'a Peripheral) -> Communicator<'a> {
        peripheral.discover_services().await;
        let pr = peripheral.properties().await.unwrap();
        
        let mut srvc: Option<Service> = None;
        let mut c: Option<Characteristic> = None;
        for service in peripheral.services() {
            srvc = Some(service);
            for characteristic in srvc.unwrap().characteristics {
                c = Some(characteristic);
                break;
            }
            break;
        }

        Self { peripheral, characteristic: c.unwrap() }
    }

    pub async fn send_message<T>(&self, mp: T) -> Result<(), btleplug::Error>
    where
        T: Serialized,
    {
        let write_type = WriteType::WithResponse;
        self.peripheral.write(&self.characteristic, &mp.serialize(), write_type).await
    }
}