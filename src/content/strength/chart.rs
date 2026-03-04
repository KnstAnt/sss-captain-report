use plotters::prelude::*;
use sal_core::{dbg::Dbg, error::Error};
use std::path::PathBuf;
//
pub struct Chart {
    dbg: Dbg,
    header: (String, String, String),
    short_name: String,
    unit: String,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    result: Vec<(f64, f64)>,     //x, calc
    target_min: Vec<(f64, f64)>, //x, min
    target_max: Vec<(f64, f64)>, //x, max
}
//
impl Chart {
    //
    pub fn new(
        parent: &Dbg,
        language: &String,
        short_name: &str,
        unit: &str,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        result: &[(f64, f64)],
        target_min: &[(f64, f64)],
        target_max: &[(f64, f64)],
    ) -> Self {
        let dbg = Dbg::new(parent, "Chart");
        let header = if language.contains("en") {
            ("min".to_owned(), "calc".to_owned(), "max".to_owned())
        } else {
            ("мин".to_owned(), "расчет".to_owned(), "макс".to_owned())
        };
        Self {
            dbg,
            header,
            short_name: short_name.to_owned(),
            unit: unit.to_owned(),
            x_min,
            x_max,
            y_min,
            y_max,
            result: Vec::from(result),
            target_min: Vec::from(target_min),
            target_max: Vec::from(target_max),
        }
    }
    //
    pub fn to_string(self) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "to_string");
        let path = PathBuf::from(format!(
            "bin/assets/{}_chart.svg",
            self.short_name.to_lowercase()
        ));
        let root = SVGBackend::new(&path, (800, 600)).into_drawing_area();
        root.fill(&WHITE)
            .map_err(|err| error.err(err.to_string()))?;
        let mut chart = ChartBuilder::on(&root)
            .margin(15)
            .x_label_area_size(10)
            .y_label_area_size(30)
            .build_cartesian_2d(self.x_min..self.x_max, self.y_min..self.y_max)
            .map_err(|err| error.err(err.to_string()))?;
        chart
            .configure_mesh()
            .x_labels(10)
            .x_label_formatter(&|x| format!("{:.1}", x))
            .y_labels(10)
            .y_label_formatter(&|x| format!("{:.1}", x))
            .draw()
            .map_err(|err| error.err(err.to_string()))?;
        // ось x
        chart
            .draw_series(LineSeries::new(
                [(self.x_min, 0.), (self.x_max, 0.)],
                &RGBColor(0, 0, 0),
            ))
            .map_err(|err| error.err(err.to_string()))?;
        chart
            .draw_series(LineSeries::new(self.result.clone(), &RGBColor(150, 0, 0)))
            .map_err(|err| error.err(err.to_string()))?
            .label(&self.short_name)
            .legend(|(x, y)| Rectangle::new([(x - 15, y + 1), (x, y)], &RGBColor(150, 0, 0)));
        chart
            .draw_series(LineSeries::new(
                self.target_max.clone(),
                &RGBColor(150, 150, 0),
            ))
            .map_err(|err| error.err(err.to_string()))?
            .label(self.short_name.clone() + "_max")
            .legend(|(x, y)| Rectangle::new([(x - 15, y + 1), (x, y)], &RGBColor(150, 150, 0)));
        chart
            .draw_series(LineSeries::new(
                self.target_min.clone(),
                &RGBColor(0, 150, 0),
            ))
            .map_err(|err| error.err(err.to_string()))?
            .label(self.short_name.clone() + "_min")
            .legend(|(x, y)| Rectangle::new([(x - 15, y + 1), (x, y)], &RGBColor(0, 150, 0)));
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperLeft)
            .margin(20)
            .legend_area_size(5)
            .border_style(BLUE)
            .background_style(BLUE.mix(0.1))
            .label_font(("Calibri", 20))
            .draw()
            .map_err(|err| error.err(err.to_string()))?;
        match root.present() {
            Ok(_) => Ok(()),
            Err(e) => Err(error.err(format!("strength chart root.present() error: {e}"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chart() {
        let result = Chart::new(
            &Dbg::new("Chart", "test"),
            &String::new(),
            "name",
            "unit",
            -1.,
            9.,
            -10.,
            10.,
            &[
                (-1.0, 0.),
                (1.0, -5.),
                (3.0, -10.),
                (5.0, -5.),
                (7.0, 3.),
                (9.0, 0.),
            ],
            &[(-1., -5.), (0., -10.), (8., -10.), (9., -5.)],
            &[(-1., 5.), (0., 10.), (8., 10.), (9., 5.)],
        )
        .to_string();
        assert!(result.is_ok());
    }
}
