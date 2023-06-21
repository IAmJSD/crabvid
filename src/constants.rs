use std::sync::atomic::AtomicBool;

// Defines if both threads should die.
pub const SHOULD_DIE: AtomicBool = AtomicBool::new(false);
