// info
#[macro_export]
macro_rules! info {
  ($($args:tt)*) => {
    #[cfg(feature = "logging")]
    log::info!($($args)*)
  }
}

// debug
#[macro_export]
macro_rules! debug {
  ($($args:tt)*) => {
    #[cfg(feature = "logging")]
    log::debug!($($args)*)
  }
}

// trace
#[macro_export]
macro_rules! trace {
  ($($args:tt)*) => {
    #[cfg(feature = "logging")]
    log::trace!($($args)*)
  }
}

// warn
#[macro_export]
macro_rules! warn {
  ($($args:tt)*) => {
    #[cfg(feature = "logging")]
    log::warn!($($args)*)
    
  }
}

// error
#[macro_export]
macro_rules! error {
  ($($args:tt)*) => {
    #[cfg(feature = "logging")]
    log::error!($($args)*)
  }
}
