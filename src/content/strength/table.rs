pub struct Table {
    header: Vec<String>,
    // x, min, max, calc, percent, state
    values: Vec<(String, f64, f64, f64, f64, f64, bool)>,
}
//
impl Table {
    // x, min,  calc, max, state
    pub fn new(language: &str, name: &str, values: &[(f64, f64, f64, f64, f64, bool)]) -> Self {
        let header = if language.contains("en") {
            vec![
                "Criterion".to_owned(),
                "X".to_owned(),
                format!("${name}_{}$", r"{min}"),
                format!("${name}$"),
                format!("${name}, %$"),
                format!("${name}_{}$", r"{max}"),
                "Status".to_owned(),
            ]
        } else {
            vec![
                "Критерий".to_owned(),
                "X".to_owned(),
                format!("${name}_{}$", r"{мин}"),
                format!("${name}$"),
                format!("${name}, %$"),
                format!("${name}_{}$", r"{макс}"),
                "Статус".to_owned(),
            ]
        };
        let values: Vec<_> = vec!["Max value".to_owned(), "Max %".to_owned()]
            .into_iter()
            .enumerate()
            .map(|(i, v)| 
                (
                    v,
                    values[i].0,
                    values[i].1,
                    values[i].2,
                    values[i].3,
                    values[i].4,
                    values[i].5,
                )
            )
            .collect();
        Self::new_header(header, &values)
    }
    //
    pub fn new_header(header: Vec<String>, values: &[(String, f64, f64, f64, f64, f64, bool)]) -> Self {
        Self {
            header,
            values: Vec::from(values),
        }
    }
    //
    pub fn to_string(self) -> String {
        let mut string = self
            .header
            .iter()
            .map(|s| format!("|{s}"))
            .collect::<String>()
            + "|\n"
            + &(0..self.header.len()).map(|_| "|---").collect::<String>()
            + "|\n";
        for (name, x, min, max, abs, percent, state) in self.values {
            let state = match state {
                true => "+",
                _ => "-",
            };
            //   dbg!(result, target, delta, delta_result_percent);
            string += &format!(
                "|{}|{:.3}|{:.3}|{:.3}|{:.1}|{:.3}|{state}|\n",
                name, x, min, abs, percent, max,
            );
        }
        string
    }
}
