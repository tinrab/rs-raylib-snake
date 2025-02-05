use std::cell::RefCell;

use crate::raylib::{self, KeyboardKey};

pub struct Input {
    key_state: [bool; 4],
}

thread_local! {
    static INSTANCE: RefCell<Input> = RefCell::new(Input::new());
}

impl Input {
    fn new() -> Self {
        Input {
            key_state: [false; 4],
        }
    }

    pub fn update() {
        INSTANCE.with(|r| {
            let key_state = &mut r.borrow_mut().key_state;

            macro_rules! key {
                ($key:expr) => {
                    if raylib::is_key_down($key) {
                        key_state[($key as usize)] = true;
                    }
                };
            }

            key!(KeyboardKey::KeyUp);
            key!(KeyboardKey::KeyDown);
            key!(KeyboardKey::KeyLeft);
            key!(KeyboardKey::KeyRight);
        });
    }

    pub fn clear() {
        INSTANCE.with(|r| {
            r.borrow_mut().key_state = [false; 4];
        });
    }

    pub fn is_key_down(key: KeyboardKey) -> bool {
        INSTANCE.with_borrow(|r| r.key_state[key as usize])
    }
}
