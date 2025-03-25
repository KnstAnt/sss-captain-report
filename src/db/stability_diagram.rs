//! Промежуточные структуры для serde_json для парсинга данных
//! диаграммы плечей остойчивости
use serde::{Deserialize, Serialize};

use super::DataArray;
/// Данные диаграммы плечей остойчивости
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StabilityDiagramData {
    pub angle: f64,
    pub value_dso: f64,
    pub value_ddo: f64,
}
//angle, value_dso
impl std::fmt::Display for StabilityDiagramData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StabilityDiagramData(angle:{}, value_dso:{}, value_ddo:{} )",
            self.angle, self.value_dso, self.value_ddo,
        )
    }
}
pub type StabilityDiagramDataArray = DataArray<StabilityDiagramData>;
//
impl StabilityDiagramDataArray {
    // (angle, value_dso,)
    pub fn data(self) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
        self.data
            .into_iter()
            .map(|v| ((v.angle, v.value_dso), (v.angle, v.value_ddo)))
            .unzip()
    }
}
