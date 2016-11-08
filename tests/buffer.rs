extern crate hoedown;
extern crate timebomb;

use std::vec::Vec;
use std::io::Read;

use hoedown::Buffer;

use timebomb::timeout_ms;

#[test]
fn test_read_to_end() {
    let buffer = Buffer::from("This is a test");
    let mut destination = Vec::new();

    timeout_ms(move || {
        let mut slice: &[u8] = &buffer;

        slice.read_to_end(&mut destination).unwrap();
    }, 1000);
}
