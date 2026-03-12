use std::collections::HashMap;
use sal_core::{ error::Error};
use crate::{content::{Content, Parameters}, db::parameters::ParameterData};


pub struct Draught {
    table: Parameters,
}
//
impl Draught {
    pub fn from(
        language: &str, 
        data: &HashMap<i32, ParameterData>,
    ) -> Self {
        Self{
            table: Parameters::from(
                language, 
                &[3,4,5,6,7,51,80,81,82,83,84,85,86,87,88,89,90,91,92,93],
                data,
            ),            
        }
    }
    //
    pub fn to_string(self) -> Result<String, Error> {
        Ok(self.table.to_string()?)
    }
}
