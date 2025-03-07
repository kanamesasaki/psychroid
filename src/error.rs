use crate::common::UnitSystem;
use roots::SearchError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PsychroidError {
    /// Relative humidity is out of range (0.0〜1.0)
    InvalidRelativeHumidity(f64),

    /// Dry-bulb temperature is out of range
    InvalidTDryBulb { t_dry_bulb: f64, unit: UnitSystem },

    /// その他の無効なパラメータ
    InvalidParameter(String),

    /// 数値計算におけるエラー (ニュートン法など)
    CalculationError(String),
}

impl fmt::Display for PsychroidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRelativeHumidity(value) => write!(
                f,
                "Invalid relative humidity: {}. Value must be between 0 and 1",
                value
            ),
            Self::InvalidTDryBulb { t_dry_bulb, unit } => write!(
                f,
                "Dry-bulb temperature {} is out of range for unit {:?}",
                t_dry_bulb, unit
            ),
            Self::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            Self::CalculationError(msg) => write!(f, "Calculation error: {}", msg),
        }
    }
}

impl Error for PsychroidError {}

// Transform roots::SearchError into PsychroidError
impl From<SearchError> for PsychroidError {
    fn from(error: SearchError) -> Self {
        PsychroidError::CalculationError(error.to_string())
    }
}
