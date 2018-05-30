#[macro_use]
extern crate proc_macro_hack;
extern crate syn;

use syn::LitStr;

proc_macro_expr_impl! {
    /// Add one to an expression.
    pub fn include_dir_impl(input: &str) -> String {
        let path: LitStr =  syn::parse_str(input).expect("include_dir!() only accepts a single string argument");

        format!("1 + {}", input)
    }
}
