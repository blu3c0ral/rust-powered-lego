extern crate rust_powered_lego;

#[cfg(test)]
mod tests {
    use std::{str::FromStr, time::Duration};
    use tokio::time;

    use btleplug::api::BDAddr;
    use rust_powered_lego::{
        hub::TechnicHubPorts, 
        connection_manager::ConnectionManager, MotorType, HubType, ports::EndState
    };

    #[tokio::test]
    async fn start_speed_test() {
        let port_id = TechnicHubPorts::A;

        let cm = ConnectionManager::new();

        let technic_hub_add = BDAddr::from_str("90:84:2b:56:8e:77");
        let technic_hub_add2 = BDAddr::from_str("90:84:2b:4e:5b:96");
        let hub_no_4 = BDAddr::from_str("90:84:2b:c7:ed:8d");
        let mut address: Option<BDAddr> = None;
        if let Ok(x) = technic_hub_add2 {
            address = Some(x);
        }

        let res = cm.get_technic_hub(None, address).await;

        assert_eq!(res.is_ok(), true);

        let hub = res.unwrap();

        let res = hub.get_motor(port_id).await;

        if res.is_err() {
            println!("{:?}", res.as_ref().err())
        }

        let motor = res.unwrap();

        let res = motor.start_speed(100, 100, false, true).await;

        if res.is_err() {
            println!("[Error] {:?}", res.err());
        } else {
            println!("[TrainMotor] {:?}", res.unwrap());
        }

        time::sleep(Duration::from_secs(3)).await;

        let res = motor.start_speed(0, 0, false, false).await;
        time::sleep(Duration::from_secs(1)).await;

        if res.is_err() {
            println!("[Error] {:?}", res.err());
        }

        let res = motor.start_speed(-100, 100, false, false).await;

        if res.is_err() {
            println!("[Error] {:?}", res.err());
        }

        time::sleep(Duration::from_secs(3)).await;

        let res = motor.start_speed(0, 0, true, false).await;

        if res.is_err() {
            println!("[Error] {:?}", res.err());
        }
        
    }

    #[tokio::test]
    async fn go_to_abs_pos_test() {
        let port_id = TechnicHubPorts::A;

        let cm = ConnectionManager::new();

        let technic_hub_add = BDAddr::from_str("90:84:2b:56:8e:77");
        let technic_hub_add2 = BDAddr::from_str("90:84:2b:4e:5b:96");
        let hub_no_4 = BDAddr::from_str("90:84:2b:c7:ed:8d");
        let mut address: Option<BDAddr> = None;
        if let Ok(x) = technic_hub_add2 {
            address = Some(x);
        }

        let res = cm.get_technic_hub(None, address).await;

        assert_eq!(res.is_ok(), true);

        let hub = res.unwrap();

        let res = hub.get_motor(port_id).await;

        if res.is_err() {
            println!("{:?}", res.as_ref().err())
        }

        let motor = res.unwrap();

        let res = motor.go_to_aps_position(
            3,
            5,
            100,
            EndState::HOLD,
            true,
            true
        ).await;

        if res.is_err() {
            println!("[Error] {:?}", res.err());
        } else {
            println!("[Motor Response] {:?}", res.unwrap());
        }

        time::sleep(Duration::from_secs(10)).await;

        let res = motor.go_to_abs_position(
            0,
            5,
            100,
            EndState::HOLD,
            true,
            true
        ).await;

        if res.is_err() {
            println!("[Error] {:?}", res.err());
        } else {
            println!("[Motor Response] {:?}", res.unwrap());
        }

    }

    #[tokio::test]
    async fn start_speed_for_deg_test() {
        let port_id = TechnicHubPorts::A;

        let cm = ConnectionManager::new();

        let technic_hub_add = BDAddr::from_str("90:84:2b:56:8e:77");
        let technic_hub_add2 = BDAddr::from_str("90:84:2b:4e:5b:96");
        let hub_no_4 = BDAddr::from_str("90:84:2b:c7:ed:8d");
        let mut address: Option<BDAddr> = None;
        if let Ok(x) = technic_hub_add2 {
            address = Some(x);
        }

        let res = cm.get_technic_hub(None, address).await;

        assert_eq!(res.is_ok(), true);

        let hub = res.unwrap();

        let res = hub.get_motor(port_id).await;

        if res.is_err() {
            println!("{:?}", res.as_ref().err())
        }

        let motor = res.unwrap();

        let res = motor.start_speed_for_deg(
            180,
            50,
            25,
            EndState::HOLD,
            false,
            true
        ).await;

        if res.is_err() {
            println!("[Error] {:?}", res.err());
        } else {
            println!("[Motor Response] {:?}", res.unwrap());
        }

        time::sleep(Duration::from_secs(10)).await;

        let res = motor.start_speed_for_deg(
            180,
            -50,
            25,
            EndState::HOLD,
            false,
            true
        ).await;

        if res.is_err() {
            println!("[Error] {:?}", res.err());
        } else {
            println!("[Motor Response] {:?}", res.unwrap());
        }

    }

}