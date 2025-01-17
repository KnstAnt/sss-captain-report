use crate::{
    content::{misc::Table, Content}, db::cargo::CargoData, error::Error
};

pub struct Cargo {
    table: Table,
}
//
impl Cargo {
    //
    pub fn new(table: Table) -> Self {
        Self { table }
    }
    //
    pub fn from(language: &String, data: &[CargoData]) -> Result<Self, Error> {
        let header = if language.contains("en") { 
            vec!["Name", "Weight", "$x_g$ [m]", "$y_g$ [m]", "$z_g$ [m]",]
        } else {
            vec!["Наименование", "Масса", "$x_g$ [м]", "$y_g$ [м]", "$z_g$ [м]"]
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
                )
            })
            .collect::<Vec<Vec<String>>>();
        Ok(Self::new(Table::new(&header, content)))
    }
}
//
impl Content for Cargo {
    //
    fn to_string(self) -> Result<String, crate::error::Error> {
        self.table.to_string()
    }
}

