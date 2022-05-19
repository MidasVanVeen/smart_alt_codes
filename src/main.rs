extern crate csv;
extern crate enigo;

use std::process::exit;
use std::fs::{File};
use std::io::{Read, Write};
use std::{env, mem, thread, time::Duration};

use enigo::*;

mod input;
use input::{is_key_event, is_key_press, is_key_release, is_shift, get_key_text, InputEvent};

struct Options {
    device_path: String,
    patterns_path: String
}

fn parse_arguments() -> Options {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: ./smart_alt_codes <device file path (check /dev/input/by-path)> <pattern csv file>");
        exit(-1);
    }
    Options {
        device_path: args[1].clone(),
        patterns_path: args[2].clone()
    }
}

fn main() {
    let options: Options = parse_arguments();
    
    let mut device_file = File::open(options.device_path.as_str()).unwrap_or_else(|e| panic!("{}",e));

    let mut enigo = Enigo::new();

    let mut buf: [u8; 24] = unsafe { mem::zeroed() };
    let mut sequence = String::new();
    let mut shift_pressed = 0;
    let mut recording: bool = false;
    loop {
        let num_bytes = device_file.read(&mut buf).unwrap_or_else(|e| panic!("{}", e));
        if num_bytes != mem::size_of::<InputEvent>() {
            panic!("Error while reading from device file");
        }
        let event: InputEvent = unsafe { mem::transmute(buf) };
        if is_key_event(event.type_) {
            if is_key_press(event.value) {
                if is_shift(event.code) {
                    shift_pressed += 1;
                }

                let text = get_key_text(event.code, shift_pressed);

                if recording {
                    if text != "<LShift>" && text != "<RShift>" {
                        sequence += text;
                    }
                }

                if text == "<RAlt>" {
                    recording = true;
                    sequence = String::new();
                }

                let mut pattern_reader = csv::Reader::from_path(options.patterns_path.as_str()).unwrap();
                for r in pattern_reader.records() {
                    let record = r.unwrap();
                    if record[0] == sequence {
                        thread::sleep(Duration::from_millis(20));
                        for _ in 0..sequence.chars().count() {
                            enigo.key_click(Key::Backspace);
                        }
                        thread::sleep(Duration::from_millis(50));
                        enigo.key_down(Key::Shift);
                        enigo.key_sequence(&record[1]);
                        sequence = String::new();
                        recording = false;
                    }
                }

                //print!("{}", text);
                //io::stdout().flush().unwrap();

            } else if is_key_release(event.value) {
                if is_shift(event.code) {
                    shift_pressed -= 1;
                }
            }
        }
    }
}