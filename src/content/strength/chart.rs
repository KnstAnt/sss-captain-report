use crate::error::Error;
use charts_rs::{BarChart, Box, SeriesCategory};
//
pub struct Chart {
    header: String,
    short_name: String,
    unit: String,
    values: Vec<(i32, f64, f64, f64, f64)>, //fr, min, doc, calc, max
}
//
impl Chart {
    //
    pub fn new(
        header: String,
        short_name: String,
        unit: String,
        values: &[(i32, f64, f64, f64, f64)],
    ) -> Self {
        Self {
            header,
            short_name,
            unit,
            values: Vec::from(values),
        }
    }
    //
    pub fn to_string(self) -> Result<String, Error> {
        let (fr, (min, (doc, (calc, max)))): (
            Vec<i32>,
            (Vec<f64>, (Vec<f64>, (Vec<f64>, Vec<f64>))),
        ) = self
            .values
            .into_iter()
            .map(|(fr, min, doc, calc, max)| (fr, (min, (doc, (calc, max)))))
            .unzip();
        let mut chart = BarChart::new(
            vec![
                ("мин", min.into_iter().map(|v| v as f32).collect()).into(),
                ("док", doc.into_iter().map(|v| v as f32).collect()).into(),
                ("расчет", calc.into_iter().map(|v| v as f32).collect()).into(),
                ("макс", max.into_iter().map(|v| v as f32).collect()).into(),
            ],
            fr.iter().map(|v| format!("{v}")).collect(),
        );
        chart.title_text = self.header;
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
        chart.series_list[3].category = Some(SeriesCategory::Line);
        chart.series_list[3].y_axis_index = 0;
        chart.series_list[3].label_show = false;

        chart.y_axis_configs[0].axis_min = Some(-10.);
        chart.y_axis_configs[0].axis_max = Some(10.);
        chart.y_axis_configs[0].axis_formatter = Some(format!("{}, {}", self.short_name, self.unit));

        Ok(format!("{}", chart.svg().unwrap()))
    }
}
