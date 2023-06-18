extern crate serial;

use std::process::Command;
use serial::{prelude::*, unix::TTYPort};

use std::io::prelude::*;

pub trait AudioInterface {
    fn adjust_volume(&self, volume: u8);
    fn adjust_app_volume(&self, app_name: &str, volume: u8);
}

pub trait HardwareController {
    fn read_slidervalues(&mut self) -> Vec<SliderValue>;
}

pub struct PaControl;
pub struct SerialController {
    port: TTYPort
}

pub struct SliderValue {
    pub id: u8,
    pub raw_val: u16,
    pub perc: u8
}

impl SerialController {
    pub fn new(device: &str) -> SerialController {
        let mut port = serial::open(device).unwrap();
        
        port.reconfigure(&|settings| {
            settings.set_baud_rate(serial::Baud9600)?;
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        });

        SerialController { port }
    }
}

impl HardwareController for SerialController {
    fn read_slidervalues(&mut self) -> Vec<SliderValue> {
        let mut string: Vec<u8> = Vec::with_capacity(30);
        let mut first_run = true;
        let mut values: Vec<SliderValue> = Vec::new();

        loop {
            let mut buf = [0 as u8];
            self.port.read(&mut buf);

            if buf[0] >= 128 {
                continue;
            }

            match buf[0] {
                10 => { // Buffer matches a new line?
                    if first_run == true {
                        first_run = false;
                        continue;
                    }
                    let line = String::from_utf8(string.clone()).expect("Found invalid UTF-8");
                    let mut index: u8 = 0;

                    for slider in line.split('|') {
                        if slider.parse::<u16>().is_ok() {
                            values.push(SliderValue { id: index, raw_val: slider.parse::<u16>().expect("Failed to parse"), perc: 0 });
                            index += 1;
                            if index >= 4 {
                                break;
                            }
                            println!("{}", line);
                        }
                    }
                    break;
                }
                c => {
                    string.push(c);
                }
            }

            /*match buf[0] {
                10 => { // Buffer matches a new line?
                    
                    let line = String::from_utf8(string.clone()).expect("Found invalid UTF-8");
                    let mut index: u8 = 0;

                    /*for slider in line.split('|') {
                        if slider.parse::<u16>().is_ok() {
                            values.push(SliderValue { id: index, raw_val: slider.parse::<u16>().expect("Failed to parse"), perc: 0 });
                            index += 1;
                        }
                    }*/
                    println!("Line: {}", line);

                    if first_run == true {
                        first_run = false;
                        //continue;
                    }
                    //break;
                }
                c => {
                    string.push(c);
                }
            }*/
        }
        values
    }
}


impl PaControl {
    fn search_sink(&self, app_name: &str) -> Vec<String> {
        let mut matching_sinks: Vec<String> = Vec::new();
        let mut active_sink = "";
        let mut active_sink_matched = false;
        let pactl_out = cmd("pactl list sink-inputs");

        for line in pactl_out.split('\n') 
        {
            if line.contains("Sink Input #") 
            {
                active_sink = &line[12..];
                active_sink_matched = false;
            } 
            else if !active_sink_matched && (line.to_lowercase().contains(&format!("application.name = \"{0}\"", app_name.to_lowercase())) || line.to_lowercase().contains(&format!("application.process.binary = \"{0}\"", app_name.to_lowercase())))
            {
                matching_sinks.push(active_sink.to_string()); //Let that sink in!
                active_sink_matched = true;
            }
        }
        matching_sinks
    }
}

impl AudioInterface for PaControl {
    fn adjust_volume(&self, volume: u8) {
        println!("Now adjusting general volume to {0}%", volume);
        cmd(&format!("pactl set-sink-volume @DEFAULT_SINK@ {0}%", volume));
    }

    fn adjust_app_volume(&self, app_name: &str, volume: u8) {
        println!("Now adjusting app volume of {0} to {1}%", app_name, volume);
        let app_sinks = &self.search_sink(app_name);

        for sink in app_sinks.iter() {
            cmd(&format!("pactl set-sink-input-volume {0} {1}%", sink, volume));
        }
    }
}

fn cmd(cmd: &str) -> String {
    String::from_utf8(
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect(&format!("Execution of command {0} failed", cmd))
            .stdout,
    )
    .unwrap()
}