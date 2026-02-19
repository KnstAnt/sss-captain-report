use sal_core::{dbg::Dbg, error::Error};

pub struct Table {
    dbg: Dbg,
    header: Vec<String>,
    // x, min, calc, max, state
    values: Vec<(f64, f64, f64, f64, bool)>,
}
//
impl Table {
    // x, min,  calc, max, state
    pub fn new(
        parent: &Dbg,
        language: &String,
        name: &str,
        values: &[(f64, f64, f64, f64, bool)],
    ) -> Self {
        let dbg = Dbg::new(parent, "Table");
        let header = if language.contains("en") {
            vec![
                "X".to_owned(),
                format!("{name}_min"),
                format!("{name}"),
                format!("{name}_max"),
                "Status".to_owned(),
            ]
        } else {
            vec![
                "X".to_owned(),
                format!("{name}_мин"),
                format!("{name}"),
                format!("{name}_макс"),
                "Статус".to_owned(),
            ]
        };
        Self::new_header(dbg, header, values)
    }
    //
    pub fn new_header(header: Vec<String>, values: &[(f64, f64, f64, f64, bool)]) -> Self {
        Self {
            header,
            values: Vec::from(values),
        }
    }
    //
    pub fn to_string(self) -> Result<String, crate::error::Error> {
        let mut string = self
            .header
            .iter()
            .map(|s| format!("|{s}"))
            .collect::<String>()
            + "|\n"
            + &(0..self.header.len()).map(|_| "|---").collect::<String>()
            + "|\n";
        for (x, min, result, max, state) in self.values {
            let state = match state {
                false => "-",
                true => "+",
            };
            //   dbg!(result, target, delta, delta_result_percent);
            string += &format!("|{:.3}|{:.3}|{:.3}|{:.3}|{state}|\n", x, min, result, max,);
        }
        Ok(string)
    }
}
