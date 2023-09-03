macro_rules! console_log {
  ($($t:tt)*) => (js_ffi::log(&format_args!($($t)*).to_string()))
}

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

pub(crate) use console_debug;
pub(crate) use console_log;
