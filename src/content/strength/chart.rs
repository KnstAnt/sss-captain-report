use std::path::PathBuf;
use crate::error::Error;
use plotters::prelude::*;
//
pub struct Chart {
    header: (String, String, String),
    short_name: String,
    unit: String,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    result: Vec<(f64, f64)>, //x, calc
    target_min: Vec<(f64, f64)>, //x, min
    target_max: Vec<(f64, f64)>, //x, max
}
//
impl Chart {
    //
    pub fn new(
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
        let header = if language.contains("en") {
            ("min".to_owned(), "calc".to_owned(), "max".to_owned())
        } else {
            ("мин".to_owned(), "расчет".to_owned(), "макс".to_owned())
        };
        Self {
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
        let path = PathBuf::from(format!("bin/assets/{}_chart.svg", self.short_name.to_lowercase()));
        let root  = SVGBackend::new(&path, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
        let root = root.margin(
            self.y_max, 
            self.y_min, 
            self.x_min, 
            self.x_max,);
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(self.x_min..self.x_max, self.y_min..self.y_max)?;
        chart
            .configure_mesh()
            .x_labels(10)
            .x_label_formatter(&|x| format!("{:.1}", x))
            .y_labels(10)                     
            .y_label_formatter(&|x| format!("{:.1}", x))
            .draw()?;    
        chart.draw_series(LineSeries::new(
            self.result.clone(),
            &RED,
        ))?;
        if self.result.len() <= 30 {
            chart.draw_series(PointSeries::of_element(
                self.result.clone(),
                3,
                &RED,
                &|c, s, st| {
                    return EmptyElement::at(c)    
                    + Circle::new((0,0),s,st.filled()) 
                    + Text::new(format!("{:.2}", c.1), (5, 5), ("sans-serif", 10).into_font());
                },
            ))?;
        }
        chart.draw_series(LineSeries::new(
            self.target_min.clone(),
            &GREEN,
        ))?;
        if self.target_min.len() <= 30 {
            chart.draw_series(PointSeries::of_element(
                self.target_min.clone(),
                3,
                &GREEN,
                &|c, s, st| {
                    return EmptyElement::at(c)    
                    + Circle::new((0,0),s,st.filled()) 
                    + Text::new(format!("{:.2}", c.1), (5, 5), ("sans-serif", 10).into_font());
                },
            ))?;
        }
        chart.draw_series(LineSeries::new(
            self.target_max.clone(),
            &GREEN,
        ))?;
        if self.target_max.len() <= 30 {
            chart.draw_series(PointSeries::of_element(
                self.target_max,
                3,
                &GREEN,
                &|c, s, st| {
                    return EmptyElement::at(c)   
                    + Circle::new((0,0),s,st.filled()) 
                    + Text::new(format!("{:.2}", c.1), (5, 5), ("sans-serif", 10).into_font());
                },
            ))?;
        }
        match root.present() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::FromString(format!("strength chart root.present() error: {e}"))),
        }
    }
}

impl From<DrawingAreaErrorKind<std::io::Error>> for Error {
    fn from(value: DrawingAreaErrorKind<std::io::Error>) -> Self {
        Self::FromString(format!("DrawingArea error: {}", value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chart() {
        let result = Chart::new(
            &String::new(), 
            "name", 
            "unit", 
            -1.,
            9.,
            -10.,
            10.,
            &[(-1.0, 0.), (1.0, -5.), (3.0, -10.), (5.0, -5.), (7.0, 3.), (9.0, 0.),],
            &[(-1., -5.), (0., -10.), (8., -10.), (9., -5.),],
            &[(-1., 5.), (0., 10.), (8., 10.), (9., 5.),],
        ).to_string();
        assert!(result.is_ok());
    }        
}