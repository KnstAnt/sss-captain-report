use crate::{
    content::{misc::Table, Content},
    db::voyage::VoyageData,
    error::Error,
};

pub struct Voyage {
    header: String,
    table: Table,
}
//
impl Voyage {
    //
    pub fn new(header: String, table: Table) -> Self {
        Self { header, table }
    }
    //
    pub fn from(language: &String, data: VoyageData) -> Result<Self, Error> {
        if language.contains("en") {
            Self::from_en(data)
        } else {
            Self::from_ru(data)
        }
    }
    //
    pub fn from_ru(data: VoyageData) -> Result<Self, Error> {
        let header = "# Рейс\n\n".to_owned();
        let table_header = vec!["Наименование", "Значение"];
        let mut content = Vec::new();
        let convert_str = |head: &str, data: &Option<String>| -> (String, String) {
            (head.to_owned(), data.clone().unwrap_or("-".to_owned()))
        };
        let convert_value = |head: &str, data: &Option<f64>| -> (String, String) {
            (
                head.to_owned(),
                data.clone()
                    .clone()
                    .map(|v| format!("{:.3}", v))
                    .unwrap_or("-".to_owned()),
            )
        };
        content.push(convert_str("Код рейса", &data.code));
        content.push(convert_str("Акватория", &data.area));
        content.push(convert_value(
            "Плотность забортной воды $[т/м^3]$",
            &data.density,
        ));
        content.push(convert_str("Грузовая марка", &data.load_line));
        content.push(convert_str("Обледенение", &data.icing));
        //        content.push(push_value("Намокание палубного лесного груза", &data.wetting));
        content.push(convert_str("Описание рейса", &data.description));
        let content = content
            .into_iter()
            .map(|(v1, v2)| vec![v1.to_owned(), v2])
            .collect();
        Ok(Self::new(header, Table::new(&table_header, content)))
    }
//
    pub fn from_en(data: VoyageData) -> Result<Self, Error> {
        let header = "# Voyage\n\n".to_owned();
        let table_header = vec!["Name", "Value"];
        let mut content = Vec::new();
        let convert_str = |head: &str, data: &Option<String>| -> (String, String) {
            (head.to_owned(), data.clone().unwrap_or("-".to_owned()))
        };
        let convert_value = |head: &str, data: &Option<f64>| -> (String, String) {
            (
                head.to_owned(),
                data.clone()
                    .clone()
                    .map(|v| format!("{:.3}", v))
                    .unwrap_or("-".to_owned()),
            )
        };
        content.push(convert_str("Voyage Code", &data.code));
        content.push(convert_str("Water area", &data.area));
        content.push(convert_value(
            "Seawater density $[t/m^3]$",
            &data.density,
        ));
        content.push(convert_str("Load Line", &data.load_line));
        content.push(convert_str("Icing", &data.icing));
        //        content.push(push_value("Намокание палубного лесного груза", &data.wetting));
        content.push(convert_str("Voyage description", &data.description));
        let content = content
            .into_iter()
            .map(|(v1, v2)| vec![v1.to_owned(), v2])
            .collect();
        Ok(Self::new(header, Table::new(&table_header, content)))
    }
}
//
impl Content for Voyage {
    //
    fn to_string(self) -> Result<String, crate::error::Error> {
        Ok(self.header + &self.table.to_string()?)
    }
}
