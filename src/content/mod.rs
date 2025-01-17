use crate::error::Error;
pub use parameters::Parameters;

pub mod misc;
pub mod strength;
pub mod stability;
pub mod general;
pub mod displacement;
pub mod parameters;
pub mod draught;
//
pub trait Content {
    //
    fn to_string(self) -> Result<String, Error>;
}
