use deej_rs::PaControl;
use deej_rs::AudioInterface;

use deej_rs::HardwareController;
use deej_rs::SerialController;

use std::thread;
use std::time::Duration;
use std::fs;

fn main() {
    let tty_port_base = "/dev/ttyACM";
    let mut port_id = 0;
    let mut controller_opt: Option<SerialController> = None;
    while 1 == 1 {
        while port_id <= 9 {
            let tty_port_attempt = String::from(tty_port_base) + &port_id.to_string();
    
            println!("Testing {0}...", tty_port_attempt);
            if fs::metadata(tty_port_attempt.clone()).is_ok() {
                controller_opt = Some(SerialController::new(tty_port_attempt.as_str()));
                println!("Connected to port {0}", tty_port_attempt);
                break;
            }
            port_id = port_id + 1;
        }
        if controller_opt.is_none() {
            println!("Failed to find usable tty port, will retry in 3 seconds...");
            port_id = 0;
            thread::sleep(Duration::from_millis(3000));
        } else { break; }
    }

    let mut controller = controller_opt.unwrap();
    let interface = PaControl;

    let mut prev_values = controller.read_slidervalues();

    let mut should_sleep = true;
    let mut nochange_ticks = 0;
    loop {
        let values = controller.read_slidervalues();

        for slider in &values {
            let mut proc_name: &str = "";
            let mut pretty_name: &str = "";
            if slider.id == 0 { proc_name = "master"}
            if slider.id == 1 { proc_name = "spotify"; pretty_name = "Spotify"}
            if slider.id == 2 { proc_name = "floorp"; pretty_name = "Floorp"}
            if slider.id == 3 { proc_name = "discord"; pretty_name = "Discord"}

            if !proc_name.is_empty() && prev_values[slider.id as usize].perc != slider.perc {
                // Sliders were moved - Time to wake up
                if should_sleep { println!("Wakey wakey!") }
                should_sleep = false;
                nochange_ticks = 0;
                
                prev_values[slider.id as usize].perc = slider.perc;
                if proc_name == "master" {
                    interface.adjust_volume(slider.perc);
                } else {
                    interface.adjust_app_volume(proc_name, pretty_name, slider.perc); 
                }
            } else if !should_sleep {
                nochange_ticks += 1;
            }
        }
        if nochange_ticks >= 200 {
            // Could probably put this in the if statement below, but now it only runs once
            nochange_ticks = 0;
            should_sleep = true;
            println!("Sleeping...")
        }
        if should_sleep{
            thread::sleep(Duration::from_millis(500));
        } else {
            // We still sleep, but just a little less ;)
            thread::sleep(Duration::from_millis(50));
        }
    }
}