use btleplug::platform::Peripheral;

pub enum HubTypes {
    TechnicHub,         // # item: 88012
    HubHub,             // # item: 88009
}

enum HubTypesSystemId {
    TechnicHubSystemId  = 0b1000000,
    HubHubSystemId      = 0b1000001,
}


struct Hub {
    peripheral: Peripheral,
}


impl Hub {
    pub fn new(p: Peripheral) -> Self {
        Self { peripheral: p }
    }

    pub fn disconnect(&self) -> bool {
        true
    }
}