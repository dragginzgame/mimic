use quote::quote;

//
// just a place to mess around with tests while developing
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        let i = 3;

        println!("{}", quote!(#i));
    }
}