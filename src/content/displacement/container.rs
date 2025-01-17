use crate::{
    content::{misc::Table, Content}, db::container::ContainerData, error::Error
};

pub struct Container {
    table: Table,
}
//
impl Container {
    //
    pub fn new(table: Table) -> Self {
        Self { table }
    }
    //
    pub fn from(language: &String, data: &[ContainerData]) -> Result<Self, Error> {
        let header = if language.contains("en") { 
            vec!["Name", "BBRRTT", "Weight", "$x_g$ [m]", "$y_g$ [m]", "$z_g$ [m]",]
        } else {
            vec!["Наименование", "BBRRTT", "Масса", "$x_g$ [м]", "$y_g$ [м]", "$z_g$ [м]"]
        }.to_owned();
        let content = data
            .iter()
            .map(|v| {
                vec!(
                    v.name(), 
                    v.bbrrtt(),
                    format!("{:.3}", v.mass), 
                    format!("{:.3}", v.x_g), 
                    format!("{:.3}", v.y_g),  
                    format!("{:.3}", v.z_g), 
                )
            })
            .collect::<Vec<Vec<String>>>();
        Ok(Self::new(Table::new(&header, content)))
    }
}
//
impl Content for Container {
    //
    fn to_string(self) -> Result<String, crate::error::Error> {
        self.table.to_string()
    }
}

