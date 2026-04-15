/// Debug utilities for tracing packet parsing
use std::cell::RefCell;

thread_local! {
    pub static DEBUG_TRACE: RefCell<bool> = RefCell::new(false);
}

#[macro_export]
macro_rules! debug_pos {
    ($reader:expr, $msg:expr) => {
        #[cfg(debug_assertions)]
        if let Ok(packet_reader) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // Try to get position if it's a PacketReader
            // This is hacky but works for debugging
        })) {
            if crate::network::debug::DEBUG_TRACE.with(|dt| *dt.borrow()) {
                eprintln!("[DEBUG] {}", $msg);
            }
        }
    };
}
