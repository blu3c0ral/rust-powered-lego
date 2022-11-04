extern crate rust_powered_lego;

#[cfg(test)]
mod tests {
    use std::{str::FromStr, time::Duration};
    use num_traits::ToPrimitive;
    use tokio::time;

    use btleplug::api::BDAddr;
    use rust_powered_lego::{
        hub::TechnicHubPorts, 
        connection_manager::ConnectionManager
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

        let res = hub.get_motor(port_id as u8).await;

        if res.is_err() {
            println!("{:?}", res.as_ref().err())
        }

        let motor = res.unwrap();

        //let n: i8 = -100;
        //println!("n is: {:?}", n.to_be_bytes());

        let res = motor.start_speed(100, 100, true, false).await;

        if res.is_err() {
            println!("[Error] {:?}", res.err());
        }

        time::sleep(Duration::from_secs(3)).await;

        let res = motor.start_speed(0, 0, true, false).await;

        if res.is_err() {
            println!("[Error] {:?}", res.err());
        }

        let res = motor.start_speed(-100, 100, true, false).await;

        if res.is_err() {
            println!("[Error] {:?}", res.err());
        }

        time::sleep(Duration::from_secs(3)).await;

        let res = motor.start_speed(0, 0, true, false).await;

        if res.is_err() {
            println!("[Error] {:?}", res.err());
        }
        
    }
}