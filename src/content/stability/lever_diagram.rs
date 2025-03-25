use crate::content::misc::{Curve, ICurve};

pub struct LeverDiagram {
    header: String,
    short_name: String,
    unit: String,
    dso: Vec<(f64, f64)>,
    ddo: Vec<(f64, f64)>,
    h: Vec<(f64, f64)>,
}
//
impl LeverDiagram {
    //
    pub fn new(language: &String, dso: &[(f64, f64)], ddo: &[(f64, f64)], h: &[(f64, f64)]) -> Self {
        let (header, short_name, unit) = if language.contains("en") {
            ("| Heel | Lever |", "DSO", "m")
        } else {
            ("| Крен | Плечо расчет |", "ДСО", "м")
        };
        Self {
            header: header.to_owned(),
            short_name: short_name.to_owned(),
            unit: unit.to_owned(),
            dso: Vec::from(dso),
            ddo: Vec::from(ddo),
            h: Vec::from(h),
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
        match super::chart::Chart::new(
            &self.short_name, 
            &self.unit, 
            &dso,
            &ddo,
            &self.h, 
        ).to_string() {
            Ok(_) => (),
            Err(error) => log2::error!("LeverDiagram to_string chart error: {error}"),
        };

     /*   if let Err(error) = std::fs::write("bin/assets/dso_chart.svg", super::chart::Chart::new(&self.short_name, &self.unit, &data).to_string()?) {
            log2::error!("Strength Template to_string std::fs::write error: {error}");
        }*/
        Ok( format!("![chart](./assets/dso_chart.svg)")
            + "\n\n" +
            &string
        )
    }
}
