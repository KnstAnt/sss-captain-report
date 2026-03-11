use super::Content;
use crate::{
    content::{load_line::draught::Draught, stability::criterion::Criterion},
    db::{criterion::CriteriaData, parameters::ParameterData},
};
use sal_core::{dbg::Dbg, error::Error};
use std::collections::HashMap;
pub mod draught;

pub struct LoadLine {
    dbg: Dbg,
    language: String,
    draught: Draught,
    load_line: Criterion,
}
//
impl LoadLine {
    pub fn new(
        dbg: Dbg,
        language: &str,
        draught: Draught,
        load_line: Criterion,
    ) -> Self {
        Self {
            dbg,
            language: language.to_owned(),
            draught,
            load_line,
        }
    }
    //
    pub fn from(
        parent: &Dbg,
        language: &str,
        parameters: &HashMap<i32, ParameterData>,
        criteria: &[(i32, CriteriaData)],
    ) -> Self {
        let dbg = Dbg::new(parent, "Stability");
        Self::new(
            dbg.clone(),
            language,
            Draught::from(language, parameters),
            Criterion::from(language, criteria),
        )
    }
    //
    pub fn to_string(self) -> Result<String, Error> {
        if self.language.contains("en") {
            Ok("# Load Line".to_string()
                + "## Drafts\n\nAll drafts are moulded.\n\n"
                + &self.draught.to_string()?
                + "## Load line criterion\n\n"
                + &self.load_line.to_string()?)
        } else {
            Ok("# Посадка\n\n".to_string()
                + "## Параметры посадки\n\nОсадки приведены по теоретической поверхности корпуса.\n\n" + &self.draught.to_string()?
                + "## Грузовая марка\n\n" + &self.load_line.to_string()?)
        }
    }
}
