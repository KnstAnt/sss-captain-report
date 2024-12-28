use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_parser::IFromJson;

pub mod api_server;
pub mod ship;
pub mod voyage;
pub mod itinerary;
mod criterion;
mod serde_parser;
mod computed_frame;
mod strength_result;
mod strength_limit;
mod stability_diagram;

/// Массив ключ + значение
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataArray<T> {
    pub data: Vec<T>,
    pub error: HashMap<String, String>,
}
//
impl<T> IFromJson for DataArray<T> {
    fn error(&self) -> Option<&String> {
        self.error.values().next()
    }
}



