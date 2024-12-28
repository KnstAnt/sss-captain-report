use crate::{
    content::{misc::Table, Content},
    db::itinerary::ItineraryData,
    error::Error,
};

pub struct Itinerary {
    table: Table,
}
//
impl Itinerary {
    //
    pub fn new(table: Table) -> Self {
        Self { table }
    }
    //
    pub fn from(data: Vec<ItineraryData>) -> Result<Self, Error> {
        let header = vec!["Порт", "Код порта", "ETA", "ETD", "Макс. осадка [м]"];
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
        Ok(Self {
            table: Table::new(&header, content),
        })
    }
}
//
impl Content for Itinerary {
    //
    fn to_string(self) -> Result<String, crate::error::Error> {
        Ok("# Рейс\n".to_string() + &self.table.to_string()?)
    }
}
