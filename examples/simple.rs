use qtmd::qtmd;
use std::{thread::sleep, time::Duration};

fn main() {
    for _ in qtmd(0..10000) {
        sleep(Duration::from_millis(1));
    }
}
