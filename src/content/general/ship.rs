use crate::{
    content::{misc::Table, Content}, db::ship::ShipData, error::Error
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
        let o_str_to = |v: Option<String>| -> String {
            v.map(|v| v.clone()).unwrap_or("-".to_owned())
        };
        let o_i32_to = |v: Option<i32>| -> String {
            v.map(|v| v.to_string()).unwrap_or("-".to_owned())
        };
        content.push(("Наименование судна", data.name.clone()));
        content.push(("Позывной", o_str_to(data.call_sign)));
        content.push(("Номер ИМО", o_i32_to(data.imo)));
        content.push(("MMSI", o_i32_to(data.mmsi)));
        content.push(("Тип судна", o_str_to(data.ship_type)));
        content.push(("Район плавания", o_str_to(data.navigation_area)));    
        content.push(("Классификационное общество", o_str_to(data.classification_society)));
        content.push(("Регистровый номер", o_str_to(data.registration_number))); 
        content.push(("Порт приписки", o_str_to(data.port_of_registry)));     
        content.push(("Флаг приписки", o_str_to(data.flag_state)));     
        content.push(("Судовладелец", o_str_to(data.ship_master)));      
        content.push(("Код судовладельца", o_str_to(data.ship_owner_code))); 
        content.push(("Верфь постройки", o_str_to(data.yard_of_build)));   
        content.push(("Место постройки", o_str_to(data.place_of_build)));   
        content.push(("Год постройки", o_i32_to(data.year_of_build)));     
        content.push(("Заводской номер", o_str_to(data.ship_builder_hull_number)));   
        let content = content.into_iter().map(|(v1, v2)| vec![v1.to_owned(), v2]).collect();
        Ok(Self {
            table: Table::new(&header, content),
        })
    }
}
//
impl Content for Ship {
    //
    fn to_string(self) -> Result<String, crate::error::Error> {
        Ok("# Судно\n".to_string() + &self.table.to_string()?)
    }
}
