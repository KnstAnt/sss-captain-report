//! Кривая, позволяет получать интерполированные значения
use splines::{Interpolation, Key, Spline};

use crate::error::Error;

/// Представление кривой в виде массива пар значений
/// - Обеспечивает получение промежуточных значений с помощью простой линейной интерполяции
#[derive(Clone, Debug)]
pub struct Curve {
    spline: Spline<f64, f64>,
}
//
impl Curve {
    /// Creates new instance of the Curve with linear interpolation  
    /// from vector of the key - value pairs
    pub fn new_linear(src: &[(f64, f64)]) -> Result<Curve, Error> {
        if src.len() <= 1 {
            return Err(Error::FromString(
                "Curve new_linear error: src.len() <= 1".to_string(),
            ));
        }
        let src: Vec<_> = src
            .iter()
            .map(|v| Key::new(v.0, v.1, Interpolation::Linear))
            .collect();
        Ok(Self {
            spline: Spline::from_vec(src),
        })
    }
}

impl ICurve for Curve {
    /// Возвращает значение из таблицы по его ключу
    /// - если такого ключа нет, то возвращает промежуточное значение между двумя соседними с помощью линейной интерполяции
    /// - если ключ за пределами ключей таблицы, то вернет либо первое либо последнее значение
    /// - panic - если нет ключей
    fn value(&self, key: f64) -> Result<f64, Error> {
        let res = self.spline.clamped_sample(key).ok_or(format!(
            "Curve value spline.clamped_sample(key) error: key:{key} spline:{:?}",
            self.spline
        ))?;
        //    log::info!("\t Curve clamped_value key:{key} res:{res}");
        Ok(res)
    }
}

#[doc(hidden)]
///
/// Interface used for testing purposes only
pub trait ICurve {
    fn value(&self, _: f64) -> Result<f64, Error>;
}
#[doc(hidden)]
// заглушка для тестирования
pub struct FakeCurve {
    value: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeCurve {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}
#[doc(hidden)]
impl ICurve for FakeCurve {
    fn value(&self, _: f64) -> Result<f64, Error> {
        Ok(self.value)
    }
}
