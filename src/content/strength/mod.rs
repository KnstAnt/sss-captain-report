use template::Template;

use crate::error::Error;

use super::Content;

pub mod table;
pub mod template;

//
pub struct Strength {
    shear_force: Template,
    bending_moment: Template,
}
//
impl Strength {
    pub fn new(shear_force: Template, bending_moment: Template,) -> Self {
        Self {
            shear_force,
            bending_moment,
        }
    }
    //
    pub fn new_named(
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
            Template::new(
                "SF".to_owned(),
                &sf_result,
                &sf_limit,
            ),
            Template::new(
                "BM".to_owned(),
                &bm_result,
                &bm_limit,
            ),
        )
    }
    //
    pub fn to_string(self) -> Result<String, Error> {
        Ok("# Прочность\n".to_string() + 
            "## Максимальные изгибающие моменты\n" + 
            &self.bending_moment.to_string().map_err(|e| format!("Strength to_string bending_moment error:{}", e))? + "\n" + 
            "## Максимальные перерезывающие силы \n" + 
            &self.shear_force.to_string().map_err(|e| format!("Strength to_string shear_force error:{}", e))?
        )
    }
}
