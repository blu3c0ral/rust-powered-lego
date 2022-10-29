extern crate rust_powered_lego;

use uuid::{uuid, Uuid};

//use std::string::ToString;
//use std::str::FromStr;

//use btleplug::api::BDAddr;

#[cfg(test)]
mod tests {
    use std::{str::FromStr, collections::BTreeSet};
    use uuid::Uuid;
    use std::time::Duration;

    use tokio::time;


    use btleplug::api::{BDAddr, WriteType, Peripheral, Characteristic, CharPropFlags, Service, ValueNotification, PeripheralProperties};

    
    #[tokio::test]
    async fn get_peripheral_test() {
        let technic_hub_add = BDAddr::from_str("90:84:2b:56:8e:77");
        let technic_hub_add2 = BDAddr::from_str("90:84:2b:4e:5b:96");
        let hub_no_4 = BDAddr::from_str("90:84:2b:c7:ed:8d");
        let mut address: Option<BDAddr> = None;
        if let Ok(x) = hub_no_4 {
            address = Some(x);
        }
        let mngr = rust_powered_lego::connection_manager::ConnectionManager::new();

        let p = mngr.get_peripheral(
            None,
            address).await;
        
        let write_type = WriteType::WithResponse;
        let msg:[u8; 5] = [0x0, 0x0, 0x01, 0x0b, 0x05];
        let mut srvc: Service = Service { uuid: Uuid::parse_str("00001624-1212-efde-1623-785feabcd123").unwrap(), primary: true, characteristics: BTreeSet::new() };
        let mut c: &Characteristic = &Characteristic { uuid: Uuid::parse_str("00001624-1212-efde-1623-785feabcd123").unwrap(), service_uuid: Uuid::parse_str("00001624-1212-efde-1623-785feabcd123").unwrap(), properties: CharPropFlags::WRITE_WITHOUT_RESPONSE };
        if p.is_ok(){
            p.as_ref().unwrap().discover_services().await;

            let pr = p.as_ref().unwrap().properties().await.unwrap();
            if let Some(k) = pr {
                println!("Man. data: {:?}", k.manufacturer_data);
            }
            for service in p.as_ref().unwrap().services() {
                srvc = service;
                println!(
                    "Service UUID {}, primary: {}",
                    &srvc.uuid, &srvc.primary
                );
                for characteristic in &srvc.characteristics {
                    c = characteristic;
                    println!("  {:?}", characteristic);
                    break;
                }
                break;
            }

            println!("Sending..!");
            let ch = &Characteristic { 
                uuid: c.uuid,//Uuid::parse_str("00001624-1212-efde-1623-785feabcd123").unwrap(), 
                service_uuid: c.service_uuid, //Uuid::parse_str("00001623-1212-efde-1623-785feabcd123").unwrap(),
                                                      
                properties: CharPropFlags::WRITE | CharPropFlags::NOTIFY,
            };
            let res = p.as_ref().unwrap().write(ch,&msg, write_type).await;

            if res.is_err() {
                //println!("{:?}", res);
            }

            let res2 = p.as_ref().unwrap().read(ch).await;
            time::sleep(Duration::from_secs(5)).await;

            if !res2.is_err() {
                println!("{:?}", res2.as_ref().unwrap());
                let i = res2.unwrap()[5];
                println!("{:b}", i);
            }


        }
        
        
        //assert_eq!(true, p.is_ok());

    }
}