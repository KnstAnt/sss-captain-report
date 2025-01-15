use crate::{
    content::{misc::Table, Content},
    db::ship::ShipData,
    error::Error,
};

pub struct Ship {
    table: Table,
}
//
impl Ship {
    //
    pub fn new(table: Table) -> Self {
        Self { table }
    }
    //
    pub fn from(data: ShipData) -> Result<Self, Error> {
        let header = vec!["Параметр", "Значение"];
        let mut content = Vec::new();
        let convert_str =
            |v: Option<String>| -> String { v.map(|v| v.clone()).unwrap_or("-".to_owned()) };
        let convert_value =
            |v: Option<i32>| -> String { v.map(|v| v.to_string()).unwrap_or("-".to_owned()) };
        content.push(("Наименование судна", data.name.clone()));
        content.push(("Позывной", convert_str(data.call_sign)));
        content.push(("Номер ИМО", convert_value(data.imo)));
        content.push(("MMSI", convert_value(data.mmsi)));
        content.push(("Тип судна", convert_str(data.ship_type)));
        content.push(("Район плавания", convert_str(data.navigation_area)));
        content.push((
            "Классификационное общество",
            convert_str(data.classification_society),
        ));
        content.push(("Регистровый номер", convert_str(data.registration_number)));
        content.push(("Порт приписки", convert_str(data.port_of_registry)));
        content.push(("Флаг приписки", convert_str(data.flag_state)));
        content.push(("Судовладелец", convert_str(data.ship_owner)));
        content.push(("Код судовладельца", convert_str(data.ship_owner_code)));
        content.push(("Верфь постройки", convert_str(data.yard_of_build)));
        content.push(("Место постройки", convert_str(data.place_of_build)));
        content.push(("Год постройки", convert_value(data.year_of_build)));
        content.push((
            "Заводской номер",
            convert_str(data.ship_builder_hull_number),
        ));
        let content = content
            .into_iter()
            .map(|(v1, v2)| vec![v1.to_owned(), v2])
            .collect();
        Ok(Self::new(Table::new(&header, content)))
    }
}
//
impl Content for Ship {
    //
    fn to_string(self) -> Result<String, crate::error::Error> {
        Ok("# Судно\n".to_string() + &self.table.to_string()?)
    }
}
