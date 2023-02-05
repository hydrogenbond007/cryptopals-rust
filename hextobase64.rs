extern crate serialize;

use serialize::base64::{ToBase64, STANDARD};
use serialize::hex::{FromHex, ToHex};

fn main () {
  let stringOfText = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
  let mut config = STANDARD;

  println!("String to base64 = {}", stringOfText.as_bytes().to_base64(config));
