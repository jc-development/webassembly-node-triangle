use anyhow::{anyhow, Result};
use web_sys::{Document, Window};

// macro that allows you to log to the console with log!
macro_rules! log {
  ($($t:tt)*) => {
    web_sys::console::log_1(&format!($($t)*
      ).into());
  }
}

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("No Window Found"))
}

pub fn document() -> Result<Document> {
    window()?
        .document()
        .ok_or_else(|| anyhow!("No Document Found"))
}
