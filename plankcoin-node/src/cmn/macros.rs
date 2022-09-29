#[macro_export]
macro_rules! log_bail {
    ($expr:expr) => {{
        error!($expr);
        bail!($expr)
    }};
}

pub use log_bail;
