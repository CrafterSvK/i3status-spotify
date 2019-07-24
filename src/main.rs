use std::{str, io};
use std::process::Command;

fn main() -> io::Result<()> {
	let mut buffer = String::new();
	let stdin = io::stdin();

	//first 2 are headers and third contains [ as a start of array
	for _ in 0..3 {
		buffer.clear();
	 	stdin.read_line(&mut buffer)?;
		print!("{}", buffer)
	}

	loop {
		buffer.clear();
 		stdin.read_line(&mut buffer)?;

		if playerctl!("status").as_str() == "Playing" {
			let title = playerctl!("metadata", "title"); 
			let artist = playerctl!("metadata", "artist");
		
			//unsafe but works, because those are ASCII chars.
			let json_parsable = &buffer.as_str()[2..];

			let spotify = format!("{} - {}", artist.as_str(), title.as_str());

			//appends only on first position :(, and it's tricky, + doesn't require json library	
			print!(",[{{\"name\":\"spotify\",\"color\":\"#00FF00\",\"full_text\":\"{}\"}},{}", spotify, json_parsable);
		} else {	
			print!("{}", buffer);
		}
	}
}

#[macro_export]
macro_rules! playerctl {
	( $( $x:expr ),*) => {
		{
			let mut command = Command::new("/bin/playerctl");

			$(
				command.arg($x);
			)*

			let data = command
				.output()
				.expect("Playerctl is not installed or doesn't work correctly")
				.stdout;

			match str::from_utf8(data.as_slice()) {
				Ok(meta) => meta.trim().to_string(),
				Err(_) => panic!("Playerctl failed to give metadata")
			}			
		}
	};
}
