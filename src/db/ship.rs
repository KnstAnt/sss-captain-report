use serde::{Deserialize, Serialize};
use super::DataArray;

/// Структура для парсинга данных из таблицы ship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipData {
    pub name: String,
    pub call_sign: Option<String>,
    pub imo: Option<i32>,
    pub mmsi: Option<i32>,   
    pub ship_type: Option<String>,   
    pub navigation_area: Option<String>,   
    pub classification_society: Option<String>,
    pub registration_number: Option<String>,
    pub port_of_registry: Option<String>,
    pub flag_state: Option<String>,
    pub ship_master: Option<String>,
    pub ship_owner_code: Option<String>,  
    pub yard_of_build: Option<String>,   
    pub place_of_build: Option<String>,
    pub year_of_build: Option<i32>, 
    pub ship_builder_hull_number: Option<String>,
}
//
pub type ShipDataArray = DataArray<ShipData>;
//
impl ShipDataArray {
    ///
    pub fn data(self) -> Option<ShipData> {
        self.data.into_iter().next()
    }
}
/*
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
*/
