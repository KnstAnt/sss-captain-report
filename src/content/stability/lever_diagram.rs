use crate::content::misc::{Curve, ICurve};

pub struct LeverDiagram {
    header: String,
    short_name: String,
    unit: String,
    // angle, dso
    data: Vec<(f64, f64)>,
}
//
impl LeverDiagram {
    //
    pub fn new(language: &String, data: &[(f64, f64)]) -> Self {
        let (header, short_name, unit) = if language.contains("en") {
            ("| Heel | Lever |", "DSO", "m")
        } else {
            ("| Крен | Плечо расчет |", "ДСО", "м")
        }        ;
        Self {
            header: header.to_owned(),
            short_name: short_name.to_owned(),
            unit: unit.to_owned(),
            data: Vec::from(data),
        }
    }
    //
    pub fn to_string(self) -> Result<String, crate::error::Error> {
        let mut string = self.header.clone() + "\n|---|---|\n";
        let curve = Curve::new_linear(&self.data).map_err(|e| {
            format!(
                "LeverDiagram to_string curve error:{}, src:{:?}",
                e, &self.data
            )
        })?;
        let keys = (0..60).filter(|v| *v as f64 % 5. == 0.).collect::<Vec<_>>();
        let mut data = Vec::new();
        for key in keys {
            let value = curve
                .value(key as f64)
                .map_err(|e| format!("LeverDiagram to_string curve value error:{}", e))?;
            data.push((key, value));
            string += &format!("|{}|{:.3}|\n", key, value);
        }

        if let Err(error) = std::fs::write("bin/assets/dso_chart.svg", super::chart::Chart::new(&self.short_name, &self.unit, &data).to_string()?) {
            log2::error!("Strength Template to_string std::fs::write error: {error}");
        }
        Ok( format!("![chart](./assets/dso_chart.svg)")
            + "\n\n" +
            &string
        )
    }
}
