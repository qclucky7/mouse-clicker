use rdev::{Event, Key};
use rdev::{listen, simulate, Button, EventType, SimulateError};
use std::sync::{LazyLock, Mutex};
use std::thread;
use std::time::Duration;
use std::u64;
use tokio;
use std::collections::HashSet;

use crate::pusher;


pub static mut OPENED_MOUSE_CLICK: bool = false;
pub static mut INTERVAL_MILLISECONDS: u64 = 500;
static GLOBAL_THREAD_HANDLE: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
});

lazy_static::lazy_static! {
    static ref SHORTCUT_KEY: Mutex<CombineKeyState> = Mutex::new(CombineKeyState::new(vec![Key::MetaLeft, Key::F3]));
}
struct CombineKeyState {
    combination: Vec<Key>,
    pressed_keys: HashSet<Key>,
}

impl CombineKeyState {
    fn new(combination: Vec<Key>) -> Self {
        Self {
            combination: combination.clone(),
            pressed_keys: HashSet::with_capacity(combination.capacity()),
        }
    }
    fn key_press(&mut self, key: Key) {
        self.pressed_keys.insert(key);
    }
    fn key_release(&mut self, key: Key) {
        self.pressed_keys.remove(&key);
    }
    fn is_combination_active(&self) -> bool {
        self.combination.iter().all(|key| self.pressed_keys.contains(key))
    }
}

fn callback(event: Event) {
    //println!("event: {:?}", event);
    match event.event_type {
        EventType::KeyPress(key) => {
                SHORTCUT_KEY.lock().unwrap().key_press(key);
                if SHORTCUT_KEY.lock().unwrap().is_combination_active() {
                    unsafe {
                        OPENED_MOUSE_CLICK = !OPENED_MOUSE_CLICK;
                        if OPENED_MOUSE_CLICK {
                            GLOBAL_THREAD_HANDLE.spawn_blocking(|| {
                                mouse_click();
                            });
                            pusher::open_mouse_click(true);
                        } else {
                            pusher::open_mouse_click(false);
                        }
                    };
                }
        },
        EventType::KeyRelease(key) => SHORTCUT_KEY.lock().unwrap().key_release(key),
        _ => (),
    }
}

fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
}

fn mouse_click() {
    let delay = Duration::from_millis(unsafe { INTERVAL_MILLISECONDS });
    while unsafe { OPENED_MOUSE_CLICK } {
        send(&EventType::ButtonPress(Button::Left));
        send(&EventType::ButtonRelease(Button::Left));
        thread::sleep(delay);
    }
}

pub fn listen_event() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}
