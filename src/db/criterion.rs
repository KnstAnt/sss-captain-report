use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::DataArray;
// Структура для парсинга критериев
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CriteriaData {
    pub id: i32,
    pub name: String,
    pub unit: String,
    pub result: Option<f64>,
    pub target: Option<f64>,
    pub state: Option<bool>,
}
//
impl std::fmt::Display for CriteriaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CriteriaData(id:{}, name:{:?}, unit:{:?}, result:{:?}, target:{:?}, state:{:?})",
            self.id, self.name, self.unit, self.result, self.target, self.state
        )
    }
}
//
pub type CriteriaDataArray = DataArray<CriteriaData>;
//
impl CriteriaDataArray {
    /// Преобразование данных в массив ключ + значение
    pub fn data(self) -> Vec<(i32, CriteriaData)> {
        self.data.into_iter().map(|v| (v.id, v)).collect()
    }
}
