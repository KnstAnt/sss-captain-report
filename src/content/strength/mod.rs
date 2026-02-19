use template::Template;
use sal_core::{dbg::Dbg, error::Error};
use crate::{db::strength_result::StrengthResultData};
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
            "## Max shear forces\n\n".to_owned(),
            "## Max bending moments\n\n".to_owned(),)
        } else {
            ("# Прочность\n\n".to_owned(),
            "## Максимальные перерезывающие силы\n\n".to_owned(),
            "## Максимальные изгибающие моменты\n\n".to_owned(),)
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
        result: &[StrengthResultData],
    ) -> Self {
        let (sf_result, bm_result): (Vec<_>, Vec<_>) = result
            .iter()
            .map(|v| ((v.x, v.sf * 0.001), (v.x, v.bm * 0.001)))
            .unzip();
        let (sf_limit, bm_limit): (Vec<_>, Vec<_>) = result
            .iter()
            .map(|v| {
                (
                    (v.x, v.sf_limit_low * 0.001, v.sf_limit_high * 0.001),
                    (v.x, v.bm_limit_low * 0.001, v.bm_limit_high * 0.001),
                )
            })
            .unzip();
        Self::new(
            language,
            Template::new(
                language, 
                "SF",
                "MN",
                &sf_result,
                &sf_limit,
            ),
            Template::new(
                language,
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
            &self.header_sf + 
            &self.shear_force.to_string().map_err(|e| format!("Strength to_string shear_force error:{}", e))? + "\n" + 
            &self.header_bm + 
            &self.bending_moment.to_string().map_err(|e| format!("Strength to_string bending_moment error:{}", e))?
        )
    }
}
