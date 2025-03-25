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

 /*       let (x, (mut min, (mut calc, mut max))): (Vec<f64>, (Vec<f64>, (Vec<f64>, Vec<f64>))) = self
            .values
            .into_iter()
            .map(|(x, min, calc, max)| (x, (min, (calc, max))))
            .unzip();
        let mut chart = LineChart::new(
            vec![
                (self.header.0.as_str(), min.iter().map(|v| *v as f32).collect()).into(),
                (self.header.1.as_str(), calc.iter().map(|v| *v as f32).collect()).into(),
                (self.header.2.as_str(), max.iter().map(|v| *v as f32).collect()).into(),
            ],
            x.iter().map(|v| format!("{:.1}", v)).collect(),
        );
        //        chart.title_text = self.header;
        chart.legend_margin = Some(Box {
            top: chart.title_height,
            bottom: 5.0,
            ..Default::default()
        });
        chart.series_list[0].category = Some(SeriesCategory::Line);
        chart.series_list[0].y_axis_index = 0;
        chart.series_list[0].label_show = false;
        chart.series_list[1].category = Some(SeriesCategory::Line);
        chart.series_list[1].y_axis_index = 0;
        chart.series_list[1].label_show = false;
        chart.series_list[2].category = Some(SeriesCategory::Line);
        chart.series_list[2].y_axis_index = 0;
        chart.series_list[2].label_show = false;

        min.sort_by(|a, b| a.partial_cmp(b).unwrap()); 
        calc.sort_by(|a, b| a.partial_cmp(b).unwrap()); 
        max.sort_by(|a, b| a.partial_cmp(b).unwrap()); 
        let minimum = (min.first().unwrap_or(&0.).min(*calc.first().unwrap_or(&0.)).min(*max.first().unwrap_or(&0.)) - 1.).floor();
        let maximum = (min.last().unwrap_or(&0.).max(*calc.last().unwrap_or(&0.)).max(*max.last().unwrap_or(&0.)) + 1.).ceil();
        let (minimum, maximum) = (minimum.min(-maximum), maximum.max(-minimum));
        chart.y_axis_configs[0].axis_min = Some(minimum as f32);
        chart.y_axis_configs[0].axis_max = Some(maximum as f32);
        chart.y_axis_configs[0].axis_formatter = Some(format!("{{c}}"));//, self.unit));
        chart.y_axis_configs[0].axis_width = Some(45.);

        chart.x_axis_name_rotate = 0.6;
        chart.x_axis_height = 50.;

        Ok(format!("{}", chart.svg().unwrap()))
*/
        let path = PathBuf::from(format!("bin/assets/{}_chart.svg", self.short_name.to_lowercase()));
        let root  = SVGBackend::new(&path, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
        let root = root.margin(
            self.y_max, 
            self.y_min, 
            self.x_min, 
            self.x_max,);
        // After this point, we should be able to construct a chart context
        let mut chart = ChartBuilder::on(&root)
            // Set the caption of the chart
        //    .caption("This is our first plot", ("sans-serif", 40).into_font())
            // Set the size of the label region
            .x_label_area_size(40)
            .y_label_area_size(60)
            // Finally attach a coordinate on the drawing area and make a chart context
            .build_cartesian_2d(self.x_min..self.x_max, self.y_min..self.y_max)?;//.map_err(|e| e.into())?;

        chart
            .configure_mesh()
            // We can customize the maximum number of labels allowed for each axis
            .x_labels(10)
            .x_label_formatter(&|x| format!("{:.1}", x))
            .y_labels(10)                     
            .y_label_formatter(&|x| format!("{:.1}", x))
            .draw()?;
    
        chart.draw_series(LineSeries::new(
            self.result.clone(),//.iter().map(|(x, y)| (*x as f32, *y as f32)),
            &RED,
        ))?;
        if self.result.len() <= 30 {
            chart.draw_series(PointSeries::of_element(
                self.result.clone(),//.iter().map(|(x, y)| (*x as f32, *y as f32)),
                3,
                &RED,
                &|c, s, st| {
                    return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                    + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
                    + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
                },
            ))?;
        }
        chart.draw_series(LineSeries::new(
            self.target_min.clone(),//.iter().map(|(x, y)| (*x as f32, *y as f32)),
            &GREEN,
        ))?;
        if self.target_min.len() <= 30 {
            chart.draw_series(PointSeries::of_element(
                self.target_min.clone(),//.iter().map(|(x, y)| (*x as f32, *y as f32)),
                3,
                &GREEN,
                &|c, s, st| {
                    return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                    + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
                    + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
                },
            ))?;
        }
        chart.draw_series(LineSeries::new(
            self.target_max.clone(),//.iter().map(|(x, y)| (*x as f32, *y as f32)),
            &GREEN,
        ))?;
        if self.target_max.len() <= 30 {
            chart.draw_series(PointSeries::of_element(
                self.target_max,//.iter().map(|(x, y)| (*x as f32, *y as f32)),
                3,
                &GREEN,
                &|c, s, st| {
                    return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                    + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
                    + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
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