extern crate rust_powered_lego;

#[cfg(test)]
mod tests {
    
    use rust_powered_lego::lego::RawMessageSlice;
    use rust_powered_lego::lego::MAX_MESSAGE_SIZE;
    use rust_powered_lego::lego::parse_lego_error;

    #[test]
    fn parse_lego_error_test() {
        let mut arr: [u8; MAX_MESSAGE_SIZE] = [0x0; MAX_MESSAGE_SIZE];
        arr[0] = 0x03;
        arr[1] = 0x04;
        let res = parse_lego_error(RawMessageSlice::CommandSpecificMessageSlice(
            arr
        ));

        //assert_eq!(res.is_ok(), true);
        //assert_eq!(res.unwrap(), "[Error] On command HubAlerts: Timeout".to_string());
        println!("{}", res.unwrap());
    }
}