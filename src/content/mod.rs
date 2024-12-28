use crate::error::Error;

pub mod misc;
pub mod chart;
pub mod strength;
pub mod stability;
pub mod general;
pub mod displacement;
//
pub trait Content {
    //
    fn to_string(self) -> Result<String, Error>;
}
