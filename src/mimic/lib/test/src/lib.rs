//
// ss!
// makes a vec of &str into strings to make tests less verbose
//
#[macro_export]
macro_rules! ss {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}
