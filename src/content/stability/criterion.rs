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
    pub fn from(language: &String, data: &[(i32, CriteriaData)]) -> Result<Self, Error> {
        let header = if language.contains("en") {
            vec![
            "№",
            "Name",
            "Dimension",
            "Value",
            "Allow",
            "Status",
            ]
        } else {
            vec![
            "№",
            "Наименование",
            "Размерность",
            "Значение",
            "Допустимое значение",
            "Статуc",
            ]
        };  
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
                        .map(|v| format!("{:.3}", v))
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
        Ok(Self::new(Table::new(&header, content)))
    }
}
//
impl Content for Criterion {
    //
    fn to_string(self) -> Result<String, crate::error::Error> {
        self.table.to_string()
    }
}
