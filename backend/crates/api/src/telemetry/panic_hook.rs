use std::panic;
use tracing::error;

pub fn set_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let payload = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic payload".to_string()
        };

        let location = panic_info
            .location()
            .map(|l| l.to_string())
            .unwrap_or_else(|| "unknown location".to_string());

        error!(
            target: "panic",
            panic_payload = %payload,
            location = %location,
            "Application panicked"
        );
    }));
}
