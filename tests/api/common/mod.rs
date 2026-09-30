use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        dotenvy::from_filename(".env.development").ok();
    });
}
