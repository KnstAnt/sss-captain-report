use crate::{
    content::{misc::Table, Content}, db::voyage::VoyageData, error::Error
};

pub struct Voyage {
    table: Table,
}
//
impl Voyage {
    //
    pub fn new(table: Table) -> Self {
        Self { table }
    }
    //
    pub fn from(data: VoyageData) -> Result<Self, Error> {
        let header = vec!["Параметр", "Значение"];
        let mut content = Vec::new();
        content.push(("Код рейса", data.name.clone()));
        content.push(("Акватория", data.name.clone()));
        content.push(("Плотность забортной воды $[т/м^3]$", data.name.clone()));
        content.push(("Грузовая марка", data.name.clone()));
        content.push(("Обледенение", data.name.clone()));
        content.push(("Намокание палубного лесного груза", data.name.clone()));    
        content.push(("Описание рейса", data.name.clone()));
        let content = content.into_iter().map(|(v1, v2)| vec![v1.to_owned(), v2]).collect();
        Ok(Self {
            table: Table::new(&header, content),
        })
    }
}
//
impl Content for Voyage {
    //
    fn to_string(self) -> Result<String, crate::error::Error> {
        Ok("# Рейс\n".to_string() + &self.table.to_string()?)
    }
}
