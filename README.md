

```
code_scan_rs 0.1.0
Rust code AST parser

USAGE:
    code_scan_rs <codebase>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <codebase>    The path to the codebase directory
```


https://docs.rs/syn/1.0.80/syn/visit/trait.Visit.html
The syn::visit::Visit trait provides a wide range of methods that you can implement to visit different types of syntax tree nodes in a Rust codebase. Some commonly used visitor methods are:

visit_item_mod: Visit module declarations.
visit_item_struct: Visit struct declarations.
visit_item_enum: Visit enum declarations.
visit_item_impl: Visit impl blocks.
visit_item_trait: Visit trait declarations.
visit_item_const: Visit const items.
visit_item_static: Visit static items.
visit_item_macro: Visit macro items.
visit_item_macro2: Visit macro_rules! items.
visit_item_type: Visit type alias items.
visit_item_use: Visit use items.
visit_item_extern_crate: Visit extern crate items.
visit_item_foreign_mod: Visit foreign module items.
visit_pat_ident: Visit identifier patterns.
visit_pat_tuple_struct: Visit tuple struct patterns.
visit_pat_tuple: Visit tuple patterns.
visit_pat_struct: Visit struct patterns.
visit_pat_box: Visit box patterns.
visit_pat_ref: Visit reference patterns.
visit_pat_lit: Visit literal patterns.
visit_pat_range: Visit range patterns.
visit_pat_slice: Visit slice patterns.
visit_pat_macro: Visit macro patterns.
visit_pat_path: Visit path patterns.
visit_pat_type: Visit type patterns.
visit_pat_wild: Visit wildcard patterns.
These are just a few examples. The full list of visitor methods can be found in the syn::visit::Visit trait documentation. By implementing these methods, you can traverse and analyze various parts of the Rust syntax tree, depending on your needs.