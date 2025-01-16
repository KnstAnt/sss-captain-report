//! Функции для работы с АПИ-сервером
use crate::db::serde_parser::IFromJson;
use crate::error::Error;
use api_tools::client::api_query::*;
use api_tools::client::api_request::*;

use super::bulk_cargo::BulkCargoDataArray;
use super::bulkhead::BulkheadDataArray;
use super::cargo::CargoDataArray;
use super::computed_frame::ComputedFrameDataArray;
use super::container::ContainerDataArray;
use super::criterion::CriteriaDataArray;
use super::itinerary::ItineraryDataArray;
use super::parameters::ParameterDataArray;
use super::ship::ShipData;
use super::ship::ShipDataArray;
use super::stability_diagram::StabilityDiagramDataArray;
use super::strength_limit::StrengthLimitDataArray;
use super::strength_result::StrengthResultDataArray;
use super::tank::TankDataArray;
use super::voyage::VoyageData;
use super::voyage::VoyageDataArray;

pub struct ApiServer {
    database: String,
    host: String,
    port: String,
    ship_id: i32,
    project_id: String,
    language: String,
}
//
impl ApiServer {
    pub fn new(
        database: String,
        host: String,
        port: String,
        ship_id: i32,
        project_id: Option<i32>,
        language: Option<String>, // "ru" - russian (default) / "en" - english
    ) -> Self {
        Self {
            database,
            host,
            port,
            ship_id,
            project_id: project_id.map_or("NULL".to_owned(), |v| v.to_string()),
            language: language
                .map_or("ru", |v| if v.contains("en") { "en" } else { "ru" })
                .to_owned(),
        }
    }
    //
    fn language <'a> (&self, ru: &'a str, en: &'a str) -> &'a str {
        if self.language.contains("en") {
            en
        } else {
            ru
        }
    }
    //
    pub fn fetch(&mut self, sql: &str) -> Result<Vec<u8>, Error> {
        let mut request = ApiRequest::new(
            &api_tools::debug::dbg_id::DbgId("parent".to_owned()),
            self.host.clone() + ":" + &mut self.port,
            "auth_token",
            ApiQuery::new(
                ApiQueryKind::Sql(ApiQuerySql::new(self.database.clone(), sql)),
                false,
            ),
            true,
            false,
        );
        request
            .fetch(true)
            .map_err(|e| Error::FromString(format!("ApiServer fetch error: {e}")))
    }
    /// Чтение данных из БД. Функция читает данные за несколько запросов,
    /// парсит их и проверяет данные на корректность.
    pub fn get_criterion_data(&mut self) -> Result<CriteriaDataArray, Error> {
        CriteriaDataArray::parse(
            &self
                .fetch(&format!(
                    "SELECT 
                    head.id AS id, \
                    head.{} as name, \
                    unit.{} as unit, \
                    values.actual_value AS result, \
                    values.limit_value AS target, \
                    values.state as state
                FROM 
                    criterion as head
                JOIN
                    unit as unit on head.unit_id=unit.id
                JOIN
                    criterion_values AS values ON head.id=values.criterion_id
                WHERE 
                    values.ship_id={} AND 
                    head.category_id = 1 AND
                    values.project_id IS NOT DISTINCT FROM {}
                ORDER BY
                    head.id;",
                    self.language("title_rus", "title_eng"),
                    self.language("symbol_rus", "symbol_eng"),
                    self.ship_id, 
                    self.project_id,
                ))
                .map_err(|e| {
                    Error::FromString(format!("api_server get_criterion_data error: {e}"))
                })?,
        )
        .map_err(|e| Error::FromString(format!("api_server get_criterion_data error: {e}")))
    }
    /// Чтение данных из БД. Функция читает данные за несколько запросов,
    /// парсит их и проверяет данные на корректность.
    pub fn get_criterion_load_line(
        &mut self,
        load_line_id: i32,
    ) -> Result<CriteriaDataArray, Error> {
        CriteriaDataArray::parse(
            &self
                .fetch(&format!(
                    "SELECT
                    head.id AS id, \
                    head.{} as name, \
                    unit.{} as unit, \
                    values.actual_value AS result, \
                    values.limit_value AS target, \
                    values.state as state
                FROM
                    criterion AS head
                JOIN criterion_values AS values ON
                    values.criterion_id = head.id
                JOIN unit as unit on head.unit_id = unit.id
                LEFT JOIN load_line_type_criterions AS lltc ON
                    lltc.criterion_id = head.id
                LEFT JOIN ship_available_load_line_types AS sallt ON
                    sallt.load_line_type_id = lltc.load_line_type_id AND
                    sallt.ship_id = values.ship_id
                WHERE
                    values.ship_id = {} AND
                    values.project_id IS NOT DISTINCT FROM {} AND
                    lltc.load_line_type_id = {load_line_id} AND
                    head.category_id = 2 AND
                    (
                        sallt.is_active = TRUE OR
                        sallt.is_active IS NOT DISTINCT FROM NULL
                    )
                ORDER BY 
                    head.id;",
                    self.language("title_rus", "title_eng"),
                    self.language("symbol_rus", "symbol_eng"),
                    self.ship_id, 
                    self.project_id,
                ))
                .map_err(|e| {
                    Error::FromString(format!("api_server get_criterion_data error: {e}"))
                })?,
        )
        .map_err(|e| Error::FromString(format!("api_server get_criterion_data error: {e}")))
    }
    //
    pub fn get_parameters_data(&mut self) -> Result<ParameterDataArray, Error> {
        ParameterDataArray::parse(
            &self
                .fetch(&format!(
                "SELECT 
                    head.id as id, \
                    head.{} as name, \
                    data.result as result, \
                    unit.{} as unit
                FROM 
                    parameter_head as head
                JOIN                
                    parameter_data as data on data.parameter_id=head.id
                JOIN
                    unit as unit on head.unit_id=unit.id
                WHERE 
                    ship_id={} AND project_id IS NOT DISTINCT FROM {}
                ORDER BY
                    head.id;",
                    self.language("title_rus", "title_eng"),
                    self.language("symbol_rus", "symbol_eng"),
                    self.ship_id, 
                    self.project_id,
                ))
                .map_err(|e| {
                    Error::FromString(format!("api_server get_parameters_data error: {e}"))
                })?,
        )
        .map_err(|e| Error::FromString(format!("api_server get_parameters_data error: {e}")))
    }
    //
    pub fn get_ballast_tanks(&mut self) -> Result<TankDataArray, Error> {
        TankDataArray::parse(
            &self
                .fetch(&format!(
                "SELECT 
                    {} as name, \
                    mass, \
                    mass_shift_x as x_g, \
                    mass_shift_y as y_g, \
                    mass_shift_z as z_g, \
                    m_f_s_x as f_sx 
                FROM 
                    compartment 
                WHERE 
                    category_id=2 AND ship_id={} AND project_id IS NOT DISTINCT FROM {}",
                    self.language("name_rus", "name_engl"),
                    self.ship_id, 
                    self.project_id,
                ))
                .map_err(|e| {
                    Error::FromString(format!("api_server get_ballast_tanks error: {e}"))
                })?,
        )
        .map_err(|e| Error::FromString(format!("api_server get_ballast_tanks error: {e}")))
    }
    //
    pub fn get_stores_tanks(&mut self) -> Result<TankDataArray, Error> {
        TankDataArray::parse(
        &self.fetch(&format!(
                "SELECT 
                    {} as name, \
                    mass, \
                    mass_shift_x as x_g, \
                    mass_shift_y as y_g, \
                    mass_shift_z as z_g, \
                    m_f_s_x as f_sx 
                FROM 
                    compartment 
                WHERE 
                    category_id>=3 AND category_id<=8 AND ship_id={} AND project_id IS NOT DISTINCT FROM {}",
                self.language("name_rus", "name_engl"),
                self.ship_id,
                self.project_id,
            ))
            .map_err(|e| Error::FromString(format!("api_server get_stores_tanks error: {e}")))?,
    )
    .map_err(|e| Error::FromString(format!("api_server get_stores_tanks error: {e}")))
    }
    //
    pub fn get_stores(&mut self) -> Result<CargoDataArray, Error> {
        CargoDataArray::parse(
            &self
                .fetch(&format!(
                "SELECT 
                    name as name, \
                    mass, \
                    mass_shift_x as x_g, \
                    mass_shift_y as y_g, \
                    mass_shift_z as z_g
                FROM 
                    cargo 
                WHERE 
                    category_id=9 AND ship_id={} AND project_id IS NOT DISTINCT FROM {}",
                self.ship_id,
                self.project_id,
                ))
                .map_err(|e| Error::FromString(format!("api_server get_stores error: {e}")))?,
        )
        .map_err(|e| Error::FromString(format!("api_server get_stores error: {e}")))
    }
    //
    pub fn get_bulkheads(&mut self) -> Result<BulkheadDataArray, Error> {
        BulkheadDataArray::parse(
            &self
                .fetch(&format!(
                "SELECT 
                    b.{} as name, \
                    bp.{} as position, \
                    b.mass, \
                    bp.mass_shift_x as x_g, \
                    bp.mass_shift_y as y_g, \
                    bp.mass_shift_z as z_g
                FROM 
                    bulkhead as b
                JOIN 
                    bulkhead_place as bp ON b.id = bp.bulkhead_id
                WHERE 
                    b.ship_id={} AND b.project_id IS NOT DISTINCT FROM {}",
                    self.language("name_rus", "name_engl"),
                    self.language("name_rus", "name_engl"),
                    self.ship_id,
                    self.project_id,
                ))
                .map_err(|e| Error::FromString(format!("api_server get_bulkheads error: {e}")))?,
        )
        .map_err(|e| Error::FromString(format!("api_server get_bulkheads error: {e}")))
    }
    //
    pub fn get_bulk_cargo(&mut self) -> Result<BulkCargoDataArray, Error> {
        BulkCargoDataArray::parse(
            &self
                .fetch(&format!(
                "SELECT 
                    {} as name, \
                    mass, \
                    mass_shift_x as x_g, \
                    mass_shift_y as y_g, \
                    mass_shift_z as z_g, \
                    grain_moment
                FROM 
                    hold_compartment 
                WHERE 
                    ship_id={} AND project_id IS NOT DISTINCT FROM {}",
                    self.language("name_rus", "name_engl"),
                    self.ship_id, self.project_id,
                ))
                .map_err(|e| Error::FromString(format!("api_server get_bulk_cargo error: {e}")))?,
        )
        .map_err(|e| Error::FromString(format!("api_server get_bulk_cargo error: {e}")))
    }
    //
    pub fn get_container(&mut self) -> Result<ContainerDataArray, Error> {
        ContainerDataArray::parse(
            &self
                .fetch(&format!(
                "SELECT 
                    c.owner_code as owner_code, \
                    c.serial_code as serial_code, \
                    c.check_digit, \
                    cs.bay_number as bay_number, \
                    cs.row_number as row_number, \
                    cs.tier_number as tier_number, \
                    c.gross_mass as mass, \
                    (cs.bound_x1 + (cs.bound_x2 - cs.bound_x1) / 2) AS x_g, \
                    (cs.bound_y1 + (cs.bound_y2 - cs.bound_y1) / 2) AS y_g, \
                    (cs.bound_z1 + (cs.bound_z2 - cs.bound_z1) / 2) AS z_g
                FROM 
                    container as c
                JOIN 
                    container_slot as cs ON cs.container_id = c.id
                WHERE 
                    c.ship_id={} AND c.project_id IS NOT DISTINCT FROM {}",
                    self.ship_id, self.project_id,
                ))
                .map_err(|e| Error::FromString(format!("api_server get_container error: {e}")))?,
        )
        .map_err(|e| Error::FromString(format!("api_server get_container error: {e}")))
    }
    //
    pub fn get_general_cargo(&mut self) -> Result<CargoDataArray, Error> {
        CargoDataArray::parse(
            &self
                .fetch(&format!(
                "SELECT 
                    name as name, \
                    mass, \
                    mass_shift_x as x_g, \
                    mass_shift_y as y_g, \
                    mass_shift_z as z_g
                FROM 
                    cargo 
                WHERE 
                    category_id=14 AND ship_id={} AND project_id IS NOT DISTINCT FROM {}",
                    self.ship_id, self.project_id,
                ))
                .map_err(|e| {
                    Error::FromString(format!("api_server get_general_cargo error: {e}"))
                })?,
        )
        .map_err(|e| Error::FromString(format!("api_server get_general_cargo error: {e}")))
    }
    //
    pub fn get_strength_result(&mut self) -> Result<Vec<(f64, f64, f64)>, Error> {
        let bounds = ComputedFrameDataArray::parse(
        &self.fetch(&format!(
                "SELECT index, start_x, end_x FROM computed_frame_space WHERE ship_id={} ORDER BY index;",
                self.ship_id
            ))
            .map_err(|e| Error::FromString(format!("api_server get_strength_result bounds error: {e}")))?,
    )
    .map_err(|e| Error::FromString(format!("api_server get_strength_result bounds error: {e}")))?;
        let strength_result = StrengthResultDataArray::parse(
            &self
                .fetch(&format!(
                    "SELECT 
                    value_shear_force as sf, \
                    value_bending_moment as bm
                FROM
                    result_strength
                WHERE 
                    ship_id={} AND
                    project_id IS NOT DISTINCT FROM {}
                ORDER BY index;",
                    self.ship_id, self.project_id,
                ))
                .map_err(|e| {
                    Error::FromString(format!(
                        "api_server get_strength_result strength_result error: {e}"
                    ))
                })?,
        )
        .map_err(|e| {
            Error::FromString(format!(
                "api_server get_strength_result strength_result error: {e}"
            ))
        })?;
        Ok(bounds
            .data()
            .iter()
            .zip(strength_result.data().iter())
            .map(|(x, (sf, bm))| (*x, *sf, *bm))
            .collect())
    }
    //
    pub fn get_strength_limit(
        &mut self,
        area: &str,
    ) -> Result<Vec<(f64, f64, f64, f64, f64)>, Error> {
        Ok(StrengthLimitDataArray::parse(
            &self
                .fetch(&format!(
                "SELECT 
                    frame_x, \
                    value, \
                    limit_type::TEXT, \
                    limit_area::TEXT, \
                    force_type::TEXT
                FROM 
                    strength_force_limit
                WHERE 
                    ship_id={} AND project_id IS NOT DISTINCT FROM {};",
                    self.ship_id, self.project_id,
                ))
                .map_err(|e| {
                    Error::FromString(format!("api_server get_strength_limit error: {e}"))
                })?,
        )
        .map_err(|e| Error::FromString(format!("api_server get_strength_limit error: {e}")))?
        .data(area))
    }
    //
    pub fn get_lever_diagram(&mut self) -> Result<Vec<(f64, f64)>, Error> {
        Ok(StabilityDiagramDataArray::parse(
            &self
                .fetch(&format!(
                "SELECT 
                    angle, \
                    value_dso
                FROM 
                    stability_diagram 
                WHERE 
                    ship_id={} AND s.project_id IS NOT DISTINCT FROM {};",
                    self.ship_id, self.project_id,
                ))
                .map_err(|e| {
                    Error::FromString(format!("api_server get_lever_diagram error: {e}"))
                })?,
        )
        .map_err(|e| Error::FromString(format!("api_server get_lever_diagram error: {e}")))?
        .data())
    }
    //
    pub fn get_ship(&mut self) -> Result<ShipData, Error> {
        ShipDataArray::parse(
            &self
                .fetch(&format!(
                    "SELECT
                    s.name as name, \
                    s.call_sign as call_sign, \
                    s.IMO as imo, \
                    s.MMSI as mmsi, \
                    tr.title_eng AS ship_type, \
                    s.year_of_built as year_of_build, \
                    s.place_of_built as place_of_build, \
                    s.yard_of_build, \
                    n.area::TEXT AS navigation_area, \
                    s.classification_society, \
                    s.registration_number, \
                    s.port_of_registry, \
                    s.flag_state, \
                    s.ship_owner, \
                    s.ship_owner_code, \
                    s.ship_builder_name, \
                    s.ship_builder_hull_number
                FROM 
                    ship as s
                JOIN 
                    ship_type AS t ON s.ship_type_id = t.id
                JOIN             
                    ship_type_rmrs AS tr ON t.type_rmrs = tr.id
                JOIN
                    navigation_area AS n ON s.navigation_area_id = n.id
                WHERE s.id={} AND s.project_id IS NOT DISTINCT FROM {};",
                    self.ship_id, self.project_id,
                ))
                .map_err(|e| Error::FromString(format!("api_server get_ship error: {e}")))?,
        )
        .map_err(|e| Error::FromString(format!("api_server get_ship error: {e}")))?
        .data()
        .ok_or(Error::FromString(format!(
            "api_server get_ship error: no data!"
        )))
    }
    //
    pub fn get_voyage(&mut self) -> Result<VoyageData, Error> {
        VoyageDataArray::parse(
            &self
                .fetch(&format!(
                "SELECT
                    v.code as code, \
                    v.density as density, \
                    v.wetting_timber as wetting, \
                    i.icing_type as icing, \
                    a.name AS area, \
                    v.description AS description, \
                    t.name as load_line, \
                    v.load_line_id as load_line_id 
                FROM 
                    voyage as v
                JOIN 
                    ship_icing AS i ON v.icing_type_id = i.id
                JOIN 
                    ship_water_area AS a ON v.water_area_id = a.id
                JOIN
                    load_line_type AS t ON v.load_line_id = t.id
                JOIN
                    load_line AS l ON v.load_line_id = l.id
                WHERE v.ship_id={} AND v.project_id IS NOT DISTINCT FROM {};",
                    self.ship_id, self.project_id,
                ))
                .map_err(|e| Error::FromString(format!("api_server get_voyage error: {e}")))?,
        )
        .map_err(|e| Error::FromString(format!("api_server get_voyage error: {e}")))?
        .data()
        .ok_or(Error::FromString(format!(
            "api_server get_voyage error: no data!"
        )))
    }
    //
    pub fn get_itinerary(&mut self) -> Result<ItineraryDataArray, Error> {
        ItineraryDataArray::parse(
            &self
                .fetch(&format!(
                "SELECT
                    port_name, \
                    port_code, \
                    eta, \
                    etd, \
                    max_draught
                FROM 
                    waypoint
                WHERE ship_id={} AND project_id IS NOT DISTINCT FROM {};",
                    self.ship_id, self.project_id,
                ))
                .map_err(|e| Error::FromString(format!("api_server get_itinerary error: {e}")))?,
        )
        .map_err(|e| Error::FromString(format!("api_server get_general error: {e}")))
    }
}
