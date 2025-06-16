use derive_more::Display;

mod constant;
mod snake;
mod title;

//
// utils/case
//
// all case operations come through here as we have to include
// multiple crates to get the desired behaviour
//

// Case
#[derive(Clone, Copy, Debug, Display)]
pub enum Case {
    Camel,
    Constant, // adheres to rust constant rules, more strict than UPPER_SNAKE
    Kebab,
    Lower,
    Sentence,
    Snake,
    Title,
    Upper,
    UpperCamel, // or PascalCase
    UpperSnake, // or SCREAMING_SNAKE
    UpperKebab, // or TRAIN-CASE
}

//
// Casing
//

pub trait Casing<T: std::fmt::Display> {
    fn to_case(&self, case: Case) -> String;
    fn is_case(&self, case: Case) -> bool;
}

impl<T: std::fmt::Display> Casing<T> for T
where
    String: PartialEq<T>,
{
    // to_case
    // don't use convert_case:: Lower and Upper because they add spaces, and other
    // unexpected behaviour
    fn to_case(&self, case: Case) -> String {
        use convert_case as cc;
        let s = &self.to_string();

        match case {
            // rust
            Case::Lower => s.to_lowercase(),
            Case::Upper => s.to_uppercase(),

            // custom
            Case::Title => title::to_title_case(s),
            Case::Snake => snake::to_snake_case(s),
            Case::UpperSnake => snake::to_snake_case(s).to_uppercase(),
            Case::Constant => constant::to_constant_case(s).to_uppercase(),

            // convert_case
            Case::Camel => cc::Casing::to_case(s, cc::Case::Camel),
            Case::Kebab => cc::Casing::to_case(s, cc::Case::Kebab),
            Case::Sentence => cc::Casing::to_case(s, cc::Case::Sentence),
            Case::UpperCamel => cc::Casing::to_case(s, cc::Case::UpperCamel),
            Case::UpperKebab => cc::Casing::to_case(s, cc::Case::Kebab).to_uppercase(),
        }
    }

    // is_case
    fn is_case(&self, case: Case) -> bool {
        &self.to_case(case) == self
    }
}
