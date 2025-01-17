
use bulk_cargo::BulkCargo;
use bulkhead::Bulkhead;
use cargo::Cargo;
use container::Container;
use tank::Tank;

use crate::error::Error;
use super::{Content, Parameters};

pub mod tank;
pub mod cargo;
pub mod bulkhead;
pub mod bulk_cargo;
pub mod container;

pub struct Displacement {
    language: String, 
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
        language: &String,
        summary: Parameters,
        ballast_tank: Tank,
        stores_tank: Tank,
        stores: Cargo,
        bulkhead: Bulkhead,
        bulk_cargo: BulkCargo,
        container: Container,
        general_cargo: Cargo,
    ) -> Self {
        Self {
            language: language.to_owned(),
            summary,
            ballast_tank,
            stores_tank,
            stores,
            bulkhead,
            bulk_cargo,
            container,
            general_cargo,
        }
    }
    //
    pub fn to_string(self) -> Result<String, Error> {
        if self.language.contains("en") {
            Ok("# Displacement\n\n".to_string() +
                "## Total\n\n" + &self.summary.to_string()? + 
                "## Ballast tanks\n\n" + &self.ballast_tank.to_string()? + 
                "## Stores tanks\n\n" + &self.stores_tank.to_string()? + 
                "## Stores\n\n" + &self.stores.to_string()? +  
                "## Bulkheads\n\n" + &self.bulkhead.to_string()? +  
                "## Bulk cargo\n\n" + &self.bulk_cargo.to_string()? +  
                "## Containers\n\n" + &self.container.to_string()? +  
                "## General cargo\n\n" + &self.general_cargo.to_string()?
            )
        } else {
            Ok("# Водоизмещение\n\n".to_string() +
                "## Итого\n\n" + &self.summary.to_string()? + 
                "## Балластные цистерны\n\n" + &self.ballast_tank.to_string()? + 
                "## Цистерны запаса\n\n" + &self.stores_tank.to_string()? + 
                "## Запасы\n\n" + &self.stores.to_string()? +  
                "## Зерновые переборки\n\n" + &self.bulkhead.to_string()? +  
                "## Навалочный груз\n\n" + &self.bulk_cargo.to_string()? +  
                "## Контейнеры\n\n" + &self.container.to_string()? +  
                "## Генеральный груз\n\n" + &self.general_cargo.to_string()?
            )
        }

    }
}
