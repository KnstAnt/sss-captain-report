use std::path::PathBuf;
use crate::{content::misc::Curve, error::Error};
use plotters::prelude::*;
//
pub struct ChartBulk {
    short_name: String,
    unit: String,
    dso: Vec<(f64, f64)>,
    a: f64,         // первая точка кривой, lever
    p1_dso: (f64, f64), // пересечение кривой плеч кренящего момента и ДСО, (angle, lever)
    b: (f64, f64),   // значение кривой плеч кренящего момента при макс. разности, (angle, lever)
    p_40_dso: f64,    // значение ДСО  при макс. разности
}
//
impl ChartBulk {
    //
    pub fn new(
        short_name: &str,
        unit: &str,
        dso: &[(f64, f64)],
        a: f64,
        p1_dso: (f64, f64), 
        b: (f64, f64),  
        p_40_dso: f64,
    ) -> Self {
        Self {
            short_name: short_name.to_owned(),
            unit: unit.to_owned(),
            dso: Vec::from(dso),
            a,
            p1_dso,
            b,
            p_40_dso,
        }
    }
    //
    pub fn to_string(self) -> Result<(), Error> {
        let path = PathBuf::from(format!("bin/assets/bulk_chart.svg"));
        let root  = SVGBackend::new(&path, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
        let x_min = 0f64;
        let x_max = 90f64;
        let y_min = self.dso.iter().fold(f64::MAX, |s, v| s.min(v.1));
        let y_max = self.dso.iter().fold(f64::MIN, |s, v| s.max(v.1));
        let y_max = (y_max + 1.).ceil();
        let root = root.margin(
            y_max, 
            y_min, 
            x_min, 
            x_max,);
        // After this point, we should be able to construct a chart context
        let mut chart = ChartBuilder::on(&root)
            // Set the caption of the chart
            .caption(self.short_name, ("sans-serif", 20).into_font())
            // Set the size of the label region
            .x_label_area_size(40)
            .y_label_area_size(60)
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
            self.dso.clone(),//.iter().map(|(x, y)| (*x as f32, *y as f32)),
            &RGBColor(0, 150, 0),
        ))?;        
        // кривая плеч кренящего момента
        let curve_lever = [(0., self.a), (self.p1_dso.0, self.p1_dso.1), (self.b.0, self.b.1)];
        chart.draw_series(LineSeries::new(
            curve_lever.clone(),
            &RGBColor(150, 150, 0),
        ))?;
        let curve_lever = [(self.p1_dso.0, self.p1_dso.1), (self.b.0, self.b.1)];
        chart.draw_series(PointSeries::of_element(
            curve_lever.clone(),//.iter().map(|(x, y)| (*x as f32, *y as f32)),
            3,
            &RGBColor(150, 150, 0),
            &|c, s, st| {
                return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
                + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 14).into_font());
            },
        ))?;
        chart.draw_series(PointSeries::of_element(
            [(self.b.0, 0.), (0., self.a),],//.iter().map(|(x, y)| (*x as f32, *y as f32)),
            3,
            &RGBColor(150, 150, 0),
            &|c, s, st| {
                return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                + Circle::new((0,0),s,st.filled());
            },
        ))?;
        chart.draw_series(PointSeries::of_element(
            [(self.b.0, 0.), (0., self.a),],//.iter().map(|(x, y)| (*x as f32, *y as f32)),
            3,
            &RGBColor(150, 150, 0),
            &|c, s, st| {
                return EmptyElement::at((c.0, c.1 + 0.1))    // We want to construct a composed element on-the-fly
                + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 14).into_font());
            },
        ))?;
        // угол макс. разницы
        let curve_dif = [(self.b.0, self.p_40_dso)];
        chart.draw_series(LineSeries::new(
            curve_dif.clone(),
            &RGBColor(150, 0, 0),
        ))?;
        chart.draw_series(PointSeries::of_element(
            curve_dif.clone(),
            3,
            &RGBColor(150, 0, 0),
            &|c, s, st| {
                return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
                + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 14).into_font());
            },
        ))?;
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
            &String::new(), 
            "name", 
            &[(0., 0.), (5.0, 1.), (15.0, 2.), (25.0, 2.5), (40.0, 2.), (60.0, 1.),],
            1.1,
            (5., 1.),
            (40., 0.6),
            2.,
        ).to_string();
        assert!(result.is_ok());
    }        
}
