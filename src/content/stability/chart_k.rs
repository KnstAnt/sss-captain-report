use std::path::PathBuf;
use plotters::prelude::*;
use super::chart::*;
use sal_core::{dbg::Dbg, error::Error};
//
pub struct ChartWeather {
    language: String,
    dso: Vec<(f64, f64)>,
    theta_0: f64,        // Статический угол крена судна
    theta_w1: (f64, f64), // wind_static, (angle, lever)
    theta_w2: (f64, f64), // wind_dynamic, (angle, lever)
    point_a: (f64, f64),  // старт дуги a, (angle, lever)
    point_b: (f64, f64),  // конец дуги b, (angle, lever) 
    area_a: f64,          // площадь а
    area_b: f64,          // площадь b
}
//
impl ChartWeather {
    //
    pub fn new(
        language: String,
        dso: &[(f64, f64)],
        theta_0: f64,      
        theta_w1: (f64, f64), 
        theta_w2: (f64, f64), 
        point_a: (f64, f64),
        point_b: (f64, f64), 
        area_a: f64,
        area_b: f64,
    ) -> Self {
        Self {
            language,
            dso: Vec::from(dso),
            theta_0,
            theta_w1,
            theta_w2,
            point_a,
            point_b,
            area_a,
            area_b,
        }
    }
    //
    pub fn to_string(self) -> Result<(), Error> {
        let (header, legend_dso) = if self.language.contains("en") {
            ("Weather Criteria", "SC")
        } else {
            ("Критерий погоды", "ДСО")
        };
        let path = PathBuf::from(format!("bin/assets/k_chart.svg"));
        let root  = SVGBackend::new(&path, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
        let x_min = self.point_a.0.floor();
        let x_max = 60f64;
        let y_min = self.point_a.1;
        let y_max = self.dso.iter().fold(f64::MIN, |s, v| s.max(v.1));
        let y_max = y_max.ceil();
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
        chart.draw_series(LineSeries::new(
                self.dso.clone(),
                &RGBColor(0, 150, 0),
            ))?
            .label(legend_dso)
            .legend(|(x, y)| Rectangle::new([(x - 15, y + 1), (x, y)], RGBColor(0, 150, 0)));   
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperLeft)
            .margin(20)
            .legend_area_size(5)
            .border_style(BLUE)
            .background_style(BLUE.mix(0.1))
            .label_font(("Calibri", 20))
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
        // theta_w1
        chart.draw_series(LineSeries::new(
            [(self.theta_w1.0, 0.), (self.theta_w1.0, self.theta_w1.1)],
            &RGBColor(150, 150, 0),
        ))?;
        // theta_w2 
        chart.draw_series(LineSeries::new(
            [(self.point_a.0, self.theta_w2.1), (self.point_b.0, self.theta_w2.1)],
            &RGBColor(150, 0, 50),
        ))?;
        // min
        chart.draw_series(LineSeries::new(
            [(self.point_b.0, 0.), (self.point_b.0, self.point_b.1)],
            &RGBColor(150, 150, 0),
        ))?;
        // a
        chart.draw_series(LineSeries::new(
            [self.point_a, (self.point_a.0, self.theta_w2.1)],
            &RGBColor(150, 150, 0),
        ))?;
        // w0
        draw_point_with_text(&mut chart, &[(self.theta_0, 0.)], ShowPoint::X, "θ0=", (-60, 5))?;
        // w1
        draw_point_with_text(&mut chart, &[(self.theta_w1.0, 0.)], ShowPoint::X, "θw1=", (0, 5))?;
        draw_point(&mut chart, &[self.theta_w1], ShowPoint::All, (5, -5))?;
        // w2
        draw_point(&mut chart, &[(self.point_b.0, 0.)], ShowPoint::X, (0, 5))?;
        // угол начала области а 
        draw_point(&mut chart, &[(self.point_a.0, 0.)], ShowPoint::X, (5, -15))?;
        // надписи с площадью по центру заливки 
        // area_a, отображаем площадь в левом верхнем углу
        let x_mid = self.point_a.0 + (self.theta_w2.0 - self.point_a.0)/3.;
        let y_mid = self.point_a.1 + (self.theta_w2.1 - self.point_a.1)/1.5;
        chart.draw_series(PointSeries::of_element(
            [(x_mid, y_mid)],
            3,
            &RGBColor(0, 0, 0),
            &|c, _, _| {
                return EmptyElement::at(c)  
                + Text::new(format!("a={:.3}", self.area_a), (0, 0), ("sans-serif", 14).into_font());
            },
        ))?;
        // area_b
        let delta_x = self.point_b.0 - self.theta_w2.0;
        let y_max_area = self.dso.iter().filter(|v| v.0 <= self.point_b.0 ).fold(f64::MIN, |r, v| r.max(v.1) );
        let shift = (self.point_b.1*1.1/y_max_area).min(1.2).max(1.);
        let x_mid = self.theta_w2.0 + (delta_x * shift) / 2.;
        let delta_y = y_max_area - self.theta_w2.1;
        let y_mid = self.theta_w2.1 + delta_y / (2. * shift);
        chart.draw_series(PointSeries::of_element(
            [(x_mid, y_mid)],
            3,
            &RGBColor(0, 0, 0),
            &|c, _, _| {
                return EmptyElement::at(c)  
                + Text::new(format!("b={:.3}", self.area_b), (0, 0), ("sans-serif", 14).into_font());
            },
        ))?;
        // заливка областей
        let mut a = Vec::new();
        a.push(self.point_a);
        self.dso.iter().filter(|v| v.0 >= self.point_a.0 && v.0 <= self.theta_w2.0 ).for_each(|v| a.push(*v));
        a.push(self.theta_w2);
        chart.draw_series(AreaSeries::new(a, self.theta_w2.1, RED.mix(0.2))).unwrap();
        let mut b = Vec::new();
        b.push(self.theta_w2);
        self.dso.iter().filter(|v| v.0 >= self.theta_w2.0 && v.0 <= self.point_b.0 ).for_each(|v| b.push(*v));
        b.push(self.point_b);
        chart.draw_series(AreaSeries::new(b, self.theta_w2.1, GREEN.mix(0.2))).unwrap();
        match root.present() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::FromString(format!("stability chart root.present() error: {e}"))),
        }   
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chart() {
        let result = ChartWeather::new(
            "en".to_owned(),
            &[(-10., -0.8), (-5., -0.5), (0., 0.), (5.0, 1.), (15.0, 2.), (25.0, 2.5), (40.0, 2.), (60.0, 1.),],
            0.,
            (5., 1.),
            (10., 1.5),
            (-10., -0.8),
            (40., 2.),
            4.,
            5.,
        ).to_string();
        assert!(result.is_ok());
    }        
}
