use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

///
/// Config
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub app: App,
    pub ic: Ic,
    pub orm: Orm,
}

///
/// App
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub struct App {
    pub version: String,
}

///
/// Orm
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Orm {}

///
/// Ic
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Ic {
    pub admins: Vec<Principal>,
    pub controllers: Vec<Principal>,
}
