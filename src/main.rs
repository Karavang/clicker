use enigo::{Enigo, MouseButton, MouseControllable};
use inputbot::{handle_input_events, KeybdKey::*};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    thread, time,
};

fn main() {
    println!("Clicker run!");
    let is_active = Arc::new(AtomicBool::new(false));
    let is_active_clone = Arc::clone(&is_active);
    thread::spawn(move || {
        let mut enigo = Enigo::new();
        loop {
            if is_active_clone.load(Ordering::Relaxed) {
                enigo.mouse_click(MouseButton::Left);
                thread::sleep(time::Duration::from_millis(5));
            }
        }
    });
    Numpad6Key.bind(move || is_active.store(!is_active.load(Ordering::Relaxed), Ordering::Relaxed));
    handle_input_events();
}
