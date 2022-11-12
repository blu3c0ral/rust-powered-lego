use std::pin::Pin;

use btleplug::api::{Peripheral as _, Characteristic, Service, WriteType, ValueNotification};
use btleplug::platform::{Peripheral};

use anyhow::{Result, anyhow, Ok};
use num_traits::ToPrimitive;
use tokio_stream::Stream;


use super::check_for_lego_error;
use super::{MessageTypes, message_parameters::Serialized};

pub const MAX_MESSAGE_SIZE: usize = 130;



pub struct CommonMessageHeader {}

impl CommonMessageHeader {
    fn get_header(msg_type: MessageTypes) -> Vec<u8> {
        vec![0x0, 0x0, msg_type as u8]      // [msg_len, 0, mag_type]
    }
}


pub struct Communicator {
    peripheral: Peripheral,
    characteristic: Characteristic,
}

impl Communicator {
    pub async fn new(peripheral: Peripheral) -> Result<Self> {
        peripheral.discover_services().await?;
        
        let srvc: Option<Service>;
        let mut c: Option<Characteristic> = None;
        for service in peripheral.services() {
            srvc = Some(service);
            for characteristic in srvc.unwrap().characteristics {
                c = Some(characteristic);
                break;
            }
            break;
        }
        println!("Characteristics: {:?}", c);
        Ok(Self { peripheral, characteristic: c.unwrap() })
    }

    pub async fn send_message<T>(&self, mt: MessageTypes, mp: T) -> Result<()>
    where
        T: Serialized,
    {
        let write_type = WriteType::WithResponse;

        let mut data = CommonMessageHeader::get_header(mt);
        data.append(mp.serialize().as_mut());
        let size = data.len();
        data[0] = size.to_u8().unwrap();
        if size > 127 {
            data.insert(1, 0x01);
        }

        let res = self.peripheral.write(
            &self.characteristic, 
            &data, 
            write_type).await;

        if res.is_err() {
            Err(anyhow!("Couldn't send the message"))
        } else {
            Ok(())
        }
    }

    pub async fn read_message(&self) -> Result<Vec<u8>> {
        let mut res = self.peripheral.read(&self.characteristic).await?;
        if res.is_empty() {
            res = self.peripheral.read(&self.characteristic).await?;
        }
        check_for_lego_error(&res)?;
        Ok(res)
    }

    pub async fn get_notification_stream(&self) -> Result<Pin<Box<dyn Stream<Item = ValueNotification> + Send>>> {
        self.peripheral.subscribe(&self.characteristic).await?;
        Ok(self.peripheral.notifications().await?)
    }

    // This function is mainly for debugging and testing
    pub async fn get_message_only<T>(&self, mt: MessageTypes, mp: T) -> Result<Vec<u8>>
    where
        T: Serialized,
    {
        let mut data = CommonMessageHeader::get_header(mt);
        data.append(mp.serialize().as_mut());
        let size = data.len();

        // Yes, it is assumed that the maximal length is ok here. The Docs are unclear about different encoding.
        data[0] = size.to_u8().unwrap();
        if size > 127 {
            data.insert(1, 0x01);
        }

        Ok(data)
    }
}