use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::DataArray;
// Структура для парсинга параметров
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterData {
    pub id: i32,
    pub name: Option<String>,    
    pub result: Option<f64>,
    pub unit: Option<String>,
}
//
impl std::fmt::Display for ParameterData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParameterData(id:{}, name:{:?}, result:{:?}, unit:{:?})", self.id, self.name, self.result, self.unit)
    }
}
//
pub type ParameterDataArray = DataArray<ParameterData>;
//
impl ParameterDataArray {
    /// Преобразование данных в массив ключ + значение
    pub fn data(self) -> HashMap<i32, ParameterData> {
        self.data
            .into_iter()
            .map(|v| {
                    (v.id, v)
            })
            .collect()
    }
}
