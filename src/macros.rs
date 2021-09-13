#[macro_export]
macro_rules! verbose {
    ( $c:ident, $($arg:tt)*) => ({
        if $c.v {
            println!($($arg)*);
        }
    })
}
