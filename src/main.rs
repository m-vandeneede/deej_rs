use deej_rs::PaControl;
use deej_rs::AudioInterface;

use deej_rs::HardwareController;
use deej_rs::SerialController;

fn main() {
    let mut controller = SerialController::new("/dev/ttyACM0");
    let test = controller.read_slidervalues();

    for slider in test {
        println!("ID: {}\nRaw Value: {}", slider.id, slider.raw_val)
    }
}