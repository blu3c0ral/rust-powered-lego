extern crate rust_powered_lego;

#[cfg(test)]
mod tests {
    
    use std::{str::{FromStr, from_utf8_unchecked, from_utf8}, time::Duration};

    use btleplug::api::BDAddr;
    use rust_powered_lego::{connection_manager::ConnectionManager, 
        lego::{
            message_parameters,
        },
        hub::TechnicHubPorts
    };

    use tokio::time;

    #[tokio::test]
    async fn shut_down_hub_test() {
        let cm = ConnectionManager::new();

        let technic_hub_add = BDAddr::from_str("90:84:2b:56:8e:77");
        let technic_hub_add2 = BDAddr::from_str("90:84:2b:4e:5b:96");
        let hub_no_4 = BDAddr::from_str("90:84:2b:c7:ed:8d");
        let mut address: Option<BDAddr> = None;
        if let Ok(x) = technic_hub_add2 {
            address = Some(x);
        }

        let hub = cm.get_technic_hub(None, address).await;

        assert_eq!(hub.is_ok(), true);

        println!("Shutting down hub");
        hub.unwrap().shut_down_hub();

        time::sleep(Duration::from_secs(10)).await;
    }

    #[tokio::test]
    async fn get_port_information() {
        let port_id = TechnicHubPorts::A;

        let cm = ConnectionManager::new();

        let technic_hub_add = BDAddr::from_str("90:84:2b:56:8e:77");
        let technic_hub_add2 = BDAddr::from_str("90:84:2b:4e:5b:96");
        let hub_no_4 = BDAddr::from_str("90:84:2b:c7:ed:8d");
        let mut address: Option<BDAddr> = None;
        if let Ok(x) = technic_hub_add2 {
            address = Some(x);
        }

        let hub = cm.get_technic_hub(None, address).await;

        assert_eq!(hub.is_ok(), true);

        let msg = hub.unwrap().get_port_information(
            port_id, 
            message_parameters::PortInformationType::PortValue
        ).await;
        
        
        match msg {
            Ok(_) => {
                println!("Port Information: {:?}", msg.unwrap());
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    #[tokio::test]
    async fn get_mode_information_test() {
        let port_id = TechnicHubPorts::A;

        let cm = ConnectionManager::new();

        let technic_hub_add = BDAddr::from_str("90:84:2b:56:8e:77");
        let technic_hub_add2 = BDAddr::from_str("90:84:2b:4e:5b:96");
        let hub_no_4 = BDAddr::from_str("90:84:2b:c7:ed:8d");
        let mut address: Option<BDAddr> = None;
        if let Ok(x) = technic_hub_add2 {
            address = Some(x);
        }

        let hub = cm.get_technic_hub(None, address).await;

        assert_eq!(hub.is_ok(), true);

        for i in 0..6 {
            let msg = hub.as_ref().unwrap().get_mode_information(
                port_id, 
                i, 
                message_parameters::PortModeInformationType::Name
            ).await;
    
            match msg {
                Ok(_) => {
                    //println!("Mode Information: {:?}", &msg.as_ref().unwrap());
                    println!("Name: {}", from_utf8(&msg.as_ref().unwrap()[5..]).unwrap());
                },
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        

    }
}