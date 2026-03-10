use super::{Content, Parameters};
use crate::db::{criterion::CriteriaData, parameters::ParameterData};
use criterion::Criterion;
use lever_diagram::LeverDiagram;
use sal_core::{dbg::Dbg, error::Error};
use std::collections::HashMap;

pub mod chart;
pub mod chart_bulk;
pub mod chart_dso;
pub mod chart_k;
pub mod criterion;
pub mod lever_diagram;

pub struct Stability {
    dbg: Dbg,
    language: String,
    criterion: Criterion,
    load_line: Criterion,
    lever_diagram: LeverDiagram,
    parameters: Parameters,
}
//
impl Stability {
    pub fn new(
        dbg: Dbg,
        language: &str,
        criterion: Criterion,
        load_line: Criterion,
        lever_diagram: LeverDiagram,
        parameters: Parameters,
    ) -> Self {
        Self {
            dbg,
            language: language.to_owned(),
            criterion,
            load_line,
            lever_diagram,
            parameters,
        }
    }
    //
    pub fn from(
        parent: &Dbg,
        language: &str,
        criteria: &[(i32, CriteriaData)],
        load_line: &[(i32, CriteriaData)],
        parameters: &HashMap<i32, ParameterData>,
        dso: &[(f64, f64)],
        ddo: &[(f64, f64)],
    ) -> Result<Self, Error> {
        let dbg = Dbg::new(parent, "Stability");
        Ok(Self::new(
            dbg.clone(),
            language,
            Criterion::from(language, criteria),
            Criterion::from(language, load_line),    
            LeverDiagram::new(&dbg, language, dso, ddo, parameters.clone()),
            Parameters::from(
                language,
                &[
                    8, 9, 10, 11, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 33, 34, 35, 36,
                    37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 53, 54, 55, 95,
                ],
                parameters,
            ),
        ))
    }
    //
    pub fn to_string(self) -> Result<String, Error> {
        if self.language.contains("en") {
            Ok("# Stability\n\n".to_string()
                + "## Criterions\n\n" + &self.criterion.to_string()?
                + "## Load line\n\n" + &self.load_line.to_string()?
                + "## Stability curve\n\n" + &self.lever_diagram.to_string()?
                + "## Stability\n\n" + &self.parameters.to_string()?)
        } else {
            Ok("# Остойчивость\n\n".to_string()
                + "## Критерии\n\n" + &self.criterion.to_string()?
                + "## Посадка судна\n\n" + &self.load_line.to_string()?
                + "## Диаграмма статической остойчивости\n\n" + &self.lever_diagram.to_string()?
                + "## Параметры остойчивости\n\n" + &self.parameters.to_string()?)
        }
    }
}
