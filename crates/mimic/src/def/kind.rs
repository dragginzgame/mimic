///
/// EntityIndexDef
///

#[derive(Debug, Clone, Copy)]
pub struct EntityIndexDef {
    pub key: &'static u64,
    pub fields: &'static [&'static str],
    pub unique: bool,
}
