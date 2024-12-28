use std::collections::HashMap;

use crate::{content::Content, error::Error};

use super::parameters::Parameters;

pub struct Draught {
    table: Parameters,
}
//
impl Draught {
    pub fn from(
        data: &HashMap<i32, (String, String, Option<String>)>,
    ) -> Result<Self, Error> {
        Ok(Self {
            table: Parameters::from(
                data,
            )?,            
        })
    }
}
//
impl Content for Draught {
    //
    fn to_string(self) -> Result<String, crate::error::Error> {
        self.table.to_string()
    }
}
