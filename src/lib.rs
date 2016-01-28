
#[macro_export]
macro_rules! bit_check {
    ($bit_field:expr, $($mask:expr),+) => (
        bit_filter!($bit_field, $($mask),+) == bit_set!($($mask),+)
    )
}

#[macro_export]
macro_rules! bit_filter {
    ($bit_field:expr, $mask:expr) => ($bit_field & $mask);
    ($bit_field:expr, $mask:expr, $($masks:expr),+) => (
        $bit_field & $mask | bit_filter!($bit_field, $($masks),+)
    )
}

#[macro_export]
macro_rules! bit_set {
    ($mask:expr) => ($mask);
    ($bit_field:expr, $($masks:expr),+) => ($bit_field | bit_set!($($masks),+))
}

#[macro_export]
macro_rules! bit_clear {
    ($bit_field:expr, $mask:expr) => ($bit_field & (!$mask));
    ($bit_field:expr, $($masks:expr),+) => (
        bit_clear!($bit_field, bit_set!($($masks),+))
    )
}
