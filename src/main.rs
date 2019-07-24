extern crate regex;

use std::{str, io};
use std::process::Command;
use regex::Regex;

fn main() -> io::Result<()> {
	let mut buffer = String::new();
  let stdin = io::stdin();

	//could be optimized
	let artist = Regex::new(r"xesam:artist[ ]+([\pLN ]+)").unwrap();
	let title = Regex::new(r"xesam:title[ ]+([\pLN ]+)").unwrap();
	
	//remove version header and start of an array
	for _ in 0..3 {
		buffer.clear();
	 	stdin.read_line(&mut buffer)?;
		print!("{}", buffer)
	}

	loop {
		buffer.clear();
 		stdin.read_line(&mut buffer)?;

		//playerctl can't detect if is spotify playing or it has it paused. DBUS required.
		let meta = Command::new("/bin/playerctl")
			.arg("metadata")
			.output()
			.expect("Playerctl failed or is not installed")
			.stdout;

		let metadata = match str::from_utf8(meta.as_slice()) {
			Ok(metadata) => metadata,
			Err(_) => panic!("Playerctl failed to give metadata")
		};

		//unsafe but works, because those are ASCII chars.
		let json_parsable = &buffer.as_str()[2..];

		let current_artist = &artist.captures(metadata).unwrap()[1];
		let current_title = &title.captures(metadata).unwrap()[1];

		let spotify = format!("{} - {}", current_artist, current_title);

		//appends only on first position :(, and it's tricky, + doesn't require json library	
		print!(",[{{\"name\":\"spotify\",\"color\":\"#00FF00\",\"full_text\":\"{}\"}},{}", spotify, json_parsable)
	}
}
