//! Функции для работы с БД
use super::bulk_cargo::BulkCargoDataArray;
use super::bulkhead::BulkheadDataArray;
use super::cargo::CargoDataArray;
use super::container::ContainerDataArray;
use super::criterion::CriteriaDataArray;
use super::parameters::ParameterDataArray;
use super::stability_diagram::StabilityDiagramDataArray;
use super::strength_result::StrengthResultDataArray;
use super::tank::TankDataArray;
use crate::db::itinerary::ItineraryDataArray;
use crate::db::serde_parser::IFromJson;
use crate::db::ship::{ShipData, ShipDataArray};
use crate::db::voyage::{VoyageData, VoyageDataArray};
use sal_core::{dbg::Dbg, error::Error};

mod client;
pub(crate) use client::*;

pub struct Db {
    dbg: Dbg,
    ship_id: String,
    project_id: String,
    language: String,
    api_client: ApiClient,
}
//
impl Db {
    pub fn new(
        parent: &Dbg,
        ship_id: String,
        project_id: String,
        language: String,
        api_client: ApiClient,
    ) -> Self {
        let dbg = Dbg::new(parent, "Db");
        Self {
            dbg,
            ship_id,
            project_id,
            language,
            api_client,
        }
    }
    /// Чтение данных из БД. Функция читает данные за несколько запросов,
    /// парсит их и проверяет данные на корректность.
    /// Чтение данных из БД. Функция читает данные за несколько запросов,
    /// парсит их и проверяет данные на корректность.
    pub fn get_criterion_data(&mut self) -> Result<CriteriaDataArray, Error> {
        let error = Error::new(&self.dbg, "get_criterion_data");
        CriteriaDataArray::parse(
            &self
                .api_client
                .fetch(&format!(
                    "SELECT 
                    id AS id, \
                    title AS name, \
                    unit AS unit, \
                    result AS result, \
                    target AS target, \
                    state AS state
                FROM 
                    criterion_view
                WHERE 
                    language='{}' AND
                    category_id = 1 AND
                    ship_id={} AND 
                    project_id IS NOT DISTINCT FROM {}
                ORDER BY
                    id;",
                    self.language, self.ship_id, self.project_id,
                ))
                .map_err(|e| error.pass(e))?,
        )
        .map_err(|e| error.pass(e))
    }
    //
    pub fn get_parameters_data(&mut self) -> Result<ParameterDataArray, Error> {
        let error = Error::new(&self.dbg, "get_parameters_data");
        ParameterDataArray::parse(
            &self
                .api_client
                .fetch(&format!(
                    "SELECT 
                    id AS id, \
                    title AS name, \
                    result AS result, \
                    unit AS unit
                FROM 
                    parameter_view
                WHERE 
                    language='{}' AND
                    ship_id={} AND 
                    project_id IS NOT DISTINCT FROM {}
                ORDER BY
                    id;",
                    self.language, self.ship_id, self.project_id,
                ))
                .map_err(|e| error.pass(e))?,
        )
        .map_err(|e| error.pass(e))
    }
    //
    pub fn get_ship(&mut self) -> Result<ShipData, Error> {
        let error = Error::new(&self.dbg, "get_ship");
        ShipDataArray::parse(
            &self
                .api_client
                .fetch(&format!(
                    "SELECT
                        name, \
                        call_sign, \
                        IMO as imo, \
                        MMSI as mmsi, \
                        ship_type, \
                        year_of_build, \
                        place_of_build, \
                        yard_of_build, \
                        navigation_area, \
                        classification_society, \
                        registration_number, \
                        port_of_registry, \
                        flag_state, \
                        ship_owner, \
                        ship_owner_code, \
                        ship_builder_name, \
                        ship_builder_hull_number
                    FROM             
                        ship_view
                    WHERE  
                        id = {} AND language='{}';",
                    self.ship_id, self.language,
                ))
                .map_err(|e| error.pass(e))?,
        )
        .map_err(|e| error.pass(e))?
        .data()
        .ok_or(error.err(format!("no data!")))
    }
    //
    pub fn get_strength_result(&mut self) -> Result<StrengthResultDataArray, Error> {
        let error = Error::new(&self.dbg, "get_strength_result");
        let strength_result = StrengthResultDataArray::parse(
            &self
                .api_client
                .fetch(&format!(
                    "SELECT 
                        r.frame_x AS x, \
                        r.value_shear_force AS sf, \
                        r.value_bending_moment AS bm, \
                        r.limit_low_shear_force AS sf_limit_low, \
                        r.limit_high_shear_force AS sf_limit_high, \
                        r.percent_shear_force AS sf_percent, \
                        r.status_shear_force AS sf_status, \
                        r.limit_low_bending_moment AS bm_limit_low, \
                        r.limit_high_bending_moment AS bm_limit_high, \
                        r.percent_bending_moment AS bm_percent, \
                        r.status_bending_moment AS bm_status
                    FROM
                        result_strength_force_and_moment AS r
                    WHERE 
                        r.ship_id={} AND
                        r.project_id IS NOT DISTINCT FROM {}
                    ORDER BY x;",
                    self.ship_id, self.project_id,
                ))
                .map_err(|e| error.pass(e))?,
        )
        .map_err(|e| error.pass(e))?;
        Ok(strength_result)
    }
    //
    pub fn get_lever_diagram(&mut self) -> Result<(Vec<(f64, f64)>, Vec<(f64, f64)>), Error> {
        let error = Error::new(&self.dbg, "get_lever_diagram");
        Ok(StabilityDiagramDataArray::parse(
            &self
                .api_client
                .fetch(&format!(
                    "SELECT 
                        angle, \
                        value_dso, \
                        value_ddo
                    FROM 
                        stability_diagram 
                    WHERE 
                        ship_id={} AND project_id IS NOT DISTINCT FROM {} 
                    ORDER BY angle ASC;",
                    self.ship_id, self.project_id,
                ))
                .map_err(|e| error.pass(e))?,
        )
        .map_err(|e| error.pass(e))?
        .data())
    }

    pub fn get_ballast_tanks(&mut self) -> Result<TankDataArray, Error> {
        let error = Error::new(&self.dbg, "get_ballast_tanks");
        TankDataArray::parse(
            &self
                .api_client
                .fetch(&format!(
                    "SELECT 
                        space_name AS name, \
                        weight AS mass, \
                        centre_of_gravity_x AS x_g, \
                        centre_of_gravity_y AS y_g, \
                        centre_of_gravity_z AS z_g, \
                        trans_moment_of_inertia AS f_sx 
                    FROM 
                        liquid_cargo_view 
                    WHERE 
                        compartment_purpose='ballast_tank' AND language='{}' AND ship_id={} AND project_id IS NOT DISTINCT FROM {};",
                    self.language,
                    self.ship_id,
                    self.project_id,
                ))
                .map_err(|e| {
                    error.pass(e)
                })?,
        )
        .map_err(|e| error.pass(e))
    }
    //
    pub fn get_stores_tanks(&mut self) -> Result<TankDataArray, Error> {
        let error = Error::new(&self.dbg, "get_stores_tanks");
        TankDataArray::parse(
            &self.api_client
                .fetch(&format!(
                    "SELECT 
                        space_name AS name, \
                        weight AS mass, \
                        centre_of_gravity_x AS x_g, \
                        centre_of_gravity_y AS y_g, \
                        centre_of_gravity_z AS z_g, \
                        trans_moment_of_inertia AS f_sx 
                    FROM 
                        liquid_cargo_view 
                    WHERE 
                        assigment_context = 'stores' AND language='{}' AND ship_id={} AND project_id IS NOT DISTINCT FROM {};",
                    self.language,
                    self.ship_id,
                    self.project_id,
                ))
                .map_err(|e| error.pass(e))?,
        )
        .map_err(|e| error.pass(e))
    }
    //
    pub fn get_stores(&mut self) -> Result<CargoDataArray, Error> {
        let error = Error::new(&self.dbg, "get_stores");
        CargoDataArray::parse(
            &self
                .api_client
                .fetch(&format!(
                    "SELECT 
                        cargo_name AS name, \
                        weight AS mass, \
                        centre_of_gravity_x AS x_g, \
                        centre_of_gravity_y AS y_g, \
                        centre_of_gravity_z AS z_g
                    FROM 
                        unit_cargo_view 
                    WHERE 
                        assigment_context = 'stores' AND cargo_type != 'grain_bulkhead' AND language='{}' AND ship_id={} AND project_id IS NOT DISTINCT FROM {};",
                    self.language,
                    self.ship_id,
                    self.project_id,
                ))
                .map_err(|e| error.pass(e))?,
        )
        .map_err(|e| error.pass(e))
    }
    //
    pub fn get_bulkheads(&mut self) -> Result<BulkheadDataArray, Error> {
        let error = Error::new(&self.dbg, "get_bulkheads");
        BulkheadDataArray::parse(
            &self
                .api_client
                .fetch(&format!(
                    "SELECT 
                        name AS name, \
                        space_name AS position, \
                        weight AS mass, \
                        centre_of_gravity_x AS x_g, \
                        centre_of_gravity_y AS y_g, \
                        centre_of_gravity_z AS z_g
                    FROM 
                        bulkhead_view
                    WHERE 
                        language='{}' AND ship_id={} AND project_id IS NOT DISTINCT FROM {};",
                    self.language, self.ship_id, self.project_id,
                ))
                .map_err(|e| error.pass(e))?,
        )
        .map_err(|e| error.pass(e))
    }
    //
    pub fn get_bulk_cargo(&mut self) -> Result<BulkCargoDataArray, Error> {
        let error = Error::new(&self.dbg, "get_bulk_cargo");
        BulkCargoDataArray::parse(
            &self
                .api_client
                .fetch(&format!(
                    "SELECT 
                        cargo_name AS name, \
                        weight AS mass, \
                        centre_of_gravity_x AS x_g, \
                        centre_of_gravity_y AS y_g, \
                        centre_of_gravity_z AS z_g, \
                        allocated_shifting_moment AS grain_moment
                    FROM 
                        bulk_cargo_view 
                    WHERE 
                        language='{}' AND ship_id={} AND project_id IS NOT DISTINCT FROM {};",
                    self.language, self.ship_id, self.project_id,
                ))
                .map_err(|e| error.pass(e))?,
        )
        .map_err(|e| error.pass(e))
    }
    //
    pub fn get_container(&mut self) -> Result<ContainerDataArray, Error> {
        let error = Error::new(&self.dbg, "get_container");
        ContainerDataArray::parse(
            &self
                .api_client
                .fetch(&format!(
                    "SELECT 
                        owner_code, \
                        serial_number, \
                        check_digit, \
                        bay_number, \
                        row_number, \
                        tier_number, \
                        weight AS mass, \
                        centre_of_gravity_x AS x_g, \
                        centre_of_gravity_y AS y_g, \
                        centre_of_gravity_z AS z_g
                    FROM 
                        container_cargo_view
                    WHERE 
                        language='{}' AND ship_id={} AND project_id IS NOT DISTINCT FROM {};",
                    self.language, self.ship_id, self.project_id,
                ))
                .map_err(|e| error.pass(e))?,
        )
        .map_err(|e| error.pass(e))
    }
    //
    pub fn get_general_cargo(&mut self) -> Result<CargoDataArray, Error> {
        let error = Error::new(&self.dbg, "get_general_cargo");
        CargoDataArray::parse(
            &self
                .api_client
                .fetch(&format!(
                    "SELECT 
                        cargo_name AS name, \
                        weight AS mass, \
                        centre_of_gravity_x AS x_g, \
                        centre_of_gravity_y AS y_g, \
                        centre_of_gravity_z AS z_g
                    FROM 
                        unit_cargo_view 
                    WHERE 
                        assigment_context = 'cargo_load' AND language='{}' AND ship_id={} AND project_id IS NOT DISTINCT FROM {};",
                    self.language, self.ship_id, self.project_id,
                ))
                .map_err(|e| error.pass(e))?,
        )
        .map_err(|e| error.pass(e))
    }
    //
    pub fn get_voyage(&mut self) -> Result<VoyageData, Error> {
        let error = Error::new(&self.dbg, "get_voyage");
        VoyageDataArray::parse(
            &self
                .api_client
                .fetch(&format!(
                    "SELECT
                    v.code as code, \
                    v.density as density, \
                    v.wetting_timber as wetting, \
                    i.icing_type as icing, \
                    a.name AS area, \
                    v.description AS description, \
                    llt.name as load_line 
                FROM 
                    voyage as v
                JOIN 
                    ship_icing AS i ON v.icing_type_id = i.id
                JOIN 
                    ship_water_area AS a ON v.water_area_id = a.id
                JOIN ship_available_load_line_types AS sallt ON
                    sallt.ship_id = v.ship_id AND
                    sallt.project_id IS NOT DISTINCT FROM v.project_id
                JOIN load_line_type AS llt ON
                    sallt.load_line_type_id = llt.id
                WHERE 
                    sallt.is_active IS TRUE AND
                    v.ship_id={} AND 
                    v.project_id IS NOT DISTINCT FROM {}
                LIMIT 1;",
                    self.ship_id, self.project_id,
                ))
                .map_err(|e| error.pass(e))?,
        )
        .map_err(|e| error.pass(e))?
        .data()
        .ok_or(error.err(format!("api_server get_voyage error: no data!")))
    }
    //
    pub fn get_itinerary(&mut self) -> Result<ItineraryDataArray, Error> {
        let error = Error::new(&self.dbg, "get_itinerary");
        ItineraryDataArray::parse(
            &self
                .api_client
                .fetch(&format!(
                    "SELECT
                    p.{} AS port_name, \
                    p.port_code AS port_code, \
                    w.eta AS eta, \
                    w.etd AS etd, \
                    w.max_draught AS max_draught
                FROM 
                    waypoint AS w
                JOIN 
                    port AS p ON w.port_id = p.id
                WHERE 
                    w.ship_id={} AND w.project_id IS NOT DISTINCT FROM {}
                ORDER BY eta ASC;",
                    self.language, self.ship_id, self.project_id,
                ))
                .map_err(|e| error.pass(e))?
        )
        .map_err(|e| error.pass(e))
    }
   /// Чтение данных из БД. Функция читает данные за несколько запросов,
    /// парсит их и проверяет данные на корректность.
    pub fn get_criterion_load_line(&mut self) -> Result<CriteriaDataArray, Error> {
        let error = Error::new(&self.dbg, "get_criterion_load_line");
        CriteriaDataArray::parse(
                        &self
                .api_client
                .fetch(&format!(
                "SELECT 
                    id AS id, \
                    title AS name, \
                    unit AS unit, \
                    result AS result, \
                    target AS target, \
                    state AS state
                FROM 
                    criterion_view
                WHERE 
                    language='{}' AND
                    category_id = 1 AND
                    ship_id={} AND 
                    project_id IS NOT DISTINCT FROM {}
                ORDER BY
                    id;",
                    self.language, self.ship_id, self.project_id,
                )).map_err(|e| error.pass(e))?,
        ).map_err(|e| error.pass(e))
    }    
}
