use serde::{Deserialize, Serialize};
use super::DataArray;

/// Структура для парсинга данных из таблицы ship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItineraryData {
    pub port_name: String,
    pub port_code: String,
    pub eta: String,
    pub etd: String,
    pub max_draught: f64,
}
//
pub type ItineraryDataArray = DataArray<ItineraryData>;
//
impl ItineraryDataArray {
    ///
    pub fn data(self) -> Vec<ItineraryData> {
        self.data
    }
}
