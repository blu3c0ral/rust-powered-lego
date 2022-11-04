extern crate rust_powered_lego;

#[cfg(test)]
mod tests {
    //use rust_powered_lego::utils::parse_port_info_reply;

    #[test]
    fn parse_port_info_reply_test() {
        let reply: Vec<u8> = vec![11, 0, 67, 0, 1, 15, 6, 30, 0, 31, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0];
        //println!("{:?}", parse_port_info_reply(&reply).unwrap());

        let n: i8 = -8;
        println!("{}", n as u8);
    }
}