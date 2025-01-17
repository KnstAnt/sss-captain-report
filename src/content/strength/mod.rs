use template::Template;

use crate::error::Error;

use super::Content;

pub mod table;
pub mod chart;
pub mod template;

//
pub struct Strength {
    header_main: String,
    header_sf: String,
    header_bm: String,
    shear_force: Template,
    bending_moment: Template,
}
//
impl Strength {
    pub fn new(language: &String, shear_force: Template, bending_moment: Template,) -> Self {
        let (header_main, header_sf, header_bm) = if language.contains("en") {
            ("# Strength\n\n".to_owned(),
            "## Max bending moments\n\n".to_owned(),
            "## Max shear forces\n\n".to_owned())
        } else {
            ("# Прочность\n\n".to_owned(),
            "## Максимальные изгибающие моменты\n\n".to_owned(),
            "## Максимальные перерезывающие силы\n\n".to_owned())
        };
        Self {
            header_main,
            header_sf,
            header_bm,
            shear_force,
            bending_moment,
        }
    }
    //
    pub fn from(
        language: &String, 
        // x, sf, bm
        result: &[(f64, f64, f64)],
        // (frame_x, bm_min, bm_max, sf_min, sf_max)
        limit: &[(f64, f64, f64, f64, f64)],
    ) -> Self {
        let (sf_result, bm_result): (Vec<_>, Vec<_>) = result
            .iter()
            .map(|(x, sf, bm)| ((*x, *sf * 0.001), (*x, *bm * 0.001)))
            .unzip();
        let (sf_limit, bm_limit): (Vec<_>, Vec<_>) = limit
            .iter()
            .map(|(x, bm_min, bm_max, sf_min, sf_max)| {
                (
                    (*x, *sf_min * 0.001, *sf_max * 0.001),
                    (*x, *bm_min * 0.001, *bm_max * 0.001),
                )
            })
            .unzip();
        Self::new(
            language,
            Template::new(
                "SF",
                "MN",
                &sf_result,
                &sf_limit,
            ),
            Template::new(
                "BM",
                "MH*m",
                &bm_result,
                &bm_limit,
            ),
        )
    }
    //
    pub fn to_string(self) -> Result<String, Error> {
        Ok( self.header_main + 
            &self.header_bm + 
            &self.bending_moment.to_string().map_err(|e| format!("Strength to_string bending_moment error:{}", e))? + "\n\n" + 
            &self.header_sf + 
            &self.shear_force.to_string().map_err(|e| format!("Strength to_string shear_force error:{}", e))?
        )
    }
}
