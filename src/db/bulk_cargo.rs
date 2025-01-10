//! Промежуточные структуры для serde_json для парсинга данных
//! сыпучего груза
use serde::{Deserialize, Serialize};

use super::DataArray;
/// Данные сыпучего груза
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BulkCargoData {
    pub name: Option<String>,
    pub mass: Option<f64>,
    pub x_g: Option<f64>,
    pub y_g: Option<f64>,
    pub z_g: Option<f64>,
    pub grain_moment: Option<f64>,   
}
//
impl std::fmt::Display for BulkCargoData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BulkCargoData(name:{}, mass:{}, x_g:{}, y_g:{}, z_g:{}, grain_moment:{})",
            self.name.clone().unwrap_or("-".to_string()), 
            self.mass.unwrap_or(0.), 
            self.x_g.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()), 
            self.y_g.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()), 
            self.z_g.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()), 
            self.grain_moment.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()), 
        )
    }
}
pub type BulkCargoDataArray = DataArray<BulkCargoData>;
//
impl BulkCargoDataArray {
    /// Преобразование и возвращает данные в виде вектора
    pub fn data(self) -> Vec<BulkCargoData> {
        self.data
    }
}
