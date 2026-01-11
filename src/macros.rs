#[macro_export]
macro_rules! debug {
    ($expr:expr) => {
        println!("{:#?}", $expr)
    };
}
