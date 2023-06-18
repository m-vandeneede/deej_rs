use deej_rs::PaControl;
use deej_rs::AudioInterface;

use deej_rs::HardwareController;
use deej_rs::SerialController;

extern crate serial;

use std::io;
use serial::prelude::*;

fn main() {
    let mut controller = SerialController::new("/dev/ttyACM0");
    let test = controller.read_slidervalues();

    for slider in test {
        println!("ID: {}\nRaw Value: {}", slider.id, slider.raw_val)
    }
    

    //let mut port = serial::open("/dev/ttyACM0").unwrap();
    //interact(&mut port).unwrap();
}

fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud9600)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;

    //try!(port.set_timeout(Duration::from_millis(1000)));

    let mut string: Vec<u8> = Vec::with_capacity(30);
    loop {
        let mut buf = [0 as u8];
        port.read(&mut buf)?;

        if buf[0] >= 128 {
            continue;
        }

        match buf[0] {
            10 => {
                let s = String::from_utf8(string.clone()).expect("Found invalid UTF-8");
                println!("{}", s);
            }
            c => {
                string.push(c);
            }
        }
    }

    Ok(())
}