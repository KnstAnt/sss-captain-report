use crate::content::misc::{Curve, ICurve};
use crate::content::Content;
use sal_core::{dbg::Dbg, error::Error};
//
pub struct Template {
    dbg: Dbg,
    language: String,
    short_name: String,
    unit: String,
    result: Vec<(f64, f64)>,     //x, value
    limit: Vec<(f64, f64, f64)>, //x, min, max
}
//
impl Template {
    //
    pub fn new(
        parent: &Dbg, 
        language: &String,
        short_name: &str,
        unit: &str,
        result: &[(f64, f64)],
        limit: &[(f64, f64, f64)],
    ) -> Self {
        let dbg = Dbg::new(parent, "Template");
        Self {
            dbg,
            language: language.to_owned(),
            short_name: short_name.to_owned(),
            unit: unit.to_owned(),
            result: Vec::from(result),
            limit: Vec::from(limit),
        }
    }
}
//
impl Content for Template {
    //
    fn to_string(self) -> Result<String, Error> {
        let (limit_min, limit_max): (Vec<(f64, f64)>, Vec<(f64, f64)>) = self
            .limit
            .into_iter()
            .map(|(x, min, max)| ((x, min), (x, max)))
            .unzip();
        let x_min = self.result.first().unwrap_or(&(0., 0.)).0;
        let x_max = self.result.last().unwrap_or(&(0., 0.)).0;
        let limit_y_min = limit_min.iter().fold(f64::MAX, |r, v| r.min(v.1));
        let limit_y_max = limit_max.iter().fold(f64::MIN, |r, v| r.max(v.1));
        let result_y_min = self.result.iter().fold(f64::MAX, |r, v| r.min(v.1));
        let result_y_max = self.result.iter().fold(f64::MIN, |r, v| r.max(v.1));
        let y_min = limit_y_min.min(result_y_min);
        let y_max = limit_y_max.max(result_y_max);
        let (mult_x, mult_y) = (20., 20.);
        let x_min = (x_min/mult_x).floor()*mult_x;
        let x_max = (x_max/mult_x).ceil()*mult_x;
        let y_min = (y_min/mult_y).floor()*mult_y;
        let y_max = (y_max/mult_y).ceil()*mult_y;

        match super::chart::Chart::new(
            &self.language,
            &self.short_name,
            &self.unit,
            x_min,
            x_max,
            y_min,
            y_max,
            &self.result,
            &limit_min,
            &limit_max,
        )
        .to_string()
        {
            Ok(_) => (),
            Err(error) => log2::error!("Strength Template chart to_string error: {error}"),
        }
        let result = Curve::new_linear(&self.result).map_err(|e| {
            format!(
                "Strength Template to_string result error:{}, src:{:?}",
                e, &self.result
            )
        })?;
        let limit_min = Curve::new_linear(&limit_min).map_err(|e| {
            format!(
                "Strength Template to_string limit_min error:{}, src:{:?}",
                e, &limit_min
            )
        })?;
        let limit_max = Curve::new_linear(&limit_max).map_err(|e| {
            format!(
                "Strength Template to_string limit_max error:{}, src:{:?}",
                e, &limit_max
            )
        })?;
        let mut table_values = Vec::new();
        let compute_percent = |result: f64, limit: f64| -> Result<f64, Error> {
            if limit != 0. {
                Ok(result * 100. / limit)
            } else {
                Err(Error::FromString(format!(
                    "Strength template to_string compute_percent error: limit=0!"
                )))
            }
        };
        let compute_value = |x: f64| -> Result<(f64, f64), Error> {
            let result = result.value(x)?;
            let limit_min = limit_min.value(x)?;
            let limit_max = limit_max.value(x)?;
            //    chart_values.push((x, limit_min, result, limit_max));
            let percent = if result < 0. {
                compute_percent(result, limit_min)?
            } else {
                compute_percent(result, limit_max)?
            };
            Ok((result, percent))
        };
        let (first_x, _) = *self.result.first().ok_or(Error::FromString(format!(
            "Strength template to_string error: no result.first!"
        )))?;
        let (mut max_abs_value, mut max_percent_value) = compute_value(first_x)?;
        let (mut max_abs_x, mut max_percent_x) = (first_x, first_x);
        for (x, _) in self.result {
            let (current_abs_value, current_percent_value) = compute_value(x)?;
            if current_abs_value.abs() > max_abs_value.abs() {
                max_abs_value = current_abs_value;
                max_abs_x = x;
            }
            if current_percent_value.abs() > max_percent_value.abs() {
                max_percent_value = current_percent_value;
                max_percent_x = x;
            }
        }
        let (limit_min_value, limit_max_value) =
            (limit_min.value(max_abs_x)?, limit_max.value(max_abs_x)?);
        let state_abs = max_abs_value >= limit_min_value && max_abs_value <= limit_max_value;
        table_values.push((
            max_abs_x,
            limit_min_value,
            max_abs_value,
            limit_max_value,
            state_abs,
        ));
        let (limit_min_value, limit_max_value) = (
            limit_min.value(max_percent_x)?,
            limit_max.value(max_percent_x)?,
        );
        let state_percent = max_percent_value <= 100.;
        table_values.push((
            max_percent_x,
            limit_min_value,
            max_percent_value,
            limit_max_value,
            state_percent,
        ));
        Ok(format!(
            "![chart](./assets/{}_chart.svg)",
            self.short_name.to_lowercase()
        ) + "\n\n"
            + &super::table::Table::new(&self.language, &self.short_name, &table_values)
                .to_string()?)
    }
}
