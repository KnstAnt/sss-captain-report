use crate::error::Error;
use charts_rs::{Box, LineChart, SeriesCategory};
//
pub struct Chart {
    //    header: String,
    short_name: String,
    unit: String,
    values: Vec<(f64, f64, f64, f64)>, //x, min, calc, max
}
//
impl Chart {
    //
    pub fn new(
        //       header: String,
        short_name: &str,
        unit: &str,
        values: &[(f64, f64, f64, f64)],
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
        let (x, (mut min, (mut calc, mut max))): (Vec<f64>, (Vec<f64>, (Vec<f64>, Vec<f64>))) = self
            .values
            .into_iter()
            .map(|(x, min, calc, max)| (x, (min, (calc, max))))
            .unzip();
        let mut chart = LineChart::new(
            vec![
                ("мин", min.iter().map(|v| *v as f32).collect()).into(),
                ("расчет", calc.iter().map(|v| *v as f32).collect()).into(),
                ("макс", max.iter().map(|v| *v as f32).collect()).into(),
            ],
            x.iter().map(|v| format!("{:.3}", v)).collect(),
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
        chart.y_axis_configs[0].axis_formatter = Some(format!("{{c}} {}", self.unit));
        
        Ok(format!("{}", chart.svg().unwrap()))
    }
}
