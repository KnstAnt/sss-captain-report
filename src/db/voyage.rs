use serde::{Deserialize, Serialize};
use super::DataArray;

/// Структура для парсинга данных из таблицы ship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoyageData {
    code: Option<String>,
    area: Option<String>,
    density: Option<f64>,
    load_line: Option<String>,
    icing: Option<f64>,
    wetting: Option<f64>,
    description: Option<String>,
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
