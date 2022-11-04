extern crate rust_powered_lego;

#[cfg(test)]
mod tests {
    use rust_powered_lego::lego::check_for_lego_error;

    #[test]
    fn parse_lego_error_test() {
        let arr = vec![0x03, 0x04];
        let res = check_for_lego_error(&arr);

        assert_eq!(res.is_ok(), false);
    }
}