use std::collections::HashMap;
use sal_core::{dbg::Dbg, error::Error};
use crate::{content::Content, db::parameters::ParameterData};

use super::Parameters;


pub struct Draught {
    dbg: Dbg,
    header: String,
    table: Parameters,
}
//
impl Draught {
    pub fn from(
        parent: &Dbg, 
        language: &String, 
        data: &HashMap<i32, ParameterData>,
    ) -> Result<Self, Error> {
        let dbg = Dbg::new(parent, "Draught");
        let header = if language.contains("en") {
            "# Drafts\n\nAll drafts are moulded.\n\n"
        } else {
            "# Параметры посадки\n\nОсадки приведены по теоретической поверхности корпуса.\n\n"
        }.to_string();
        Ok(Self{
            dbg,
            header,
            table: Parameters::from(
                language, 
                &[3,4,5,6,7,51,80,81,82,83,84,85,86,87,88,89,90,91,92,93],
                data,
            )?,            
        })
    }
    //
    pub fn to_string(self) -> Result<String, Error> {
        Ok(self.header + &self.table.to_string()?)
    }
}
