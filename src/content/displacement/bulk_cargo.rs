use crate::{
    content::{misc::Table, Content}, db::bulk_cargo::BulkCargoData, error::Error
};

pub struct BulkCargo {
    table: Table,
}
//
impl BulkCargo {
    //
    pub fn new(table: Table) -> Self {
        Self { table }
    }
    //
    pub fn from(language: &String, data: &[BulkCargoData]) -> Result<Self, Error> {
        let header = if language.contains("en") { 
            vec!["Name", "Weight", "$x_g$ [m]", "$y_g$ [m]", "$z_g$ [m]", "Grain moment [tm]"]
        } else {
            vec!["Наименование", "Масса", "$x_g$ [м]", "$y_g$ [м]", "$z_g$ [м]", "Кренящий момент от смещения зерна [тм]"]
        }.to_owned();
        let content = data
            .iter()
            .map(|v| {
                vec!(
                    v.name.clone().unwrap_or("-".to_string()), 
                    v.mass.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()),   
                    v.x_g.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()), 
                    v.y_g.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()), 
                    v.z_g.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()), 
                    v.grain_moment.map(|v| format!("{:.3}", v)).unwrap_or("-".to_string()),
                )
            })
            .collect::<Vec<Vec<String>>>();
        Ok(Self::new(Table::new(&header, content)))
    }
}
//
impl Content for BulkCargo {
    //
    fn to_string(self) -> Result<String, crate::error::Error> {
        self.table.to_string()
    }
}
