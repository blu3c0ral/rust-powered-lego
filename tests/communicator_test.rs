extern crate rust_powered_lego;

#[cfg(test)]
mod tests {
    
    use std::{str::FromStr, time::Duration};

    use btleplug::api::BDAddr;
    use rust_powered_lego::{connection_manager::ConnectionManager, lego::message_parameters, utils::parse_port_info_reply};

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
        let port_id = 0;

        let cm = ConnectionManager::new();

        let technic_hub_add = BDAddr::from_str("90:84:2b:56:8e:77");
        let technic_hub_add2 = BDAddr::from_str("90:84:2b:4e:5b:96");
        let hub_no_4 = BDAddr::from_str("90:84:2b:c7:ed:8d");
        let mut address: Option<BDAddr> = None;
        if let Ok(x) = hub_no_4 {
            address = Some(x);
        }

        let hub = cm.get_technic_hub(None, address).await;

        assert_eq!(hub.is_ok(), true);

        let msg = hub.unwrap().get_port_information(
            port_id, 
            message_parameters::PortInformationType::ModeInfo
        ).await;

        println!("Port Information: {:?}", parse_port_info_reply(msg.unwrap()));
    }
}


// [11, 0, 67, 0, 1, 15, 6, 30, 0, 31, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0]