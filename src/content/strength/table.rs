pub struct Table {
    header: Vec<String>,
    // x, min, calc, max, state
    values: Vec<(f64, f64, f64, f64, bool)>,
}
//
impl Table {
    // x, min,  calc, max, state
    pub fn new(name: &str, values: &[(f64, f64, f64, f64, bool)]) -> Self {
        Self::new_header(
            &vec![
                "X",
                &format!("${name}_{{min}}$"),
                &format!("${name}$"),
                &format!("${name}_{{max}}$"),
                "Статус",
            ],
            values,
        )
    }
    //
    pub fn new_header(header: &[&str], values: &[(f64, f64, f64, f64, bool)]) -> Self {
        Self {
            header: header.iter().map(|s| s.to_string()).collect(),
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
            string += &format!(
                "|{:.3}|{:.3}|{:.3}|{:.3}|{state}|\n",
                x, min, result, max,
            );
        }
        Ok(string)
    }
}
