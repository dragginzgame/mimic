use crate::types;

///
/// EnumHash
///
/// a EnumHash is a constant value hashed from a fixture's name and
/// variant.  This means we can connect a specific Entity to backend code
/// and not have to rely on a ULID which could be regenerated
///

pub type EnumHash = types::U64;

///
/// EntityPath
///

pub type EntityPath = String;
