use charts_rs::{
    BarChart, Box, SeriesCategory,
};

use crate::error::Error;

use super::{misc::{Curve, ICurve}, Content};
//
pub struct ChartStrength {
    target: Vec<(f64, f64, f64)>, //dX, SF, BM
    result: Vec<(f64, f64, f64)>, //dX, SF, BM
}
//
impl ChartStrength {
    //
    pub fn new( 
        target: &[(f64, f64, f64)],
        result: &[(f64, f64, f64)],
    ) -> Self {
        Self{
            target: Vec::from(target),
            result: Vec::from(result),
        }
    }
}
//
impl Content for ChartStrength {
    //
    fn to_string(self) -> Result<String, Error> {
        let (x_calc, (sf_calc, bm_calc)): (Vec<f64>, (Vec<f64>, Vec<f64>))  = self.result.iter().map(|(x, s, b)| (x, (s*0.001, b*0.001))).unzip();
        let (sf_doc, bm_doc): (Vec<_>, Vec<_>)  = self.target.clone().into_iter().map(|(x, s, b)| ((x, s*0.001), (x, b*0.001))).unzip();
        let sf_doc_curve = Curve::new_linear(&sf_doc)?;
        let bm_doc_curve = Curve::new_linear(&bm_doc)?;
        let mut sf_doc = Vec::new();
        let mut bm_doc = Vec::new();
    //    dbg!(&self.result, &self.target, &x_calc);
        let mut start_index = 0;
        for &x in &x_calc {
            if x < self.target[0].0 {
                start_index +=1;
                continue;
            }
            if x > self.target.last().unwrap().0 {
                break;
            }
            sf_doc.push(sf_doc_curve.value(x)?);
            bm_doc.push(bm_doc_curve.value(x)?);
        }
        
        let mut sf_chart = BarChart::new(
            vec![
                ("SF, calc", sf_calc.into_iter().map(|v| v as f32).collect()).into(),
                ("SF, doc", sf_doc.into_iter().map(|v| v as f32).collect()).into(),
            ],
            x_calc.iter().map(|v| format!("{:.2}", v)).collect(),
        );
        sf_chart.title_text = "Прочность, перерезывающие силы".to_string();
        sf_chart.legend_margin = Some(Box {
            top: sf_chart.title_height,
            bottom: 5.0,
            ..Default::default()
        });
        sf_chart.series_list[0].category = Some(SeriesCategory::Line);
        sf_chart.series_list[0].y_axis_index = 0;
        sf_chart.series_list[0].label_show = false;
        sf_chart.series_list[1].category = Some(SeriesCategory::Line);
        sf_chart.series_list[1].y_axis_index = 0;
        sf_chart.series_list[1].label_show = false;
        sf_chart.series_list[1].start_index = start_index;

        sf_chart.y_axis_configs[0].axis_min = Some(-10.);
        sf_chart.y_axis_configs[0].axis_max = Some(10.);
        sf_chart.y_axis_configs[0].axis_formatter = Some("{c} MN".to_string());
            
        let mut bf_chart = BarChart::new(
            vec![
                ("BM, calc", bm_calc.into_iter().map(|v| v as f32).collect()).into(),
                ("BM, doc", bm_doc.into_iter().map(|v| v as f32).collect()).into(),
            ],
            x_calc.iter().map(|v| format!("{:.2}", v)).collect(),
        );
        bf_chart.title_text = "Прочность, изгибающий момент".to_string();
        bf_chart.legend_margin = Some(Box {
            top: bf_chart.title_height,
            bottom: 5.0,
            ..Default::default()
        });
        bf_chart.series_list[0].category = Some(SeriesCategory::Line);
        bf_chart.series_list[0].y_axis_index = 0;
        bf_chart.series_list[0].label_show = false;
        bf_chart.series_list[1].category = Some(SeriesCategory::Line);
        bf_chart.series_list[1].y_axis_index = 0;
        bf_chart.series_list[1].label_show = false;
        bf_chart.series_list[1].start_index = start_index;

        bf_chart.y_axis_configs[0].axis_min = Some(-10.);
        bf_chart.y_axis_configs[0].axis_max = Some(10.);
        bf_chart.y_axis_configs[0].axis_formatter = Some("{c} MH*m".to_string());

        Ok(format!("{}", sf_chart.svg().unwrap() + "\n\n" + &bf_chart.svg().unwrap()))
    }
}
/*
pub fn chart() -> String {
    let mut bar_chart = BarChart::new_with_theme(
        vec![
            ("Evaporation", vec![2.0, 4.9, 7.0, 23.2, 25.6, 76.7, 135.6]).into(),
            (
                "Precipitation",
                vec![2.6, 5.9, 9.0, 26.4, 28.7, 70.7, 175.6],
            )
                .into(),
            ("Temperature", vec![2.0, 2.2, 3.3, 4.5, 6.3, 10.2, 20.3]).into(),
        ],
        vec![
            "Mon".to_string(),
            "Tue".to_string(),
            "Wed".to_string(),
            "Thu".to_string(),
            "Fri".to_string(),
            "Sat".to_string(),
            "Sun".to_string(),
        ],
        THEME_GRAFANA,
    );
    bar_chart.title_text = "Mixed Line and Bar".to_string();
    bar_chart.legend_margin = Some(Box {
        top: bar_chart.title_height,
        bottom: 5.0,
        ..Default::default()
    });
    bar_chart.series_list[2].category = Some(SeriesCategory::Line);
    bar_chart.series_list[2].y_axis_index = 1;
    bar_chart.series_list[2].label_show = true;

    bar_chart
        .y_axis_configs
        .push(bar_chart.y_axis_configs[0].clone());
    bar_chart.y_axis_configs[0].axis_formatter = Some("{c} ml".to_string());
    bar_chart.y_axis_configs[1].axis_formatter = Some("{c} °C".to_string());

    format!("{}", &bar_chart.svg().unwrap())
}*/