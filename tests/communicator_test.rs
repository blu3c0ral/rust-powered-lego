extern crate rust_powered_lego;

#[cfg(test)]
mod tests {
    
    use std::{
        str::FromStr, 
        time::Duration
    };

    use btleplug::api::BDAddr;
    use rust_powered_lego::{
        connection_manager::ConnectionManager, 
        HubType
    };

    use tokio::time;

    #[tokio::test]
    async fn shut_down_hub_test() {
        let cm = ConnectionManager::new();

        let technic_hub_add = BDAddr::from_str("90:84:2b:4e:5b:96");
        let mut address: Option<BDAddr> = None;
        if let Ok(x) = technic_hub_add {
            address = Some(x);
        }

        let hub = cm.get_hub(None, address, 5).await;

        assert_eq!(hub.is_ok(), true);

        assert_eq!(hub.unwrap().shut_down_hub().await.is_ok(), true);

        time::sleep(Duration::from_secs(5)).await;
    }
}