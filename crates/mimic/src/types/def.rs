///
/// EntityIndexDef
///

#[derive(Debug, Clone, Copy)]
pub struct EntityIndexDef {
    pub fields: &'static [&'static str],
    pub unique: bool,
    pub store: &'static str,
}
