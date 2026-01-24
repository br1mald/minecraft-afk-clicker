use std::{fs, path::Path, thread::sleep, time::Duration};

use enigo::{
    Button, Direction::Click, Direction::Press, Direction::Release, Enigo, Keyboard, Mouse,
    Settings,
};

const ATTACK_DELAY_MS: u64 = 650; // diamond sword attacks need a 0.625 delay to recharge in Java edition
const ATTACKS_BEFORE_EATING: i32 = 1850; //this rougly corresponds to 20 minutes
const FOOD_SLOT: u16 = 28; // this keycode corresponds to slot 8.
const WEAPON_SLOT: u16 = 19; // this keycode corresponds to slot 2.
const EATING_DURATION: f32 = 1.6; // time needed to eat, this is 1.6s for most food items.
const STOP_FILE: &str = "/tmp/stop_autoclicker";

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let initial_delay = Duration::from_secs(3);
    let attack_delay = Duration::from_millis(ATTACK_DELAY_MS);
    let swapping_delay = Duration::from_millis(100); // to avoid having too many inputs simultaneously
    let holding_time = Duration::from_secs_f32(EATING_DURATION);

    sleep(initial_delay);

    let mut attack_counter = 1845;
    loop {
        if Path::new(STOP_FILE).exists() {
            let _ = fs::remove_file(STOP_FILE);
            break;
        }

        if attack_counter == ATTACKS_BEFORE_EATING {
            enigo
                .raw(FOOD_SLOT, Click)
                .expect("Failed to switch to food");
            sleep(swapping_delay);
            enigo
                .button(Button::Right, Press)
                .expect("Failed to hold right click");
            sleep(holding_time);
            enigo.button(Button::Right, Release).unwrap();
            sleep(swapping_delay);
            enigo
                .raw(WEAPON_SLOT, Click)
                .expect("Failed to switch to sword");
            sleep(swapping_delay);
            attack_counter = 0;
        }

        enigo
            .button(Button::Left, Click)
            .expect("Failed to input a click");
        attack_counter += 1;
        sleep(attack_delay);
    }
}
