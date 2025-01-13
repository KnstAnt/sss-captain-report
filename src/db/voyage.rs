use serde::{Deserialize, Serialize};
use super::DataArray;

/// Структура для парсинга данных из таблицы ship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoyageData {
    pub code: Option<String>,
    pub area: Option<String>,
    pub density: Option<f64>,
    pub load_line: Option<String>,
    pub icing: Option<String>,
    pub wetting: Option<f64>,
    pub description: Option<String>,
}
//
pub type VoyageDataArray = DataArray<VoyageData>;
//
impl VoyageDataArray {
    ///
    pub fn data(self) -> Option<VoyageData> {
        self.data.into_iter().next()
    }
}
