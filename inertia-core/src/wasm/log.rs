macro_rules! console_debug {
  ($val:expr) => {
    match $val {
      tmp => {
        js_ffi::debug(
          &format_args!(
            "[{}:{}] {} = {:#?}",
            file!(),
            line!(),
            stringify!($val),
            &tmp
          )
          .to_string(),
        );
        tmp
      }
    }
  };
}

macro_rules! console_log {
  ($($t:tt)*) => (js_ffi::log(&format_args!($($t)*).to_string()))
}

macro_rules! console_warn {
  ($($t:tt)*) => (js_ffi::warn(&format_args!($($t)*).to_string()))
}

macro_rules! console_error {
  ($($t:tt)*) => (js_ffi::error(&format_args!($($t)*).to_string()))
}

pub(crate) use console_debug;

pub(crate) use console_log;

pub(crate) use console_warn;

pub(crate) use console_error;
