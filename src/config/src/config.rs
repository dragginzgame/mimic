use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

///
/// Config
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub ic: Ic,
    pub orm: Orm,
}

///
/// Orm
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct Orm {
    pub hash_salt: String,
}

///
/// Ic
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Ic {
    pub admins: Vec<Principal>,
    pub controllers: Vec<Principal>,
}
