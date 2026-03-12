pub use parameters::Parameters;
pub mod misc;
pub mod strength;
pub mod stability;
pub mod general;
pub mod displacement;
pub mod parameters;
pub mod load_line;

//
pub trait Content {
    //
    fn to_string(self) -> Result<String, sal_core::error::Error>;
}
