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
    pub fn from(data: &[ContainerData]) -> Result<Self, Error> {
        let header = vec!["Наименование", "BBRRTT", "Масса", "$x_g$ [м]", "$y_g$ [м]", "$z_g$ [м]"];
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
        Ok(Self {
            table: Table::new(&header, content),
        })
    }
}
//
impl Content for Container {
    //
    fn to_string(self) -> Result<String, crate::error::Error> {
        self.table.to_string()
    }
}

