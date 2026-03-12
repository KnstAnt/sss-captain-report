use crate::{
    content::{misc::Table, Content},
    db::criterion::CriteriaData,
};
use sal_core::error::Error;

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
    pub fn from(language: &str, data: &[(i32, CriteriaData)]) -> Self {
        let header = if language.contains("en") {
            vec!["№", "Name", "Dimension", "Value", "Relation", "Allow", "Status"]
        } else {
            vec![
                "№",
                "Наименование",
                "Размерность",
                "Значение",
                "Cравнениe",
                "Допустимое значение",
                "Статуc",
            ]
        };
        let content = data
            .iter()
            .map(|(_, v)| {
                format!(
                    "{},{},{},{},{},{},{}",
                    v.id,
                    v.name,
                    v.unit.clone().unwrap_or("-".to_owned()),
                    v.result
                        .clone()
                        .map(|v| format!("{:.3}", v))
                        .unwrap_or("-".to_owned()),
                    v.relation
                        .clone()
                        .unwrap_or("-".to_owned()),
                    v.target
                        .clone()
                        .map(|v| format!("{:.3}", v))
                        .unwrap_or("-".to_owned()),
                    match v.state {
                        Some(true) => "+",
                        Some(false) => "-",
                        None => " ",
                    }
                    .to_owned()
                )
            })
            .collect::<Vec<String>>();
        let content: Vec<Vec<String>> = content
            .into_iter()
            .map(|v| v.split(',').map(|v| v.to_owned()).collect())
            .collect();
        Self::new(Table::new(&header, content))
    }
}
//
impl Content for Criterion {
    //
    fn to_string(self) -> Result<String, Error> {
        Ok(self.table.to_string())
    }
}
