use std::collections::HashMap;

use crate::{
    content::{misc::Table, Content},
    db::criterion::CriteriaData,
    error::Error,
};

pub struct Criterion {
    table: Table,
}
//
impl Criterion {
    //
    pub fn new(table: Table) -> Self {
        Self { table }
    }
    //
    pub fn from(data: &HashMap<i32, CriteriaData>) -> Result<Self, Error> {
        let header = vec![
            "№",
            "Наименование",
            "Размерность",
            "Значение",
            "Допустимое значение",
            "Статуc",
        ];
        let content = data
            .iter()
            .map(|(_, v)| {
                format!(
                    "{},{},{},{},{},{}",
                    v.id,
                    v.name,
                    v.unit,
                    v.result
                        .clone()
                        .map(|v| v.to_string())
                        .unwrap_or("-".to_owned()),
                    v.target
                        .clone()
                        .map(|v| v.to_string())
                        .unwrap_or("-".to_owned()),
                    v.state
                        .clone()
                        .map(|v| v.to_string())
                        .unwrap_or("-".to_owned()),
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
impl Content for Criterion {
    //
    fn to_string(self) -> Result<String, crate::error::Error> {
        self.table.to_string()
    }
}
