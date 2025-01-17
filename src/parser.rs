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
use crate::ApiServer;
use std::collections::HashMap;
use std::path::PathBuf;
//
pub struct Report {
    language: String,
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
    pub fn new(language: Option<String>, api_server: ApiServer) -> Self {
        Self {
            language: language.unwrap_or("ru".to_owned()),
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
        let ship = self.api_server.get_ship()?;
        self.imo = ship.imo.clone();
        self.ship = Some(ship);
        let voyage = self.api_server.get_voyage()?;
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
            self.api_server.get_itinerary()?.data();
        self.criteria =
            self.api_server.get_criterion_data()?.data();
        self.criteria.append(
            &mut self.api_server.get_criterion_load_line(load_line_id)?
            .data(),
        );
        self.criteria.sort_by(|a, b| a.0.cmp(&b.0) );
        self.parameters =
            self.api_server.get_parameters_data()?.data();
        self.ballast_tanks =
            self.api_server.get_ballast_tanks()?.data();
        self.stores_tanks =
            self.api_server.get_stores_tanks()?.data();
        self.stores = self.api_server.get_stores()?.data();
        self.bulkheads =
            self.api_server.get_bulkheads()?.data();
        self.bulk_cargo =
            self.api_server.get_bulk_cargo()?.data();
        self.container =
            self.api_server.get_container()?.data();
        self.general_cargo =
            self.api_server.get_general_cargo()?.data();
        self.strength_result =
            self.api_server.get_strength_result()?;
        self.strength_limit =
            self.api_server.get_strength_limit(area)?;
        self.lever_diagram =
            self.api_server.get_lever_diagram()?;
        Ok(())
    }
    //
    pub fn write(self, path: &str, name: &str) -> Result<(), Error> {
        log2::info!("Parser write_to_file begin");
        let imo = self
            .imo
            .ok_or(Error::FromString("Formatter error: no imo!".to_owned()))?;
        let mut content = /*if self.language.contains("ru") {
            Title::new(&format!("Судно, предназначенное для перевозки сухих генеральных грузов София (IMO№ {imo})")).print_ru() 
        } else {
            Title::new(&format!("General dry cargo ship Sofia (IMO№ {imo})")).print_en() 
        };
        content += "\n\n";
        content +=*/ crate::content::general::General::new(
            crate::content::general::ship::Ship::from(&self.language, self.ship.ok_or(Error::FromString(
                "Formatter error: no ship data!".to_owned(),
            ))?)?,
            crate::content::general::voyage::Voyage::from(&self.language, self.voyage.ok_or(
                Error::FromString("Formatter error: no voyage data!".to_owned()),
            )?)?,
            crate::content::general::itinerary::Itinerary::from(&self.language, self.itinerary)?,
        )
        .to_string()?;
        content += &crate::content::displacement::Displacement::new(
            &self.language,
            crate::content::parameters::Parameters::from(
                &self.language,
                &[2, 32, 56, 12, 1, 52],
                &self.parameters,
            )?,
            crate::content::displacement::tank::Tank::from(
                &self.language,
                &self.ballast_tanks
            )?,
            crate::content::displacement::tank::Tank::from(
                &self.language,
                &self.stores_tanks
            )?,
            crate::content::displacement::cargo::Cargo::from(
                &self.language,
                &self.stores
            )?,
            crate::content::displacement::bulkhead::Bulkhead::from(
                &self.language,
                &self.bulkheads
            )?,
            crate::content::displacement::bulk_cargo::BulkCargo::from(
                &self.language,
                &self.bulk_cargo
            )?,
            crate::content::displacement::container::Container::from(
                &self.language,
                &self.container
            )?,
            crate::content::displacement::cargo::Cargo::from(
                &self.language,
                &self.general_cargo
            )?,
        )
        .to_string()?;
        content += &crate::content::draught::Draught::from(
            &self.language,
            &self.parameters
        )?.to_string()?;
        content +=
            &crate::content::strength::Strength::from(
                &self.language,
                &self.strength_result, 
                &self.strength_limit
            )
                .to_string()?;
        content += "\n";
        content += &crate::content::stability::Stability::from(
            &self.language,
            &self.criteria,
            &self.parameters,
            &self.lever_diagram,
        )?
        .to_string()?;

        let src = ("bin/md".to_owned() + "/" + name + ".md").replace("//", "/");
        if let Err(error) = std::fs::write(src.clone(), content) {
            return Err(Error::FromString(format!("Parser write error: std::fs::write error: {error}, src:{src}")));
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        log2::info!("Parser write md ok");

        let assets = PathBuf::from("bin/assets");
        let output = (path.to_owned() + "/" + name).replace("//", "/");
        let output = PathBuf::from(output);
        let src = PathBuf::from(src);
        let template = if self.language.contains("en") { 
            PathBuf::from("bin/assets/template_en.html")
        } else {
            PathBuf::from("bin/assets/template_ru.html")
        };
        ComrakConvert::new(&src, &output, assets, template).convert();
        log2::info!("Parser write html ok");

    /*    if let Err(error) = std::fs::remove_file(src) {
            return Err(Error::FromString(format!("Parser write error: std::fs::remove_file: {error}")));
        }
*/
        log2::info!("Parser write_to_file end");
        Ok(())
    }
}
