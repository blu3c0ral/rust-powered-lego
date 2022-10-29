extern crate rust_powered_lego;

#[cfg(test)]
mod tests {
    
    use rust_powered_lego::lego::MessageTypes;
    use rust_powered_lego::lego::CommonMessageHeader;

    #[test]
    fn get_header() {
        let cmh = CommonMessageHeader::new();

        println!("{:?}", cmh.get_header(MessageTypes::HubAlerts.into()));
    }
}