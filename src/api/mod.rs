pub mod build;
pub mod fmt;
pub mod json;
pub mod lush;
pub mod math;
pub mod string;
pub mod sys;

#[macro_export]
macro_rules! reg {
    ($table:expr, $lua:expr, $( $name:expr => $fn:expr ),* $(,)?) => {{
        $(
            $table.set($name, $lua.create_function($fn).unwrap()).unwrap();
        )*
    }};
}

#[macro_export]
macro_rules! regv {
    ($table:expr, $lua:expr, $( $key:expr => $value:expr ),* $(,)?) => {{
        $(
            $table.set($key, $value).unwrap();
        )*
    }};
}
