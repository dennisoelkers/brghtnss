use std::env;
use std::fs::File;
use std::io::prelude::*;

static ACTUAL_BRIGHTNESS: &'static str = "/sys/class/backlight/gmux_backlight/actual_brightness";
static MAX_BRIGHTNESS: &'static str = "/sys/class/backlight/gmux_backlight/max_brightness";
static SET_BRIGHTNESS: &'static str = "/sys/class/backlight/gmux_backlight/brightness";

fn read_int_from_file(filename: String) -> Result<u32, std::io::Error> {
    File::open(filename).map(|mut file| {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        contents
    }).map(|contents| {
        contents.trim().parse::<u32>().unwrap()
    })
}

fn write_int_to_file(filename: String, number: u32) -> std::io::Result<()> {
    std::fs::write(filename, number.to_string())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let actual = read_int_from_file(String::from(ACTUAL_BRIGHTNESS))?;
            let max = read_int_from_file(String::from(MAX_BRIGHTNESS))?;

            let delta = max / 30;
            let new_brightness = match args[1].as_str() {
                "+" => { actual + delta }
                "-" => { actual - delta }
                _ => { actual - delta }
            };

            write_int_to_file(String::from(SET_BRIGHTNESS), new_brightness)
        },
        _ => {
            eprintln!("Syntax: {:?} +|-", args[0]);
            Ok(())
        }
    }

}
