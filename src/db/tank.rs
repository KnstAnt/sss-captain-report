//! Промежуточные структуры для serde_json для парсинга данных
//! отсеков
use serde::{Deserialize, Serialize};

use super::DataArray;
/// Данные отсека
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TankData {
    pub name: Option<String>,
    pub mass: Option<f64>,
    pub x_g: Option<f64>,
    pub y_g: Option<f64>,
    pub z_g: Option<f64>,
    pub f_sx: Option<f64>,   
}
//
impl std::fmt::Display for TankData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TankData(name:{}, mass:{}, x_g:{}, y_g:{}, z_g:{}, f_sx:{})",
            self.name.clone().unwrap_or("-".to_string()), 
            self.mass.unwrap_or(0.), 
            self.x_g.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()), 
            self.y_g.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()), 
            self.z_g.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()), 
            self.f_sx.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()), 
        )
    }
}
pub type TankDataArray = DataArray<TankData>;
//
impl TankDataArray {
    /// Преобразование и возвращает данные в виде вектора
    pub fn data(self) -> Vec<TankData> {
        self.data
    }
}
