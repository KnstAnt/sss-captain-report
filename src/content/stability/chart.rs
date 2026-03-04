use plotters::{coord::types::RangedCoordf64, prelude::*};
use sal_core::{dbg::Dbg, error::Error};
//
pub enum ShowPoint {
    X,
    Y,
    TEXT,
    All,
}
//
pub fn draw_point(
    parent: &Dbg,
    chart: &mut ChartContext<'_, SVGBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    points: &[(f64, f64)],
    //  color: impl Color,
    show: ShowPoint,
    text_shift: (i32, i32),
) -> Result<(), Error> {
    draw_point_with_text(
        &Dbg::new(parent, "draw_point"),
        chart,
        points,
        show,
        "",
        text_shift,
    )
}
//
pub fn draw_point_with_text(
    parent: &Dbg,
    chart: &mut ChartContext<'_, SVGBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    points: &[(f64, f64)],
    //  color: impl Color,
    show: ShowPoint,
    text: &str,
    text_shift: (i32, i32),
) -> Result<(), Error> {
    let error = Error::new(
        &Dbg::new(parent, "draw_point_with_text"),
        "draw_point_with_text",
    );
    chart
        .draw_series(PointSeries::of_element(
            Vec::from(points),
            3,
            &RGBColor(150, 0, 0), // color,
            &|c, s, st| {
                return EmptyElement::at(c)
                    + Circle::new((0, 0), s, st.filled())
                    + Text::new(
                        match show {
                            ShowPoint::X => format!("{text}{:.3}", c.0),
                            ShowPoint::Y => format!("{text}{:.3}", c.1),
                            ShowPoint::TEXT => text.to_owned(),
                            ShowPoint::All => format!("{text}{:.3};{:.3}", c.0, c.1),
                        },
                        text_shift,
                        ("sans-serif", 14).into_font(),
                    );
            },
        ))
        .map_err(|err| error.err(err.to_string()))?;
    Ok(())
}
