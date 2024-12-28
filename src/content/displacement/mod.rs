use std::collections::HashMap;
use crate::error::Error;
use super::Content;

pub mod tank;
pub mod cargo;
pub mod bulkhead;
pub mod bulk_cargo;
pub mod container;

pub struct Displacement {
    summary: Parameters,
    ballast_tank: Tank,
    stores_tank: Tank,
    stores: Cargo,
    bulkhead: Bulkhead,
    bulk_cargo: BulkCargo,
    container: Container,
    general_cargo: Cargo,
}
//
impl Displacement {
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
