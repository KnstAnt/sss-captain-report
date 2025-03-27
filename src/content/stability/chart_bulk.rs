use std::path::PathBuf;
use crate::error::Error;
use plotters::prelude::*;
//
pub struct ChartBulk {
    language: String,
    dso: Vec<(f64, f64)>,
    a: f64,         // первая точка кривой, lever
    p1_dso: (f64, f64),// пересечение кривой плеч кренящего момента и ДСО, (angle, lever)
    b: (f64, f64),  // значение кривой плеч кренящего момента при макс. разности, (angle, lever)
    p_40_dso: f64,  // значение ДСО  при макс. разности
    area: f64,      // остаточная площадь
}
//
impl ChartBulk {
    //
    pub fn new(
        language: String,
        dso: &[(f64, f64)],
        a: f64,
        p1_dso: (f64, f64), 
        b: (f64, f64),  
        p_40_dso: f64,
        area: f64,
    ) -> Self {
        Self {
            language,
            dso: Vec::from(dso),
            a,
            p1_dso,
            b,
            p_40_dso,
            area,
        }
    }
    //
    pub fn to_string(self) -> Result<(), Error> {
        let (header, legend_dso) = if self.language.contains("en") {
            ("Grain Shift", "SC")
        } else {
            ("Смещение зерна", "ДСО")
        };
        let path = PathBuf::from(format!("bin/assets/bulk_chart.svg"));
        let root  = SVGBackend::new(&path, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
        let x_min = 0f64;
        let x_max = 60f64;
        let y_min = self.dso.iter().fold(f64::MAX, |s, v| s.min(v.1));
        let y_max = self.dso.iter().fold(f64::MIN, |s, v| s.max(v.1));
        let y_max = y_max.ceil();
        let root = root.margin(
            y_max, 
            y_min, 
            x_min, 
            x_max,);
        let mut chart = ChartBuilder::on(&root)
            .caption(header, ("sans-serif", 20).into_font())
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)?;
        chart
            .configure_mesh()
            .x_labels(10)            
            .y_labels(10)         
            .x_label_formatter(&|x| format!("{:.1}", x))            
            .y_label_formatter(&|x| format!("{:.1}", x))
            .draw()?;
        // отрисовка линии кривой ДСО
        chart.draw_series(LineSeries::new(
                self.dso.clone(),
                &RGBColor(0, 150, 0),
            ))?
            .label(legend_dso)
            .legend(|(x, y)| Rectangle::new([(x - 15, y + 1), (x, y)], RGBColor(0, 150, 0)));   
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperRight)
            .margin(20)
            .legend_area_size(5)
            .border_style(BLUE)
            .background_style(BLUE.mix(0.1))
            .label_font(("Calibri", 20))
            .draw()?;     
        // отрисовка линии кривой плеч кренящего момента
        let curve_lever = [(0., self.a), (self.p1_dso.0, self.p1_dso.1), (self.b.0, self.b.1)];
        chart.draw_series(LineSeries::new(
            curve_lever.clone(),
            &RGBColor(150, 150, 0),
        ))?;
        // отрисовка точек кривой плеч кренящего момента
        let curve_lever = [(self.p1_dso.0, self.p1_dso.1), (self.b.0, self.b.1)];
        chart.draw_series(PointSeries::of_element(
            curve_lever.clone(),
            3,
            &RGBColor(150, 150, 0),
            &|c, s, st| {
                return EmptyElement::at(c)  
                + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
                + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 14).into_font());
            },
        ))?;
        // часть точек рисуется отдельно чтобы значение было выше
        chart.draw_series(PointSeries::of_element(
            [(self.b.0, 0.), (0., self.a),],
            3,
            &RGBColor(150, 150, 0),
            &|c, s, st| {
                return EmptyElement::at(c)  
                + Circle::new((0,0),s,st.filled())
                + Text::new(format!("{:?}", c), (5, -15), ("sans-serif", 14).into_font());
            },
        ))?;
        // отрисовка линии угла макс. разницы
        let curve_dif = [(self.b.0, self.p_40_dso)];
        chart.draw_series(LineSeries::new(
            curve_dif.clone(),
            &RGBColor(150, 0, 0),
        ))?;
        // отрисовка точек угла макс. разницы
        chart.draw_series(PointSeries::of_element(
            curve_dif.clone(),
            3,
            &RGBColor(150, 0, 0),
            &|c, s, st| {
                return EmptyElement::at(c)  
                + Circle::new((0,0),s,st.filled()) 
                + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 14).into_font());
            },
        ))?;
        // отрисовка вертикальной линии к первой точке пересечения ДСО и кривой кренящих плеч
        let curve_dif = [(self.p1_dso.0, 0.), (self.p1_dso.0, self.p1_dso.1)];
        chart.draw_series(LineSeries::new(
            curve_dif.clone(),
            &RGBColor(150, 0, 0),
        ))?;
        // отрисовка точки на оси Х, надпись выше линии
        chart.draw_series(PointSeries::of_element(
            [(self.p1_dso.0, 0.)],
            3,
            &RGBColor(150, 150, 0),
            &|c, s, st| {
                return EmptyElement::at(c)  
                + Circle::new((0,0),s,st.filled())
                + Text::new(format!("θg = {:?}", c.0), (5, -15), ("sans-serif", 14).into_font());
            },
        ))?;
        // надпись с площадью по центру заливки
        let x_mid = self.p1_dso.0 + (self.b.0 - self.p1_dso.0)/2.;
        let y_mid = self.b.1 + (self.dso.iter().fold(f64::MIN, |r, v| r.max(v.1) ) - self.b.1)/2.;
        chart.draw_series(PointSeries::of_element(
            [(x_mid, y_mid)],
            3,
            &RGBColor(0, 0, 0),
            &|c, _, _| {
                return EmptyElement::at(c)  
                + Text::new(format!("S={:.3}", self.area), (0, -7), ("sans-serif", 14).into_font());
            },
        ))?;
        // заливка области
        let mut points = Vec::new();
        points.push(self.p1_dso);
        self.dso.iter().filter(|v| v.0 >= self.p1_dso.0 && v.0 <= self.b.0 ).for_each(|v| points.push(*v));
        points.push((self.b.0, self.p_40_dso));
        chart.draw_series(AreaSeries::new(points, self.p1_dso.1, RED.mix(0.2))).unwrap();
        let curve_lever = [(self.p1_dso.0, self.p1_dso.1), (self.b.0, self.b.1)];
        chart.draw_series(AreaSeries::new(curve_lever, self.p1_dso.1, RED.mix(0.2))).unwrap();
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
        let result = ChartBulk::new(
            "en".to_owned(), 
            &[(0., 0.), (5.0, 1.), (15.0, 2.), (25.0, 2.5), (40.0, 2.), (60.0, 1.),],
            1.1,
            (5., 1.),
            (40., 0.6),
            2.,
            5.,
        ).to_string();
        assert!(result.is_ok());
    }        
}
