use std::sync::{atomic::AtomicBool, Arc};

// Defines if both threads should die.
pub static SHOULD_DIE: AtomicBool = AtomicBool::new(false);

// Defines if this should be in a paused state.
pub static PAUSED: AtomicBool = AtomicBool::new(false);

// Defines a stack.
pub struct Stack<T> {
    pub item: T,
    pub next: Option<Box<Stack<T>>>,
}

// Defines a optional boxed stack.
pub type OptionalBoxedStack = Option<Box<Stack<Arc<Vec<u8>>>>>;
