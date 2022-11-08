use std::time::Duration;
use std::str::FromStr;
use std::collections::HashMap;

use tokio::time;

use btleplug::api::{BDAddr, Manager as _, Central, ScanFilter, Peripheral as _};
use btleplug::platform::{Manager, Peripheral};

use anyhow::{Result, anyhow};

use crate::hub::Hub;

struct PeripheralInfo {
    address: BDAddr,
    local_name: String,
    // manufacturer_data: HashMap<u16, Vec<u8>>
}

pub struct ConnectionManager {}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_technic_hub(&self, peripheral_name: Option<String>, bd_add: Option<BDAddr>) -> Result<Hub> {
        let p = self.get_peripheral(peripheral_name, bd_add).await?;
        Hub::new(p).await
    }

    async fn get_peripheral(&self, mut peripheral_name: Option<String>, mut bd_add: Option<BDAddr>) -> Result<Peripheral> {
        if bd_add.is_none() {
            if let Ok(x) = BDAddr::from_str("00:00:00:00:00:00") {
                bd_add = Some(x);
            }
        }
        if peripheral_name.is_none() {
            peripheral_name = Some(String::from_str("")?);
        }
        let peripherals = self.get_peripherals().await?;
        println!("{}", peripherals.len());
        let mut peripheral_info: PeripheralInfo;
        if !peripherals.is_empty() {
            
            // All peripheral devices in range
            for peripheral in peripherals.into_iter() {
                peripheral_info = self.get_peripheral_info(&peripheral).await?;
                println!("{}", peripheral_info.local_name);
                if peripheral_info.local_name.eq(peripheral_name.as_ref().unwrap()) || peripheral_info.address.eq(bd_add.as_ref().unwrap()) {
                    let is_connected = peripheral.is_connected().await?;
                    //while peripheral_info.manufacturer_data.is_empty() {
                    //    peripheral_info = self.get_peripheral_info(&peripheral).await?;
                    //}
                    if !is_connected {
                        println!("Connecting to peripheral {:?}...", &peripheral_info.local_name);
                        if let Err(err) = peripheral.connect().await {
                            eprintln!("Error connecting to peripheral, skipping: {}", err);
                            continue;
                        }
                    }
                    if peripheral.is_connected().await? {
                        return Ok(peripheral);
                    }
                }
            }
        }
        Err(anyhow!("No connections found"))
    }

    async fn get_peripherals(&self) -> Result<Vec<Peripheral>> {
        let manager = Manager::new().await?;
        let adapter_list = manager.adapters().await?;
        if adapter_list.is_empty() {
            eprintln!("No Bluetooth adapters found");
        }

        let mut peripherals: Vec<Peripheral> = Vec::new();

        for adapter in adapter_list.iter() {
            println!("Starting scan on {}...", adapter.adapter_info().await?);
            adapter
                .start_scan(ScanFilter::default())
                .await
                .expect("Can't scan BLE adapter for connected devices...");
            time::sleep(Duration::from_secs(10)).await;
            peripherals.append(&mut adapter.peripherals().await?);
        }
        Ok(peripherals)
    }

    async fn get_peripheral_info(&self, peripheral: &Peripheral) -> Result<PeripheralInfo> {
        let properties = peripheral.properties().await?;
        let def_str = String::from("(peripheral name unknown)");
        let local_name = properties.as_ref().unwrap()
            .local_name.as_ref()
            .unwrap_or(&def_str);
        let address = properties.as_ref()
            .unwrap()
            .address;
        let mut manufacturer_data: HashMap<u16, Vec<u8>> = HashMap::new();
        manufacturer_data.clone_from(&properties.as_ref()
        .unwrap()
        .manufacturer_data);
        
        Ok(PeripheralInfo { 
            address: address, 
            local_name: local_name.to_string(), 
            // manufacturer_data: manufacturer_data,
    })

    }
}

