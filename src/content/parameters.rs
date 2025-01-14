use std::collections::HashMap;

use crate::{
    content::{misc::Table, Content},
    db::parameters::ParameterData,
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
    pub fn from(numbers: &[i32], data: &HashMap<i32, ParameterData>) -> Result<Self, Error> {
        let header = vec!["№", "Наименование", "Размерность", "Значение"];
        let content = numbers
            .iter()
            .map(|i| {
                data.get(i)
                    .map(|v| {
                        format!(
                            "{},{},{},{}",
                            v.id,
                            v.name
                                .clone()
                                .map(|v| v.to_string())
                                .unwrap_or("-".to_owned()),
                            v.unit
                                .clone()
                                .map(|v| v.to_string())
                                .unwrap_or("-".to_owned()),
                            v.result
                                .clone()
                                .map(|v| format!("{:.3}", v))
                                .unwrap_or("-".to_owned())
                        )
                    })
                    .unwrap_or(format!("{i},-,-,-"))
            })
            .collect::<Vec<String>>();
        let content: Vec<Vec<String>> = content
            .into_iter()
            .map(|v| v.split(',').map(|v| v.to_owned()).collect())
            .collect();
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
