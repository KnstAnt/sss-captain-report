use std::collections::HashMap;
use criterion::Criterion;
use lever_diagram::LeverDiagram;
use parameters::Parameters;
use crate::error::Error;
use super::Content;

pub mod lever_diagram;
pub mod draught;
pub mod parameters;
pub mod criterion;

pub struct Stability {
    criterion: Criterion,
    lever_diagram: LeverDiagram,
    parameters: Parameters,
}
//
impl Stability {
    pub fn new(    
        criterion: Criterion,
        lever_diagram: LeverDiagram,
        parameters: Parameters,
    ) -> Self {
        Self {
            criterion,
            lever_diagram,
            parameters,
        }
    }
    //
    pub fn new_named(
        criteria: &HashMap<i32, (String, String, Option<String>, Option<String>, Option<String>)>, // id, value        
        parameters: &HashMap<i32, (String, String, Option<String>)>, // id, value  
        lever_diagram: &[(f64, f64)],
    ) -> Result<Self, Error> {
        Ok(Self::new(
            Criterion::from(
                criteria,
            )?,
            LeverDiagram::new(
                lever_diagram,
            ),
            Parameters::from(
                parameters,
            )?,
        ))
    }
    //
    pub fn to_string(self) -> Result<String, Error> {
        Ok("# Остойчивость\n".to_string() + 
            "## Критерии\n" + 
            &self.criterion.to_string()? + "\n" + 
            "## Диаграмма статической остойчивости\n" +
            &self.lever_diagram.to_string()? + "\n" + 
            "## Параметры остойчивости\n" + 
            &self.parameters.to_string()?
        )
    }
}
