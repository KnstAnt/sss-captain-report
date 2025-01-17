use crate::content::misc::{Curve, ICurve};

pub struct LeverDiagram {
    header: String,
    // angle, dso
    data: Vec<(f64, f64)>,
}
//
impl LeverDiagram {
    //
    pub fn new(language: &String, data: &[(f64, f64)]) -> Self {
        let header = if language.contains("en") {
            "| Heel | Lever |"
        } else {
            "| Крен | Плечо расчет |"
        }
        .to_owned();
        Self {
            header,
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
        Ok(super::chart::Chart::new("DSO", "m", &data).to_string()? + "\n\n" + &string)
    }
}
