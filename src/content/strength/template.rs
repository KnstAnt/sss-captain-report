use crate::content::Content;
use sal_core::{dbg::Dbg, error::Error};
//
pub struct Template {
    dbg: Dbg,
    language: String,
    short_name: String,
    unit: String,
    data: Vec<(f64, f64, f64, f64, f64, bool)>, //x, limit_min, limit_max, value_abs, value_percent, status (0/1)
}
//
impl Template {
    //
    pub fn new(
        parent: &Dbg,
        language: &str,
        short_name: &str,
        unit: &str,
        data: &[(f64, f64, f64, f64, f64, bool)],
    ) -> Self {
        let dbg = Dbg::new(parent, "Template");
        Self {
            dbg,
            language: language.to_owned(),
            short_name: short_name.to_owned(),
            unit: unit.to_owned(),
            data: Vec::from(data),
        }
    }
}
//
impl Content for Template {
    //
    fn to_string(self) -> Result<String, Error> {
        let data = self
            .data
            .iter()
            .map(|(x, min, max, value, ..)| (*x, *min, *max, *value))
            .collect();
        let (limit_min, limit_max): (Vec<(f64, f64)>, Vec<(f64, f64)>) = self
            .data
            .iter()
            .map(|(x, min, max, ..)| ((*x, *min), (*x, *max)))
            .unzip();
        let x_min = self.data.first().map(|(v, ..)| *v).unwrap_or(0.);
        let x_max = self.data.last().map(|(v, ..)| *v).unwrap_or(0.);
        let limit_y_min = limit_min.iter().fold(f64::MAX, |r, v| r.min(v.1));
        let limit_y_max = limit_max.iter().fold(f64::MIN, |r, v| r.max(v.1));
        let result_y_min = self
            .data
            .iter()
            .fold(f64::MAX, |r, (_, _, _, v, ..)| r.min(*v));
        let result_y_max = self
            .data
            .iter()
            .fold(f64::MIN, |r, (_, _, _, v, ..)| r.max(*v));
        let y_min = limit_y_min.min(result_y_min);
        let y_max = limit_y_max.max(result_y_max);
        let (mult_x, mult_y) = (20., 20.);
        let x_min = (x_min / mult_x).floor() * mult_x;
        let x_max = (x_max / mult_x).ceil() * mult_x;
        let y_min = (y_min / mult_y).floor() * mult_y;
        let y_max = (y_max / mult_y).ceil() * mult_y;
        match super::chart::Chart::new(
            &self.dbg,
            &self.language,
            &self.short_name,
            &self.unit,
            x_min,
            x_max,
            y_min,
            y_max,
            data,
        )
        .to_string()
        {
            Ok(_) => (),
            Err(error) => log2::error!("Strength Template chart to_string error: {error}"),
        }
        let max_abs =
            self.data.iter().fold(
                (0., 0., 0., 0., 0., false),
                |acc, v| if acc.3 < v.3 { *v } else { acc },
            );
        let max_percent =
            self.data.iter().fold(
                (0., 0., 0., 0., 0., false),
                |acc, v| if acc.4 < v.4 { *v } else { acc },
            );
        Ok(format!(
            "![chart](./assets/{}_chart.svg)",
            self.short_name.to_lowercase()
        ) + "\n\n"
            + &super::table::Table::new(&self.language, &self.short_name, &[max_abs, max_percent])
                .to_string())
    }
}
