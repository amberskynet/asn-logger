#[macro_export]
macro_rules! m_error {
    ($($arg:tt)*) => {
        $crate::log::error!( target: LOG_MODULE_NAME, $($arg)+)
    };
}

#[macro_export]
macro_rules! m_debug {
    ($($arg:tt)*) => {
        $crate::log::debug!( target: LOG_MODULE_NAME, $($arg)+)
    };
}

#[macro_export]
macro_rules! m_info {
    ($($arg:tt)*) => {
        ::asn_logger::log::info!( target: LOG_MODULE_NAME, $($arg)+)
    };
}

#[macro_export]
macro_rules! m_warn {
    ($($arg:tt)*) => {
        $crate::log::warn!( target: LOG_MODULE_NAME, $($arg)+)
    };
}

#[macro_export]
macro_rules! m_trace {
    ($($arg:tt)*) => {
        $crate::log::trace!( target: LOG_MODULE_NAME, $($arg)+)
    };
}

#[macro_export]
macro_rules! t_info {
    ($target:expr, $($arg:tt)*) => {
        $crate::log::info!( target: $target, $($arg)+)
    };
}

#[macro_export]
macro_rules! t_warn {
    ($target:expr, $($arg:tt)*) => {
        $crate::log::warn!( target: $target, $($arg)+)
    };
}

#[macro_export]
macro_rules! t_trace {
    ($target:expr, $($arg:tt)*) => {
        $crate::log::trace!( target: $target, $($arg)+)
    };
}

#[macro_export]
macro_rules! t_debug {
    ($target:expr, $($arg:tt)*) => {
        $crate::log::debug!( target: $target, $($arg)+)
    };
}

#[macro_export]
macro_rules! t_error {
    ($target:expr, $($arg:tt)*) => {
        $crate::log::error!( target: $target, $($arg)+)
    };
}
