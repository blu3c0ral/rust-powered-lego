extern crate rust_powered_lego;

#[cfg(test)]
mod tests {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct Point {
        x: u8,
        y: u8,
    }

    #[test]
    fn trial_test() {
        let p = Point {x: 0x1, y: 0x2e};

        let bytes2 = bincode::serialize(&p).unwrap();
        println!("{:?}", bytes2);
    }
}