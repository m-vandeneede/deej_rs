use deej_rs::PaControl;
use deej_rs::AudioInterface;

use deej_rs::HardwareController;
use deej_rs::SerialController;

use std::thread;
use std::time::Duration;

fn main() {
    let mut controller = SerialController::new("/dev/ttyACM0");
    let interface = PaControl;

    let mut prev_values = controller.read_slidervalues();

    let mut should_sleep = true;
    let mut nochange_ticks = 0;
    loop {
        let values = controller.read_slidervalues();

        for slider in &values {
            let mut proc_name: &str = "";
            if slider.id == 0 { proc_name = "master"}
            if slider.id == 1 { proc_name = "spotify"}
            if slider.id == 2 { proc_name = "brave"}
            if slider.id == 3 { proc_name = "discord"}

            if !proc_name.is_empty() && prev_values[slider.id as usize].perc != slider.perc {
                // Sliders were moved - Time to wake up
                if should_sleep { println!("Wakey wakey!") }
                should_sleep = false;
                nochange_ticks = 0;
                
                prev_values[slider.id as usize].perc = slider.perc;
                if proc_name == "master" {
                    interface.adjust_volume(slider.perc);
                } else {
                    interface.adjust_app_volume(proc_name, slider.perc); 
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