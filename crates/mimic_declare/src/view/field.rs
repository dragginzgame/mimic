impl HasViewExpr for FieldList {
    fn view_type_expr(&self) -> TokenStream {
        let fields = self.fields.iter().map(HasViewExpr::view_type_expr);
        quote! {
            #(#fields),*
        }
    }
}
