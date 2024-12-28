use crate::error::Error;
use crate::content::Content;
use crate::content::misc::{Curve, ICurve};
//
pub struct Template {
    short_name: String,
    result: Vec<(f64, f64)>, //x, value
    limit: Vec<(f64, f64, f64)>, //x, min, max
}
//
impl Template {
    //
    pub fn new( 
        short_name: String,
        result: &[(f64, f64)],
        limit: &[(f64, f64, f64)],
    ) -> Self {
        Self {
            short_name,
            result: Vec::from(result),
            limit: Vec::from(limit),
        }
    }
}
//
impl Content for Template {
    //
    fn to_string(self) -> Result<String, Error> {
        let (limit_min, limit_max): (Vec<(f64, f64)>, Vec<(f64, f64)>) = self.limit.into_iter().map(|(x, min, max)| ((x, min), (x, max))).unzip();
    //    let (fr_x, target) = self.target.into_iter().map(|(x, fr, v)| ((x, fr as f64), (fr, v))).unzip();
        let result = Curve::new_linear(&self.result).map_err(|e| format!("Strength Template to_string result error:{}, src:{:?}", e, &self.result))?;
        let limit_min = Curve::new_linear(&limit_min).map_err(|e| format!("Strength Template to_string limit_min error:{}, src:{:?}", e, &limit_min))?; 
        let limit_max = Curve::new_linear(&limit_max).map_err(|e| format!("Strength Template to_string limit_max error:{}, src:{:?}", e, &limit_max))?; 
        let mut values = Vec::new();
        let compute_percent = |result: f64, limit: f64| -> Result<f64, Error> {
            if limit != 0. {
                Ok(result*100./limit)
            } else {
                Err(Error::FromString(format!("Strength template to_string compute_percent error: limit=0!")))
            }
        };
        let compute_value = |x: f64| -> Result<(f64, f64), Error>  {
            let result = result.value(x)?;
            let limit_min = limit_min.value(x)?;
            let limit_max = limit_max.value(x)?;
            let percent = if result < 0. {
                compute_percent(result, limit_min)?
            } else {
                compute_percent(result, limit_max)?
            };
            Ok((result, percent))
        };
        let (first_x, _) = *self.result.first().ok_or(Error::FromString(format!("Strength template to_string error: no result.first!")))?;
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
        };
        let (limit_min_value, limit_max_value) = (limit_min.value(max_abs_x)?, limit_max.value(max_abs_x)?);
        let state_abs = max_abs_value >= limit_min_value && max_abs_value <= limit_max_value;
        values.push((max_abs_x, limit_min_value, max_abs_value, limit_max_value, state_abs));
        let (limit_min_value, limit_max_value) = (limit_min.value(max_percent_x)?, limit_max.value(max_percent_x)?);
        let state_percent = max_percent_value <= 100.;
        values.push((max_percent_x, limit_min_value, max_percent_value, limit_max_value, state_percent)); 
        super::table::Table::new(&self.short_name, &values).to_string()
    } 
}
