use sal_core::error::Error;

use crate::{
    content::{misc::Table, Content},
    db::parameters::ParameterData,
};
use std::collections::HashMap;

pub struct Parameters {
    table: Table,
}
//
impl Parameters {
    //
    pub fn new(table: Table) -> Self {
        Self {table}
    }
    //
    pub fn from(
        language: &String,
        numbers: &[i32],
        data: &HashMap<i32, ParameterData>,
    ) -> Self {
        let header = if language.contains("en") {
            vec!["№", "Name", "Dimension", "Value"]
        } else {
            vec!["№", "Наименование", "Размерность", "Значение"]
        };
        let content = numbers
            .iter()
            .filter_map(|i| {
                data.get(i).filter(|v| v.result.is_some()).map(|v| {
                    vec![
                        v.id.to_string(),
                        v.name.clone().unwrap_or("-".to_owned()),
                        v.unit.clone().unwrap_or("-".to_owned()),
                        v.result
                            .clone()
                            .map(|v| format!("{:.3}", v))
                            .unwrap_or("-".to_owned()),
                    ]
                })
            })
            .collect::<Vec<Vec<String>>>();
        Self::new(Table::new(&header, content))
    }
}
//
impl Content for Parameters {
    //
    fn to_string(self) -> Result<String, Error> {
        Ok(self.table.to_string())
    }
}
