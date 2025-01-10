//! Промежуточные структуры для serde_json для парсинга данных
//! зерновых перегородок
use serde::{Deserialize, Serialize};

use super::DataArray;
/// Данные зерновых перегородок
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BulkheadData {
    pub name: Option<String>,
    pub fr: Option<f64>,    
    pub mass: Option<f64>,
    pub x_g: Option<f64>,
    pub y_g: Option<f64>,
    pub z_g: Option<f64>,
}
//
impl std::fmt::Display for BulkheadData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BulkheadData(name:{}, fr:{}, mass:{}, x_g:{}, y_g:{}, z_g:{})",
            self.name.clone().unwrap_or("-".to_string()), 
            self.fr.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()), 
            self.mass.unwrap_or(0.),             
            self.x_g.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()), 
            self.y_g.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()), 
            self.z_g.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()), 
        )
    }
}
pub type BulkheadDataArray = DataArray<BulkheadData>;
//
impl BulkheadDataArray {
    /// Преобразование и возвращает данные в виде вектора
    pub fn data(self) -> Vec<BulkheadData> {
        self.data
    }
}
