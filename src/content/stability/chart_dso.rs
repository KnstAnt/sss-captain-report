use std::path::PathBuf;
use plotters::prelude::*;
use sal_core::{dbg::Dbg, error::Error};
use super::chart::*;
//
pub struct ChartDSO {
    language: String,
    dso: Vec<(f64, f64)>,
    ddo: Vec<(f64, f64)>, 
    h: Vec<(f64, f64)>, 
}
//
impl ChartDSO {
    //
    pub fn new(
        language: String,
        dso: &[(f64, f64)],
        ddo: &[(f64, f64)],
        h: &[(f64, f64)],
    ) -> Self {
        Self {
            language,
            dso: Vec::from(dso),
            ddo: Vec::from(ddo),
            h: Vec::from(h),
        }
    }
    //
    pub fn to_string(self) -> Result<(), Error> {
        let (header, legend_dso, legend_ddo, legend_h) = if self.language.contains("en") {
            ("Initial, Static & Dynamic Stability", "SC", "DC", "h")
        } else {
            ("Начальная, статическая и динамическая остойчивость", "ДСО", "ДДО", "h")
        };
        let path = PathBuf::from(format!("bin/assets/dso_chart.svg"));
        let root  = SVGBackend::new(&path, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
        let x_min = 0f64.min(self.h.first().unwrap_or(&(0., 0.)).0).floor();
        let x_max = 60f64;
        let y_min = -0.1f64;//self.dso.iter().fold(f64::MAX, |s, v| s.min(v.1));
        let y_max = self.dso.iter().fold(f64::MIN, |s, v| s.max(v.1));
        let y_max = y_max.ceil();
        // After this point, we should be able to construct a chart context
        let mut chart = ChartBuilder::on(&root)
            .margin(15)
            // Set the caption of the chart
            .caption(header, ("sans-serif", 20).into_font())
            // Set the size of the label region
            .x_label_area_size(10)
            .y_label_area_size(25)
            // Finally attach a coordinate on the drawing area and make a chart context
            .build_cartesian_2d(x_min..x_max, y_min..y_max)?;//.map_err(|e| e.into())?;
        chart
            .configure_mesh()
            // We can customize the maximum number of labels allowed for each axis
            .x_labels(10)            
            .y_labels(10)         
            .x_label_formatter(&|x| format!("{:.1}", x))            
            .y_label_formatter(&|x| format!("{:.1}", x))
            .draw()?;
        // ось x
        chart.draw_series(LineSeries::new(
            [(x_min, 0.), (x_max, 0.)],
            &RGBColor(0, 0, 0),
        ))?;
        // ось y
        chart.draw_series(LineSeries::new(
            [(0., y_min), (0., y_max)],
            &RGBColor(0, 0, 0),
        ))?;
        // отрисовка линии ДСО
        chart.draw_series(LineSeries::new(
                self.dso.clone(),//.iter().map(|(x, y)| (*x as f32, *y as f32)),
                &RGBColor(0, 150, 0),
            ))?
            .label(legend_dso)
            .legend(|(x, y)| Rectangle::new([(x - 15, y + 1), (x, y)], RGBColor(0, 150, 0)));   
        // отрисовка линии ДДО
        chart.draw_series(LineSeries::new(
                self.ddo.clone(),
                &RGBColor(150, 150, 0),
            ))?
            .label(legend_ddo)
            .legend(|(x, y)| Rectangle::new([(x - 15, y + 1), (x, y)], RGBColor(150, 150, 0)));   
        // отрисовка линии h
        chart.draw_series(LineSeries::new(
                self.h.clone(),
                &RGBColor(150, 0, 0),
            ))?
            .label(legend_h)
            .legend(|(x, y)| Rectangle::new([(x - 15, y + 1), (x, y)], RGBColor(150, 0, 0)));   
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperLeft)
            .margin(20)
            .legend_area_size(5)
            .border_style(BLUE)
            .background_style(BLUE.mix(0.1))
            .label_font(("Calibri", 20))
            .draw()?; 
        // отрисовка первой точки h
        draw_point(&mut chart, &[self.h[0]], ShowPoint::X, (0, 5))?;
        // отрисовка последней точки h
        draw_point(&mut chart, &[self.h.last().unwrap_or(&(0.,0.)).clone()], ShowPoint::All, (-80, -15))?;
        match root.present() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::FromString(format!("stability chart_dso root.present() error: {e}"))),
        }   
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chart() {
        let result = ChartDSO::new(
            "en".to_owned(),
            &[(0., 0.), (5.0, 1.), (15.0, 2.), (25.0, 2.5), (40.0, 2.), (60.0, 1.),],
            &[(0., 0.), (5., 0.3), (15., 0.7), (25., 0.7), (40.0, 1.2), (60.0, 1.5),],
            &[(0., 0.), (57.3, 1.2),],
        ).to_string();
        assert!(result.is_ok());
    }        
}
