use crate::AppError;
use crate::shutdown::{SHUTDOWN_EVENTS, ShutdownEventsLock};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::{JsValue, wasm_bindgen};

// There is no shutdown mechanism on the web.
static _SHUTDOWN_EVENTS: &ShutdownEventsLock = &SHUTDOWN_EVENTS;

pub fn start_rust_logic_real<F, T>(main_fn: F) -> Result<(), AppError>
where
  F: Fn() -> T + 'static,
{
  // Add kind description for panics.
  #[cfg(debug_assertions)]
  {
    use crate::debug_print;
    use std::panic::set_hook;
    set_hook(Box::new(|panic_info| {
      debug_print!("A panic occurred in Rust.\n{}", panic_info);
    }));
  }

  // Run the main function.
  main_fn();

  Ok(())
}

#[wasm_bindgen]
extern "C" {
  // The reason this extern function is marked `catch`
  // and returns a `Result` is that the
  // `rinfBindings` JavaScript object created by Dart
  // does not exist in web workers; it is only available
  // in the main JavaScript thread. Loading the function
  // fails in web workers.
  #[wasm_bindgen(js_namespace = rinfBindings, catch)]
  pub fn rinf_send_rust_signal_extern(
    endpoint: &str,
    message_bytes: Uint8Array,
    binary: Uint8Array,
  ) -> Result<(), JsValue>;
}

pub fn send_rust_signal_real(
  endpoint: &str,
  message_bytes: Vec<u8>,
  binary: Vec<u8>,
) -> Result<(), AppError> {
  let result = rinf_send_rust_signal_extern(
    endpoint,
    Uint8Array::from(message_bytes.as_slice()),
    Uint8Array::from(binary.as_slice()),
  );
  result.map_err(|_| AppError::NoBindings)
}
