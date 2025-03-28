use std::collections::HashMap;
use criterion::Criterion;
use lever_diagram::LeverDiagram;
use crate::{db::{criterion::CriteriaData, parameters::ParameterData}, error::Error};
use super::{Content, Parameters};

pub mod lever_diagram;
pub mod criterion;
pub mod chart_dso;
pub mod chart_bulk;
pub mod chart_k;
pub mod chart;

pub struct Stability {
    language: String,
    criterion: Criterion,
    lever_diagram: LeverDiagram,
    parameters: Parameters,
}
//
impl Stability {
    pub fn new(    
        language: &String,
        criterion: Criterion,
        lever_diagram: LeverDiagram,
        parameters: Parameters,
    ) -> Self {
        Self {
            language: language.clone(),
            criterion,
            lever_diagram,
            parameters,
        }
    }
    //
    pub fn from(
        language: &String,
        criteria: &[(i32, CriteriaData)],  
        parameters: &HashMap<i32, ParameterData>,
        dso: &[(f64, f64)],
        ddo: &[(f64, f64)],
    ) -> Result<Self, Error> {
        Ok(Self::new(
            language,
            Criterion::from(
                language,
                criteria,
            )?,
            LeverDiagram::new(
                language,
                dso,
                ddo,
                parameters.clone(),
            ),
            Parameters::from(
                language,
                &[8,9,10,11,13,14,15,16,17,18,19,20,21,22,23,24,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,53,54,55,95],
                parameters,
            )?,
        ))
    }
    //
    pub fn to_string(self) -> Result<String, Error> {
        if self.language.contains("en") {
            Ok("# Stability\n\n".to_string() + 
                "## Criterions\n\n" + 
                &self.criterion.to_string()? +  
                "## Stability curve\n\n" +
                &self.lever_diagram.to_string()?  + "\n" +  
                "## Stability\n\n" + 
                &self.parameters.to_string()?
            )
        } else  {
            Ok("# Остойчивость\n\n".to_string() + 
                "## Критерии\n\n" + 
                &self.criterion.to_string()? +  
                "## Диаграмма статической остойчивости\n\n" +
                &self.lever_diagram.to_string()?  + "\n" +  
                "## Параметры остойчивости\n\n" + 
                &self.parameters.to_string()?
            )
        }
    }
}
