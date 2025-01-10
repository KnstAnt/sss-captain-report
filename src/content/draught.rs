use std::collections::HashMap;

use crate::{content::Content, db::parameters::ParameterData, error::Error};

use super::Parameters;


pub struct Draught {
    table: Parameters,
}
//
impl Draught {
    pub fn from(
        data: &HashMap<i32, ParameterData>,
    ) -> Result<Self, Error> {
        Ok(Self {
            table: Parameters::from(
                &[3,4,5,6,7,51,80,81,82,83,84,85,86,87,88,89,90,91,92,93],
                data,
            )?,            
        })
    }
    //
    pub fn to_string(self) -> Result<String, Error> {
        Ok("# Параметры посадки\n\n".to_string() + &self.table.to_string()?)
    }
}
