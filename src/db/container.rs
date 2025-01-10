//! Промежуточные структуры для serde_json для парсинга данных
//! контейнеров
use serde::{Deserialize, Serialize};

use super::DataArray;
/// Данные контейнеров
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContainerData {
    pub owner_code: i32,
    pub serial_code: i32,
    pub check_digit: i32,
    pub bay_number: i32,
    pub row_number: i32,
    pub tier_number: i32, 
    pub mass: f64,
    pub x_g: f64,
    pub y_g: f64,
    pub z_g: f64,
}
//
impl ContainerData {
    //
    pub fn name(&self) -> String {
        format!("{} U {:6} {}", self.owner_code, self.serial_code, self.check_digit)
    }
    //
    pub fn bbrrtt(&self) -> String {
        format!("{:2}{:2}{:2}", self.bay_number, self.row_number, self.tier_number)
    }
}
//
impl std::fmt::Display for ContainerData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ContainerData(name:{}, bbrrtt:{}, mass:{}, x_g:{}, y_g:{}, z_g:{})",
            self.name(), 
            self.bbrrtt(), 
            self.mass,             
            self.x_g, 
            self.y_g, 
            self.z_g, 
        )
    }
}
pub type ContainerDataArray = DataArray<ContainerData>;
//
impl ContainerDataArray {
    /// Преобразование и возвращает данные в виде вектора
    pub fn data(self) -> Vec<ContainerData> {
        self.data
    }
}
