use crate::error::Error;
use charts_rs::{BarChart, Box, SeriesCategory};
//
pub struct Chart {
    //    header: String,
    short_name: String,
    unit: String,
    values: Vec<(i32, f64)>, //angle, dso
}
//
impl Chart {
    //
    pub fn new(
        //       header: String,
        short_name: &str,
        unit: &str,
        values: &[(i32, f64)],
    ) -> Self {
        Self {
            //           header,
            short_name: short_name.to_owned(),
            unit: unit.to_owned(),
            values: Vec::from(values),
        }
    }
    //
    pub fn to_string(self) -> Result<String, Error> {
        let (angle, dso): (Vec<i32>, Vec<f64>) = self
            .values
            .into_iter()
            .map(|(angle, dso)| (angle, dso))
            .unzip();
        let mut chart = BarChart::new(
            vec![
                ("ДСО", dso.into_iter().map(|v| v as f32).collect()).into(),
            ],
            angle.iter().map(|a| format!("{a}")).collect(),
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

        chart.y_axis_configs[0].axis_min = Some(-3.);
        chart.y_axis_configs[0].axis_max = Some(3.);
        chart.y_axis_configs[0].axis_formatter =
            Some(format!("{}, {}", self.short_name, self.unit));

        Ok(format!("{}", chart.svg().unwrap()))
    }
}
