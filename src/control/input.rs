use std::sync::{Mutex, OnceLock};

use enigo::{Enigo, Settings};

static ENIGO: OnceLock<Mutex<Enigo>> = OnceLock::new();

pub fn get_enigo() -> &'static Mutex<Enigo> {
    ENIGO.get_or_init(|| Mutex::new(Enigo::new(&Settings::default()).unwrap()))
}
