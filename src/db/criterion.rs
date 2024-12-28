use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::DataArray;
// Структура для парсинга данных критериев и параметров
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataRow {
    pub id: i32,
    pub result: Option<f64>,
    pub target: Option<f64>,
}
//
impl std::fmt::Display for DataRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DataRow(id:{}, result:{:?}, target:{:?})", self.id, self.result, self.target)
    }
}
//
pub type DataRowArray = DataArray<DataRow>;
//
impl DataRowArray {
    /// Преобразование данных в массив ключ + значение
    pub fn data(&self) -> HashMap<i32, f64> {
        self.data
            .iter()
            .map(|v| {
                // Если ид=17 - Минимальная метацентрическая высота деления на отсеки
                // то для отчета берем целевое значение
                if v.id != 17 {
                    (v.id, v.result.unwrap_or(0.))
                } else {
                    (v.id, v.target.unwrap_or(0.))
                }
            })
            .collect()
    }
}
// Структура для парсинга данных параметров судна
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataShip {
    pub key: String,
    pub value: Option<f64>,
}
//
impl std::fmt::Display for DataShip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DataShip(key:{}, value:{:?})", self.key, self.value)
    }
}
//
pub type DataShipArray = DataArray<DataShip>;
//
impl DataShipArray {
    /// Преобразование данных в массив ключ + значение
    pub fn data(&self) -> HashMap<String, f64> {
        self.data
            .iter()
            .filter(|v| v.value.is_some())
            .map(|v| (v.key.clone(), v.value.unwrap()))
            .collect()
    }
}
