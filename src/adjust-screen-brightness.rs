//! rustc adjust-screen-brightness.rs
//! sudo install -g0 -o0 -s -t /usr/local/bin adjust-screen-brightness
//! if running without backlight.rules: sudo chmod u+s /usr/local/bin/adjust-screen-brightness


use std::{env, fs};


fn main() {
	let delta: i8 = env::args().nth(1).ok_or("Need delta as first argument").unwrap().parse().unwrap();

	// dbg!(delta);
	let cur_brightness: i8 = fs::read_to_string("/sys/class/backlight/acpi_video0/brightness").unwrap().trim_end().parse().unwrap();
	// dbg!(cur_brightness);
	let max_brightness: i8 = fs::read_to_string("/sys/class/backlight/acpi_video0/max_brightness").unwrap().trim_end().parse().unwrap();
	// dbg!(max_brightness);

	let mut new_brightness = cur_brightness + delta;
	if new_brightness > max_brightness {
		new_brightness = max_brightness;
	} else if new_brightness < 0 {
		new_brightness = 0;
	}

	if new_brightness != cur_brightness {
		// dbg!(new_brightness);
		fs::write("/sys/class/backlight/acpi_video0/brightness", new_brightness.to_string()).unwrap();
	}
}
