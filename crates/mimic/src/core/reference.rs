use crate::core::{
    Key,
    traits::{
        FieldSearchable, FieldSortable, FieldValue, TypeView, ValidateAuto, ValidateCustom,
        Visitable,
    },
};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

///
/// Reference
///

#[derive(Clone, Debug, CandidType, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Reference {
    key: Key,
    canister_pid: Principal,
}

impl Reference {
    #[must_use]
    pub const fn new(key: Key, canister_pid: Principal) -> Self {
        Self { key, canister_pid }
    }

    /*
    pub fn encode(&self) -> Principal {
        let mut array = Vec::new();
        array.extend_from_slice(b"\x0Acat");
        array.extend_from_slice(&self.canister_pid.as_slice());
        array.extend_from_slice(&Self::to_u32_be_bytes(self.key));

        Ok(Principal::from_slice(&array))
    }

    pub fn decode(encoded_identifier: &Principal) -> (u64, Principal, String) {
        let mut p = encoded_identifier.as_slice().to_vec();
        let custom_identifier = p[..4].to_vec();

        if custom_identifier == b"\x0Acat".to_vec() {
            p.drain(..4);
            let kind = String::from_utf8(p[..3].to_vec()).unwrap();
            p.drain(..3);

            let index_bytes = p.drain(p.len() - 4..).collect::<Vec<u8>>();
            let index = Self::from32bits(&index_bytes);
            return (index, Principal::from_slice(&p), kind);
        } else {
            return (0, *encoded_identifier, "principal".to_string());
        }
    }

    pub fn id(encoded_identifier: &Principal) -> u64 {
        Self::decode(encoded_identifier).0
    }

    pub fn principal(encoded_identifier: &Principal) -> Principal {
        Self::decode(encoded_identifier).1
    }

    pub fn kind(encoded_identifier: &Principal) -> String {
        Self::decode(encoded_identifier).2
    }

    fn to_u32_be_bytes(n: u64) -> [u8; 4] {
        [(n >> 24) as u8, (n >> 16) as u8, (n >> 8) as u8, n as u8]
    }

    fn from32bits(ba: &[u8]) -> u64 {
        let mut value = 0;
        for i in 0..4 {
            value = (value << 8) | (ba[i] as u64);
        }
        value
    }
    */
}

impl Default for Reference {
    fn default() -> Self {
        Self {
            key: Key::MIN,
            canister_pid: Principal::anonymous(),
        }
    }
}

impl Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "key: {} - canister: {}", self.key, self.canister_pid,)
    }
}

impl FieldSearchable for Reference {}

impl FieldSortable for Reference {}

impl FieldValue for Reference {}

impl TypeView for Reference {
    type View = Self;

    fn to_view(&self) -> Self::View {
        self.clone()
    }

    fn from_view(view: Self::View) -> Self {
        view
    }
}

impl ValidateAuto for Reference {}

impl ValidateCustom for Reference {}

impl Visitable for Reference {}
