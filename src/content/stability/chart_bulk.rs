use super::chart::*;
use plotters::prelude::*;
use sal_core::{dbg::Dbg, error::Error};
use std::path::PathBuf;
//
pub struct ChartBulk {
    dbg: Dbg,
    language: String,
    dso: Vec<(f64, f64)>,
    a: f64,             // первая точка кривой, lever
    p1_dso: (f64, f64), // пересечение кривой плеч кренящего момента и ДСО, (angle, lever)
    b: (f64, f64),      // значение кривой плеч кренящего момента при макс. разности, (angle, lever)
    p_40_dso: f64,      // значение ДСО  при макс. разности
    area: f64,          // остаточная площадь
}
//
impl ChartBulk {
    //
    pub fn new(
        parent: &Dbg,
        language: String,
        dso: &[(f64, f64)],
        a: f64,
        p1_dso: (f64, f64),
        b: (f64, f64),
        p_40_dso: f64,
        area: f64,
    ) -> Self {
        let dbg = Dbg::new(parent, "ChartBulk");
        Self {
            dbg,
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
        let error = Error::new(&self.dbg, "to_string");
        let (header, legend_dso) = if self.language.contains("en") {
            ("Grain Shift", "SC")
        } else {
            ("Смещение зерна", "ДСО")
        };
        let path = PathBuf::from(format!("bin/assets/bulk_chart.svg"));
        let root = SVGBackend::new(&path, (800, 600)).into_drawing_area();
        root.fill(&WHITE)
            .map_err(|err| error.err(err.to_string()))?;
        let x_min = 0f64;
        let x_max = 60f64;
        let y_min = -0.1f64; //self.dso.iter().fold(f64::MAX, |s, v| s.min(v.1));
        let y_max = self.dso.iter().fold(f64::MIN, |s, v| s.max(v.1));
        let y_max = y_max.ceil();
        let mut chart = ChartBuilder::on(&root)
            .margin(15)
            .caption(header, ("sans-serif", 20).into_font())
            .x_label_area_size(10)
            .y_label_area_size(25)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .map_err(|err| error.err(err.to_string()))?;
        chart
            .configure_mesh()
            .x_labels(10)
            .y_labels(10)
            .x_label_formatter(&|x| format!("{:.1}", x))
            .y_label_formatter(&|x| format!("{:.1}", x))
            .draw()
            .map_err(|err| error.err(err.to_string()))?;
        // ось x
        chart
            .draw_series(LineSeries::new(
                [(x_min, 0.), (x_max, 0.)],
                &RGBColor(0, 0, 0),
            ))
            .map_err(|err| error.err(err.to_string()))?;
        // ось y
        chart
            .draw_series(LineSeries::new(
                [(0., y_min), (0., y_max)],
                &RGBColor(0, 0, 0),
            ))
            .map_err(|err| error.err(err.to_string()))?;
        // отрисовка линии кривой ДСО
        chart
            .draw_series(LineSeries::new(self.dso.clone(), &RGBColor(0, 150, 0)))
            .map_err(|err| error.err(err.to_string()))?
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
            .draw()
            .map_err(|err| error.err(err.to_string()))?;
        // отрисовка линии кривой плеч кренящего момента
        let curve_lever = [
            (0., self.a),
            (self.p1_dso.0, self.p1_dso.1),
            (self.b.0, self.b.1),
        ];
        chart
            .draw_series(LineSeries::new(curve_lever.clone(), &RGBColor(150, 0, 0)))
            .map_err(|err| error.err(err.to_string()))?;
        // отрисовка линии угла макс. разницы
        let curve_dif = [(self.b.0, self.p_40_dso), (self.b.0, 0.)];
        chart
            .draw_series(LineSeries::new(curve_dif.clone(), &RGBColor(150, 150, 0)))
            .map_err(|err| error.err(err.to_string()))?;
        // отрисовка вертикальной линии к первой точке пересечения ДСО и кривой кренящих плеч
        let curve_theta_g = [(self.p1_dso.0, 0.), (self.p1_dso.0, self.p1_dso.1)];
        chart
            .draw_series(LineSeries::new(
                curve_theta_g.clone(),
                &RGBColor(150, 150, 0),
            ))
            .map_err(|err| error.err(err.to_string()))?;
        // отрисовка точек кривой плеч кренящего момента
        draw_point(
            &self.dbg,
            &mut chart,
            &[curve_lever[1], curve_lever[2]],
            ShowPoint::All,
            (5, -15),
        )?;
        // первая точка на оси, отображается только значение по y
        draw_point(
            &self.dbg,
            &mut chart,
            &[curve_lever[0]],
            ShowPoint::Y,
            (5, -15),
        )?;
        // отрисовка точек угла макс. разницы
        draw_point(
            &self.dbg,
            &mut chart,
            &[curve_dif[0]],
            ShowPoint::All,
            (5, 5),
        )?;
        draw_point(&self.dbg, &mut chart, &[curve_dif[1]], ShowPoint::X, (0, 5))?;
        // отрисовка точки на оси Х
        draw_point_with_text(
            &self.dbg,
            &mut chart,
            &[curve_theta_g[0]],
            ShowPoint::X,
            "θg=",
            (0, 5),
        )?;
        // надпись с площадью по центру заливки
        let delta_x = self.b.0 - self.p1_dso.0;
        let y_max_area = self
            .dso
            .iter()
            .filter(|v| v.0 <= self.b.0)
            .fold(f64::MIN, |r, v| r.max(v.1));
        let shift = (self.p_40_dso * 1.1 / y_max_area).min(1.2).max(1.);
        let x_mid = self.p1_dso.0 + (delta_x * shift) / 2.;
        let delta_y = y_max_area - self.b.1;
        let y_mid = self.b.1 + delta_y / (2. * shift);
        chart
            .draw_series(PointSeries::of_element(
                [(x_mid, y_mid)],
                3,
                &RGBColor(0, 0, 0),
                &|c, _, _| {
                    return EmptyElement::at(c)
                        + Text::new(
                            format!("S={:.3}", self.area),
                            (0, 0),
                            ("sans-serif", 14).into_font(),
                        );
                },
            ))
            .map_err(|err| error.err(err.to_string()))?;
        // заливка области
        let mut points = Vec::new();
        points.push(self.p1_dso);
        self.dso
            .iter()
            .filter(|v| v.0 >= self.p1_dso.0 && v.0 <= self.b.0)
            .for_each(|v| points.push(*v));
        points.push((self.b.0, self.p_40_dso));
        chart
            .draw_series(AreaSeries::new(points, self.p1_dso.1, RED.mix(0.2)))
            .unwrap();
        let curve_lever = [(self.p1_dso.0, self.p1_dso.1), (self.b.0, self.b.1)];
        chart
            .draw_series(AreaSeries::new(curve_lever, self.p1_dso.1, RED.mix(0.2)))
            .unwrap();
        match root.present() {
            Ok(_) => Ok(()),
            Err(e) => Err(error.pass(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chart() {
        let result = ChartBulk::new(
            &Dbg::new("ChartBulk", "test"),
            "en".to_owned(),
            &[
                (0., 0.),
                (5.0, 1.),
                (15.0, 2.),
                (25.0, 2.5),
                (40.0, 2.),
                (60.0, 1.),
            ],
            1.1,
            (5., 1.),
            (40., 0.6),
            2.,
            5.,
        )
        .to_string();
        assert!(result.is_ok());
    }
}
