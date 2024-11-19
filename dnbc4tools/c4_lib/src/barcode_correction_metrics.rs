use serde::{Deserialize, Serialize};
use metric::{CountMetric, Metric, PercentMetric, TxHashMap};


#[derive(Deserialize, Serialize, Default, Metric, JsonReport)]
pub struct BarcodeCorrectionMetrics {
    corrected_bc: PercentMetric,

}
