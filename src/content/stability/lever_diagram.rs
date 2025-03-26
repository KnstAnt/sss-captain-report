use std::collections::HashMap;

use crate::{
    content::misc::{Curve, ICurve},
    db::parameters::ParameterData,
};

pub struct LeverDiagram {
    header: String,
    dso_name: String,
    bulk_name: String, 
    wheather_name: String,
    unit: String,
    dso: Vec<(f64, f64)>,
    ddo: Vec<(f64, f64)>,
    parameters: HashMap<i32, ParameterData>,
}
//
impl LeverDiagram {
    //
    pub fn new(
        language: &String,
        dso: &[(f64, f64)],
        ddo: &[(f64, f64)],
        parameters: HashMap<i32, ParameterData>,
    ) -> Self {
        let (header, dso_name, bulk_name, wheather_name, unit) = if language.contains("en") {
            ("| Heel | Lever |", "DSO", "Bulk", "Wheather","m")
        } else {
            ("| Крен | Плечо расчет |", "ДСО", "Зерно", "Погода", "м")
        };
        Self {
            header: header.to_owned(),
            dso_name: dso_name.to_owned(),
            bulk_name: bulk_name.to_owned(),
            wheather_name: wheather_name.to_owned(),
            unit: unit.to_owned(),
            dso: Vec::from(dso),
            ddo: Vec::from(ddo),
            parameters,
        }
    }
    //
    pub fn to_string(self) -> Result<String, crate::error::Error> {
        let mut string = self.header.clone() + "\n|---|---|\n";
        let dso_curve = Curve::new_linear(&self.dso).map_err(|e| {
            format!(
                "LeverDiagram to_string dso_curve error:{}, src:{:?}",
                e, &self.dso
            )
        })?;
        let ddo_curve = Curve::new_linear(&self.ddo).map_err(|e| {
            format!(
                "LeverDiagram to_string ddo_curve error:{}, src:{:?}",
                e, &self.dso
            )
        })?;
        let keys = (0..60).filter(|v| *v as f64 % 5. == 0.).collect::<Vec<_>>();
        let mut ddo = Vec::new();
        let mut dso = Vec::new();
        for key in keys {
            let key = key as f64;
            let dso_value = dso_curve
                .value(key)
                .map_err(|e| format!("LeverDiagram to_string dso_curve value error:{}", e))?;
            dso.push((key, dso_value));
            let ddo_value = ddo_curve
                .value(key)
                .map_err(|e| format!("LeverDiagram to_string ddo_curve value error:{}", e))?;
            ddo.push((key, ddo_value));
            string += &format!("|{}|{:.3}|\n", key, dso_value);
        }
        let mut h = Vec::new();
        if let (Some(theta0), Some(data)) = (self.parameters.get(&7), self.parameters.get(&18)) {
            let theta0 = theta0.result.unwrap_or(0.);
            let result = data.result.unwrap_or(0.);
            h.push((theta0, 0.));
            h.push((theta0 + 7.3, result));
        }
        match super::chart_dso::ChartDSO::new(&self.dso_name, &self.unit, &dso, &ddo, &h)
            .to_string()
        {
            Ok(_) => (),
            Err(error) => log2::error!("LeverDiagram to_string ChartDSO error: {error}"),
        };
        let a = self.parameters.get(&99);
        let p1_dso = (self.parameters.get(&101), self.parameters.get(&103));
        let b = (self.parameters.get(&49), self.parameters.get(&102));
        let p_40_dso = self.parameters.get(&100);
        if let (Some(a), (Some(p1_dso_0), Some(p1_dso_1)), (Some(b_0), Some(b_1)), Some(p_40_dso)) =
            (a, p1_dso, b, p_40_dso)
        {
            if let (Some(a), Some(p1_dso_0), Some(p1_dso_1), Some(b_0), Some(b_1), Some(p_40_dso)) = (
                a.result,
                p1_dso_0.result,
                p1_dso_1.result,
                b_0.result,
                b_1.result,
                p_40_dso.result,
            ) {
                match super::chart_bulk::ChartBulk::new(
                    &self.bulk_name,
                    &self.unit,
                    &dso,
                    a,
                    (p1_dso_0, p1_dso_1),
                    (b_0, b_1),
                    p_40_dso,
                )
                .to_string()
                {
                    Ok(_) => (),
                    Err(error) => log2::error!("LeverDiagram to_string ChartBulk error: {error}"),
                };
            }
        }
        let theta_0 = self.parameters.get(&7);
        let theta_w1 = (self.parameters.get(&38), self.parameters.get(&36));
        let theta_w2 = (self.parameters.get(&39), self.parameters.get(&37));
        let a = (self.parameters.get(&107), self.parameters.get(&106));
        let b = (self.parameters.get(&104), self.parameters.get(&105));
        if let (Some(theta_0), (Some(theta_w1_0), Some(theta_w1_1)), (Some(theta_w2_0), Some(theta_w2_1)), (Some(a_0), Some(a_1)), (Some(b_0), Some(b_1))) =
            (theta_0, theta_w1, theta_w2, a, b)
        {
            if let (Some(theta_0), Some(theta_w1_0), Some(theta_w1_1), Some(theta_w2_0), Some(theta_w2_1), Some(a_0), Some(a_1), Some(b_0), Some(b_1)) = (
                theta_0.result,
                theta_w1_0.result,
                theta_w1_1.result,
                theta_w2_0.result,
                theta_w2_1.result,
                a_0.result,
                a_1.result,
                b_0.result,
                b_1.result,
            ) {
                match super::chart_k::ChartWeather::new(
                    &self.wheather_name,
                    &self.unit,
                    &dso,
                    theta_0,      
                    (theta_w1_0, theta_w1_1), 
                    (theta_w2_0, theta_w2_1), 
                    (a_0, a_1),
                    (b_0, b_1), 
                )
                .to_string()
                {
                    Ok(_) => (),
                    Err(error) => log2::error!("LeverDiagram to_string ChartWeather error: {error}"),
                };
            }
        }
        Ok(format!("![DSO](./assets/dso_chart.svg)") + "\n\n" +
                    "![grain](./assets/bulk_chart.svg)" + "\n\n" +
                    "![wheather](./assets/k_chart.svg)" + "\n\n" + &string)
    }
}
