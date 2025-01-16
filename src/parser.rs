//! Класс-коллекция таблиц. Проверяет данные и выполняет их запись
use crate::converter::comrak_convert::ComrakConvert;
use crate::db::bulk_cargo::BulkCargoData;
use crate::db::bulkhead::BulkheadData;
use crate::db::cargo::CargoData;
use crate::db::container::ContainerData;
use crate::db::criterion::CriteriaData;
use crate::db::itinerary::ItineraryData;
use crate::db::parameters::ParameterData;
use crate::db::ship::ShipData;
use crate::db::tank::TankData;
use crate::db::voyage::VoyageData;
use crate::error::Error;
use crate::formatter::title::Title;
//use crate::formatter::{Formatter, Page};
use crate::ApiServer;
use std::collections::HashMap;
use std::path::PathBuf;
//
pub struct Report {
    ship_id: usize,
    api_server: ApiServer,
    imo: Option<i32>,
    ship: Option<ShipData>,
    voyage: Option<VoyageData>,
    itinerary: Vec<ItineraryData>,
    ballast_tanks: Vec<TankData>,
    stores_tanks: Vec<TankData>,
    stores: Vec<CargoData>,
    bulkheads: Vec<BulkheadData>,
    bulk_cargo: Vec<BulkCargoData>,
    container: Vec<ContainerData>,
    general_cargo: Vec<CargoData>,
    strength_result: Vec<(f64, f64, f64)>,          //x, SF, BM
    strength_limit: Vec<(f64, f64, f64, f64, f64)>, //fr, bm_min, bm_max, sf_min, sf_max
    lever_diagram: Vec<(f64, f64)>,                 //angle, level
    criteria: Vec<(i32, CriteriaData)>,
    parameters: HashMap<i32, ParameterData>,
}
//
impl Report {
    //
    pub fn new(ship_id: usize, api_server: ApiServer) -> Self {
        Self {
            ship_id,
            api_server,
            imo: None,
            ship: None,
            voyage: None,
            itinerary: Vec::new(),
            ballast_tanks: Vec::new(),
            stores_tanks: Vec::new(),
            stores: Vec::new(),
            bulkheads: Vec::new(),
            bulk_cargo: Vec::new(),
            container: Vec::new(),
            general_cargo: Vec::new(),
            strength_result: Vec::new(),
            strength_limit: Vec::new(),
            lever_diagram: Vec::new(),
            criteria: Vec::new(),
            parameters: HashMap::new(),
        }
    }
    //
    pub fn get_from_db(&mut self) -> Result<(), Error> {
        let ship = crate::db::api_server::get_ship(&mut self.api_server, self.ship_id)?;
        self.imo = ship.imo.clone();
        self.ship = Some(ship);
        let voyage = crate::db::api_server::get_voyage(&mut self.api_server, self.ship_id)?;
        let area = if voyage
            .area
            .clone()
            .unwrap_or("-".to_owned())
            .contains("harbor")
        {
            "harbor"
        } else {
            "sea"
        };
        let load_line_id = voyage.load_line_id.ok_or(Error::FromString(
            "Formatter get_from_db error: no load_line_id!".to_owned(),
        ))?;
        self.voyage = Some(voyage);
        self.itinerary =
            crate::db::api_server::get_itinerary(&mut self.api_server, self.ship_id)?.data();
        self.criteria =
            crate::db::api_server::get_criterion_data(&mut self.api_server, self.ship_id)?.data();
        self.criteria.append(
            &mut crate::db::api_server::get_criterion_load_line(
                &mut self.api_server,
                self.ship_id,
                load_line_id,
            )?
            .data(),
        );
        self.criteria.sort_by(|a, b| a.0.cmp(&b.0) );
        self.parameters =
            crate::db::api_server::get_parameters_data(&mut self.api_server, self.ship_id)?.data();
        self.ballast_tanks =
            crate::db::api_server::get_ballast_tanks(&mut self.api_server, self.ship_id)?.data();
        self.stores_tanks =
            crate::db::api_server::get_stores_tanks(&mut self.api_server, self.ship_id)?.data();
        self.stores = crate::db::api_server::get_stores(&mut self.api_server, self.ship_id)?.data();
        self.bulkheads =
            crate::db::api_server::get_bulkheads(&mut self.api_server, self.ship_id)?.data();
        self.bulk_cargo =
            crate::db::api_server::get_bulk_cargo(&mut self.api_server, self.ship_id)?.data();
        self.container =
            crate::db::api_server::get_container(&mut self.api_server, self.ship_id)?.data();
        self.general_cargo =
            crate::db::api_server::get_general_cargo(&mut self.api_server, self.ship_id)?.data();
        self.strength_result =
            crate::db::api_server::get_strength_result(&mut self.api_server, self.ship_id)?;
        self.strength_limit =
            crate::db::api_server::get_strength_limit(&mut self.api_server, self.ship_id, area)?;
        self.lever_diagram =
            crate::db::api_server::get_lever_diagram(&mut self.api_server, self.ship_id)?;
        Ok(())
    }
    //
    pub fn write(self, path: &str, name: &str) -> Result<(), Error> {
        println!("Parser write_to_file begin");
        /*
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
        */
        let imo = self
            .imo
            .ok_or(Error::FromString("Formatter error: no imo!".to_owned()))?;
        let mut content =
            Title::new(&format!("Сухогрузное судно Sofia (IMO№ {imo})")).print() + "\n";
        content += &crate::content::general::General::new(
            crate::content::general::ship::Ship::from(self.ship.ok_or(Error::FromString(
                "Formatter error: no ship data!".to_owned(),
            ))?)?,
            crate::content::general::voyage::Voyage::from(self.voyage.ok_or(
                Error::FromString("Formatter error: no voyage data!".to_owned()),
            )?)?,
            crate::content::general::itinerary::Itinerary::from(self.itinerary)?,
        )
        .to_string()?;
        content += "\n\n";
        content += &crate::content::displacement::Displacement::new(
            crate::content::parameters::Parameters::from(
                &[2, 32, 56, 12, 1, 52],
                &self.parameters,
            )?,
            crate::content::displacement::tank::Tank::from(&self.ballast_tanks)?,
            crate::content::displacement::tank::Tank::from(&self.stores_tanks)?,
            crate::content::displacement::cargo::Cargo::from(&self.stores)?,
            crate::content::displacement::bulkhead::Bulkhead::from(&self.bulkheads)?,
            crate::content::displacement::bulk_cargo::BulkCargo::from(&self.bulk_cargo)?,
            crate::content::displacement::container::Container::from(&self.container)?,
            crate::content::displacement::cargo::Cargo::from(&self.general_cargo)?,
        )
        .to_string()?;
        content += "\n\n";
        content += &crate::content::draught::Draught::from(&self.parameters)?.to_string()?;
        content += "\n\n";
        content +=
            &crate::content::strength::Strength::from(&self.strength_result, &self.strength_limit)
                .to_string()?;
        content += "\n\n";
        content += &crate::content::stability::Stability::from(
            &self.criteria,
            &self.parameters,
            &self.lever_diagram,
        )?
        .to_string()?;
        content += "\n\n";

        let src = ("bin/md".to_owned() + "/" + name + ".md").replace("//", "/");
        if let Err(error) = std::fs::write(src.clone(), content) {
            return Err(Error::FromString(format!("Parser write error: std::fs::write error: {error}, src:{src}")));
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("Parser write md ok");

        let assets = PathBuf::from("bin/assets");
        let output = (path.to_owned() + "/" + name).replace("//", "/");
        let output = PathBuf::from(output);
        let src = PathBuf::from(src);
        let template = PathBuf::from("bin/assets/template.html");
        ComrakConvert::new(&src, &output, assets, template).convert();
        println!("Parser write html ok");

        println!("Parser write_to_file end");
        Ok(())
    }
}
