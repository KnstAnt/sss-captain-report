use crate::{
    content::{misc::Table, Content},
    db::parameters::ParameterData,
};
use sal_core::{dbg::Dbg, error::Error};
use std::collections::HashMap;

pub struct Parameters {
    dbg: Dbg,
    table: Table,
}
//
impl Parameters {
    //
    pub fn new(parent: &Dbg, table: Table) -> Self {
        let dbg = Dbg::new(parent, "Parameters");
        Self {dbg, table}
    }
    //
    pub fn from(
        language: &String,
        numbers: &[i32],
        data: &HashMap<i32, ParameterData>,
    ) -> Result<Self, Error> {
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
        Ok(Self::new(Table::new(&header, content)))
    }
}
//
impl Content for Parameters {
    //
    fn to_string(self) -> Result<String, crate::error::Error> {
        self.table.to_string()
    }
}
