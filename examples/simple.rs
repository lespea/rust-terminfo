#[macro_use(expand)]
extern crate terminfo;

use std::io::{self, Write};
use terminfo::{Expand, Database, capability as cap};

fn main() {
  let info = Database::from_env().unwrap();

  if let Some(cap::MaxColors(n)) = info.get::<cap::MaxColors>() {
    println!("The terminal supports {} colors.", n);
  }
  else {
    println!("The terminal does not support colors, what year is this?");
  }

  if let Some(flash) = info.get::<cap::FlashScreen>() {
		expand!(io::stdout(), flash).unwrap();
  }
	else {
		println!("FLASH GORDON!");
	}
}
