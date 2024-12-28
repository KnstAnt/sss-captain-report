pub struct LeverDiagram {
    // angle, dso
    data: Vec<(f64, f64)>,
}
//
impl LeverDiagram {
    //
    pub fn new(data: &[(f64, f64)]) -> Self {
        Self {
            data: Vec::from(data),
        }
    }
    //
    pub fn to_string(self) -> Result<String, crate::error::Error> {
        let mut string = "### Диаграмма статической остойчивости\n".to_owned() + 
        &"| Крен | Плечо расчет |\n"  +
        &"|---|---|\n";
        for (angle, value) in self.data {
            string += &format!(
                "|{}|{:.3}|\n",
                angle as i32, value
            );
        }
        Ok(string)
    }
}
