use crate::{
    content::{misc::Table, Content},
    db::itinerary::ItineraryData,
};
use sal_core::error::Error;

pub struct Itinerary {
    header: String,
    table: Table,
}
//
impl Itinerary {
    //
    pub fn new(header: String, table: Table) -> Self {
        Self {header, table}
    }
    //
    pub fn from(language: &String, data: Vec<ItineraryData>) -> Self {
        if language.contains("en") {
            Self::from_en(data)
        } else {
            Self::from_ru(data)
        }
    }
    //
    pub fn from_ru(data: Vec<ItineraryData>) -> Self {
        let header = "# Маршрут\n\n".to_owned();
        let table_header = vec!["Порт", "Код порта", "ETA", "ETD", "Макс. осадка [м]"];
        let content = data
            .into_iter()
            .map(|v| {
                vec![
                    v.port_name.clone(),
                    v.port_code.clone(),
                    v.eta.clone(),
                    v.etd.clone(),
                    format!("{:.3}", v.max_draught),
                ]
            })
            .collect();
        Self::new(header, Table::new(&table_header, content))
    }
    //
    pub fn from_en(data: Vec<ItineraryData>) -> Self {
        let header = "# Itinerary\n\n".to_owned();
        let table_header = vec!["Port", "Port code", "ETA", "ETD", "Max. draft [m]"];
        let content = data
            .into_iter()
            .map(|v| {
                vec![
                    v.port_name.clone(),
                    v.port_code.clone(),
                    v.eta.clone(),
                    v.etd.clone(),
                    format!("{:.3}", v.max_draught),
                ]
            })
            .collect();
        Self::new(header, Table::new(&table_header, content))
    }
}
//
impl Content for Itinerary {
    //
    fn to_string(self) -> Result<String, Error> {
        Ok(self.header + &self.table.to_string())
    }
}
