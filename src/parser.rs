//! Класс-коллекция таблиц. Проверяет данные и выполняет их запись
use crate::content::stability::Stability;
use crate::content::strength::Strength;
use crate::content::Content;
use crate::db::itinerary::ItineraryData;
use crate::db::ship::ShipData;
use crate::error::Error;
use crate::formatter::title::Title;
use crate::formatter::{Formatter, Page};
use crate::ApiServer;
use std::collections::HashMap;
//
pub struct Report {
    ship_id: usize,
    api_server: ApiServer,
    ship: Option<ShipData>,
    itinerary: Vec<ItineraryData>,
    strength_result: Vec<(f64, f64, f64)>,          //x, SF, BM
    strength_limit: Vec<(f64, f64, f64, f64, f64)>, // fr, bm_min, bm_max, sf_min, sf_max
    lever_diagram: Vec<(f64, f64)>,          //angle, level
    criteria: HashMap<i32, f64>,             // criterion_id, value
    parameters: HashMap<i32, f64>,           // parameter_id, value
}
//
impl Report {
    //
    pub fn new(ship_id: usize, api_server: ApiServer) -> Self {
        Self {
            ship_id,
            api_server,
            ship: None,
            itinerary: Vec::new(),
            strength_result: Vec::new(),
            strength_limit: Vec::new(),
            lever_diagram: Vec::new(),
            criteria: HashMap::new(),
            parameters: HashMap::new(),
        }
    }
    //
    pub fn get_from_db(&mut self) -> Result<(), Error> {
        let ship = crate::db::api_server::get_ship(&mut self.api_server, self.ship_id)?;
        let area = if ship.limit_area.contains("sea") {
            "sea"
        } else {
            "harbor"
        };
        self.ship = Some(ship);
        //self.voyage = crate::db::api_server::get_voyage(&mut self.api_server, self.ship_id)?.data();
        self.itinerary = crate::db::api_server::get_itinerary(&mut self.api_server, self.ship_id)?.data();
        self.criteria =
            crate::db::api_server::get_criterion_data(&mut self.api_server, self.ship_id)?.data();
        self.parameters =
            crate::db::api_server::get_parameters_data(&mut self.api_server, self.ship_id)?.data();
        self.strength_result =
            crate::db::api_server::get_strength_result(&mut self.api_server, self.ship_id)?;
        self.strength_limit =
            crate::db::api_server::get_strength_limit(&mut self.api_server, self.ship_id, area)?;
        self.lever_diagram =
            crate::db::api_server::get_lever_diagram(&mut self.api_server, self.ship_id)?;
        Ok(())
    }
    //
    pub fn write(self, path: &str) -> Result<(), Error> {
        println!("Parser write_to_file begin");
        let mut formatter = Formatter::new(Page::new(Title::new("Сухогрузное судно Sofia (IMO№ 555666333)\nРасчет прочности и остойчивости").print(), None));
        formatter.add_page(crate::content::general::General::new(
            crate::content::general::ship::Ship::from(self.ship.ok_or(Error::FromString("Formatter error: no ship data!".to_owned()))?)?,
            crate::content::general::itinerary::Itinerary::from(self.itinerary)?,
        ).to_string()?);
        let mut content = crate::content::stability::displacement::Displacement::from_data(
            &self.parameters,
        )?.to_string().map_err(|e| format!("Parser write Displacement error:{}", e))? + "\n";
        content += &(crate::content::stability::draught::Draught::from(
            &self.parameters,
        )?.to_string().map_err(|e| format!("Parser write Draught error:{}", e))? + "\n");        
        content += &(Strength::new_named(
                &self.strength_result,
                &self.strength_limit,
            ).to_string().map_err(|e| format!("Parser write Strength error:{}", e))? + "\n"); 
        content += &(Stability::new_named(
            &self.criteria,
            &self.parameters,
            &self.lever_diagram,
        )?.to_string().map_err(|e| format!("Parser write Stability error:{}", e))? + "\n"); 
        std::fs::write(format!("{}", path), content).expect("Unable to write {path}");
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("Parser write_to_file end");
        Ok(())
    }
}
