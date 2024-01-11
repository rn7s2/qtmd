use qtmd::{Iter, Style};
use std::{thread::sleep, time::Duration};

fn main() {
    for _ in (0..).take(10000).qtmd().style(Style::Balloon) {
        sleep(Duration::from_millis(1));
    }
}
