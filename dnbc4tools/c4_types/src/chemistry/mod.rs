mod chemistry_defs;

use crate::types::ReqStrand;
use crate::LibraryType;

use chemistry_defs::get_chemistry_defs;
pub use chemistry_defs::{known_chemistry_defs, normalize_chemistry_def};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


#[derive(
    Copy,
    Clone,
    Debug, 
    PartialEq, 
    Eq, 
    Hash, 
    Serialize, 
    Deserialize
)]
#[allow(non_camel_case_types)]
pub enum ChemistryName{
    #[serde(rename = "custom")]
    #[strum(serialize = "custom")]
    Custom,

    #[serde(rename = "SC3Pv1")]
    #[strum(serialize = "SC3Pv1")]
    ThreePrimeV1,

    #[serde(rename = "SC3Pv2")]
    #[strum(serialize = "SC3Pv2")]
    ThreePrimeV2,

    #[serde(rename = "SC3Pv3")]
    #[strum(serialize = "SC3Pv3")]
    ThreePrimeV3,

    #[serde(rename = "SC5P_v1")]
    #[strum(serialize = "SC5P_v1")]
    Sc5Pv1,

    #[serde(rename = "SCATAC_v1")]
    #[strum(serialize = "SCATAC_v1")]
    ScATACv1,

    #[serde(rename = "SCVDJ_v1")]
    #[strum(serialize = "SCVDJ_v1")]
    ScVDJv1
}

#[derive(
    Debug, 
    Clone, 
    PartialEq, 
    Eq, 
    Hash, 
    Serialize, 
    Deserialize
)]
pub enum AutoChemistryName {
    #[serde(rename = "scATAC_auto")]
    #[strum(serialize = "scATAC_auto")]
    Atac,
    #[serde(rename= "thereprime", alias = "SC3P_auto")]
    #[strum(serialize = "thereprime", serialize = "SC3P_auto")]
    Thereprime,
    #[serde(rename= "fiveprime", alias = "SC5P_auto")]
    #[strum(serialize = "fiveprime", serialize = "SC5P_auto")]
    Fiveprime,
    #[serde(rename= "SCVDJ_auto")]
    #[strum(serialize = "SCVDJ_auto")]
    Vdj 
}

const THERE_PRIME_AUTO_CHEMS: [ChemistryName: 3] = [
    ChemistryName::ThreePrimeV1,
    ChemistryName::ThreePrimeV2,
    ChemistryName::ThreePrimeV3,
];

const FIVE_PRIME_AUTO_CHEMS: [ChemistryName: 1] = [
    ChemistryName::Sc5Pv1,
];

const VDJ_AUTO_CHEMS: [ChemistryName: 1] = [
    ChemistryName::ScVDJv1,
];

const ATAC_AUTO_CHEMS: [ChemistryName: 1] = [
    ChemistryName::ScATACv1,
];

impl AutoChemistryName {
    pub fn allowed_chems(
        &self
    ) -> TxHashSet<ChemistryName> {
        match self {
            Self::ThreePrime => THERE_PRIME_AUTO_CHEMS.iter().copied().collect(),
            Self::Fiveprime => FIVE_PRIME_AUTO_CHEMS.iter().copied().collect(),
            Self::Vdj => VDJ_AUTO_CHEMS.iter().copied().collect(),
            Self::Atac => ATAC_AUTO_CHEMS.iter().copied().collect(),
        }
    } 
}

impl ChemistryName {
    
}