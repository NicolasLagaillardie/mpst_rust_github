#[macro_export]
macro_rules! checking {
    ($session: expr) => {
        mpst_seq::checking!($session);
    };
}
