use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// Config
/// nothing here yet, but its coded so that's nice
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub orm: Orm,
}

///
/// Orm
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Orm {}
