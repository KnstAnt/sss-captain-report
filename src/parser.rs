//! Класс-коллекция таблиц. Проверяет данные и выполняет их запись
use crate::content::load_line::LoadLine;
use crate::converter::comrak_convert::ComrakConvert;
use crate::db::api::{ApiClient, Db};
use crate::db::bulk_cargo::BulkCargoData;
use crate::db::bulkhead::BulkheadData;
use crate::db::cargo::CargoData;
use crate::db::container::ContainerData;
use crate::db::criterion::CriteriaData;
use crate::db::itinerary::ItineraryData;
use crate::db::parameters::ParameterData;
use crate::db::ship::ShipData;
use crate::db::strength_result::StrengthResultData;
use crate::db::tank::TankData;
use crate::db::voyage::VoyageData;
use sal_core::dbg::Dbg;
use sal_core::error::Error;
use std::collections::HashMap;
use std::path::PathBuf;
//
pub struct Report {
    dbg: Dbg,
    db: Db,
    language: String,
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
    strength_result: Vec<StrengthResultData>, //x, SF, BM
    dso: Vec<(f64, f64)>,
    ddo: Vec<(f64, f64)>,
    criteria: Vec<(i32, CriteriaData)>,
    load_line: Vec<(i32, CriteriaData)>,
    parameters: HashMap<i32, ParameterData>,
}
//
impl Report {
    //
    pub fn new(
        parent: &Dbg,
        ship_id: String,
        project_id: String,
        language: String,
        api_client: ApiClient,
    ) -> Self {
        let dbg = Dbg::new(parent, "Report");
        Self {
            dbg: dbg.clone(),
            db: Db::new(&dbg, ship_id, project_id, language.to_owned(), api_client),
            language,
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
            dso: Vec::new(),
            ddo: Vec::new(),
            criteria: Vec::new(),
            load_line: Vec::new(),
            parameters: HashMap::new(),
        }
    }
    //
    pub fn get_from_db(&mut self) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "get_from_db");
        let ship = self.db.get_ship()?;
        self.imo = ship.imo.clone();
        self.ship = Some(ship);
        let voyage = self.db.get_voyage().map_err(|err| error.pass(err))?;
        self.voyage = Some(voyage);
        self.itinerary = self
            .db
            .get_itinerary()
            .map_err(|err| error.pass(err))?
            .data();
        self.criteria = self
            .db
            .get_criterion_data()
            .map_err(|err| error.pass(err))?
            .data();
        self.load_line = self
            .db
            .get_criterion_load_line()
            .map_err(|err| error.pass(err))?;
        self.parameters = self
            .db
            .get_parameters_data()
            .map_err(|err| error.pass(err))?
            .data();
        self.ballast_tanks = self
            .db
            .get_ballast_tanks()
            .map_err(|err| error.pass(err))?
            .data();
        self.stores_tanks = self
            .db
            .get_stores_tanks()
            .map_err(|err| error.pass(err))?
            .data();
        self.stores = self.db.get_stores().map_err(|err| error.pass(err))?.data();
        self.bulkheads = self
            .db
            .get_bulkheads()
            .map_err(|err| error.pass(err))?
            .data();
        self.bulk_cargo = self
            .db
            .get_bulk_cargo()
            .map_err(|err| error.pass(err))?
            .data();
        self.container = self
            .db
            .get_container()
            .map_err(|err| error.pass(err))?
            .data();
        self.general_cargo = self
            .db
            .get_general_cargo()
            .map_err(|err| error.pass(err))?
            .data();
        self.strength_result = self
            .db
            .get_strength_result()
            .map_err(|err| error.pass(err))?
            .data;
        (self.dso, self.ddo) = self.db.get_lever_diagram().map_err(|err| error.pass(err))?;
        Ok(())
    }
    //
    pub fn write(self, path: &str, name: &str) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "write");
        log2::info!("Parser write_to_file begin");
        //   let imo = self.imo.ok_or(error.err("Formatter error: no imo!"))?;
        let mut content = crate::content::general::General::new(
            crate::content::general::ship::Ship::from(
                &self.language,
                self.ship
                    .ok_or(error.err("Formatter error: no ship data!"))?,
            ),
            crate::content::general::voyage::Voyage::from(
                "en", //всегда на английском    &self.language,
                self.voyage
                    .ok_or(error.err("Formatter error: no voyage data!"))?,
            ),
            crate::content::general::itinerary::Itinerary::from(&self.language, self.itinerary),
        )
        .to_string()?;
        content += &crate::content::displacement::Displacement::new(
            &self.language,
            crate::content::parameters::Parameters::from(
                &self.language,
                &[2, 32, 56, 12, 1, 52],
                &self.parameters,
            ),
            crate::content::displacement::tank::Tank::from(&self.language, &self.ballast_tanks),
            crate::content::displacement::tank::Tank::from(&self.language, &self.stores_tanks),
            crate::content::displacement::cargo::Cargo::from(&self.language, &self.stores),
            crate::content::displacement::bulkhead::Bulkhead::from(&self.language, &self.bulkheads),
            crate::content::displacement::bulk_cargo::BulkCargo::from(
                &self.language,
                &self.bulk_cargo,
            )
            .map_err(|err| error.pass(err))?,
            crate::content::displacement::container::Container::from(
                &self.language,
                &self.container,
            ),
            crate::content::displacement::cargo::Cargo::from(&self.language, &self.general_cargo),
        )
        .to_string()
        .map_err(|err| error.pass(err))?;
        content += &LoadLine::from(&self.language, &self.parameters, &self.load_line)
            .to_string()
            .map_err(|err| error.pass(err))?;
        content += &crate::content::strength::Strength::from(
            &self.dbg,
            &self.language,
            &self.strength_result,
        )
        .to_string()
        .map_err(|err| error.pass(err))?;
        content += "\n";
        content += &crate::content::stability::Stability::from(
            &self.dbg,
            &self.language,
            &self.criteria,
            &self.parameters,
            &self.dso,
            &self.ddo,
        )?
        .to_string()
        .map_err(|err| error.pass(err))?;

        let src = ("bin/md".to_owned() + "/" + name + ".md").replace("//", "/");
        if let Err(err) = std::fs::write(src.clone(), content) {
            return Err(error.pass_with(
                format!("Parser write error: std::fs::write error, src:{src}"),
                err.to_string(),
            ));
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

        if let Err(err) = std::fs::remove_file(src) {
            return Err(error.pass_with(
                format!("Parser write error: std::fs::remove_file: {error}"),
                err.to_string(),
            ));
        }

        log2::info!("Parser write_to_file end");
        Ok(())
    }
}
