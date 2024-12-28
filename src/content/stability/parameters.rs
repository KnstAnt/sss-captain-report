use std::collections::HashMap;

use crate::{
    content::{misc::Table, Content},
    error::Error,
};

pub struct Parameters {
    table: Table,
}
//
impl Parameters {
    //
    pub fn new(table: Table) -> Self {
        Self { table }
    }
    //
    pub fn from(data: &HashMap<i32, (String, String, Option<String>)>) -> Result<Self, Error> {
        let header = vec!["№", "Наименование", "Размерность", "Значение"];
        let content = data
            .iter()
            .map(|(i, (name, unit, value))| {
                format!(
                    "{i},{name},{unit},{}",
                    value
                        .clone()
                        .map(|v| v.to_string())
                        .unwrap_or("-".to_owned())
                )
            })
            .collect::<Vec<String>>();
        let content: Vec<Vec<String>> = content
            .into_iter()
            .map(|v| v.split(',').map(|v| v.to_owned()).collect())
            .collect();
        Ok(Self {
            table: Table::new(&header, content),
        })
    }
}
//
impl Content for Parameters {
    //
    fn to_string(self) -> Result<String, crate::error::Error> {
        self.table.to_string()
    }
}
