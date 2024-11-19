use crate::{CountMetric, JsonReport, JsonReporter, Metric};
use serde_json::Value;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default, Metric)]
pub struct PercentMetric {
    pub numerator: CountMetric,
    pub denominator: CountMetric,
}

impl PercentMetric {
    pub fn from_parts<T>(num: T, den: T) -> Self
    where
        T: Into<CountMetric>,
    {
        Self {
            numerator: num.into(),
            denominator: den.into(),
        }
    }

    pub fn increment(&mut self, filter: bool) {
        self.denominator.increment();
        if filter {
            self.numerator.increment();
        }
    }

    pub fn increment_by(&mut self, val: T, filter: bool)
    where
        T: Copy + Into<i64>,
    {
        self.denominator.increment_by(val);
        if filter {
            self.numerator.increment_by(val);
        }
        
    }
}