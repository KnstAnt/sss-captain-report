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
        let header = vec!["Наименование", "Значение"];
        let mut content = Vec::new();
        let convert_str = |head: &str, data: &Option<String>| -> (String, String) { (head.to_owned(), data.clone().unwrap_or("-".to_owned())) };
        let convert_value = |head: &str, data: &Option<f64>| -> (String, String) { (head.to_owned(), data.clone().clone().map(|v| format!("{:.3}", v)).unwrap_or("-".to_owned())) };
        content.push(convert_str("Код рейса", &data.code));
        content.push(convert_str("Акватория", &data.area));
        content.push(convert_value("Плотность забортной воды $[т/м^3]$", &data.density));
        content.push(convert_str("Грузовая марка", &data.load_line)); 
        content.push(convert_str("Обледенение", &data.icing));
//        content.push(push_value("Намокание палубного лесного груза", &data.wetting)); 
        content.push(convert_str("Описание рейса", &data.description));                
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
