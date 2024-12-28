//! Промежуточные структуры для serde_json для парсинга данных
//! расчета прочности 
use serde::{Deserialize, Serialize};

use super::DataArray;
/// Данные расчета прочности 
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StrengthResultData {
    /// Перерезывающие силы
    pub sf: f64,
    /// Изгибающий момент
    pub bm: f64,
}
//
impl std::fmt::Display for StrengthResultData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StrengthResultData(sf:{}, bm:{} )",
            self.sf, self.bm,
        )
    }
}
pub type StrengthResultDataArray = DataArray<StrengthResultData>;
//
impl StrengthResultDataArray {
    /// Преобразование и возвращает данные в виде вектора (индекс, начало, конец) шпации
    pub fn data(mut self) -> Vec<(f64, f64)> {
        self
            .data
            .iter_mut()
            .map(|v| (v.sf, v.bm))
            .collect()
    }
}
