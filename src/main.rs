use std::{path::PathBuf, fmt};
use quote::ToTokens;
use structopt::StructOpt;
use syn::{File, visit::Visit, ExprLit, Lit, ItemFn, ExprMacro, ItemStruct, ItemEnum};
use walkdir::{WalkDir, DirEntry};


#[derive(StructOpt, Debug)]
#[structopt(name = "code_scan_rs", about = "Rust code AST parser")]
pub struct Cli {
    /// The path to the codebase directory
    #[structopt(parse(from_os_str))]
    codebase: PathBuf,
}

pub struct CodeVisitor {
    functions: Vec<FunctionInfo>,
    types: Vec<TypeInfo>,
}

impl<'ast> Visit<'ast> for CodeVisitor {
    // fn visit_expr_lit(&mut self, expr_lit: &'ast ExprLit) {
    //     match &expr_lit.lit {
    //         Lit::Str(lit_str) => {
    //             let content = lit_str.value();
    //             //Here you can use some heuristics or a simple JSON parser to check if the content looks like an Elasticsearch DSL JSON
    //             if content.len() > 80 { 
    //                 println!("Found: {}", content); 
    //             }         
    //         }
    //         _ => {}
    //     }
    // }
    // fn visit_expr_macro(&mut self, expr_macro: &'ast ExprMacro) {
    //     let macro_path = &expr_macro.mac.path;
    //     if macro_path.is_ident("json") {
    //         let macro_tokens = expr_macro.mac.tokens.to_string();
    //         // if let Some(json_content) = extract_json_from_macro_tokens(&macro_tokens) {
    //             // if is_valid_json(&json_content) {
    //                 println!("Found json! macro: {}", macro_tokens);
    //             // }
    //         }
    //     // }
    // }
    fn visit_item_struct(&mut self, item_struct: &'ast ItemStruct) {
        let name = item_struct.ident.to_string();
        let visibility = item_struct.vis.to_token_stream().to_string();

        let type_info = TypeInfo {
            name,
            kind: "struct".to_string(),
            visibility: if visibility.is_empty() { None } else { Some(visibility) },
        };

        self.types.push(type_info);
    }

    fn visit_item_enum(&mut self, item_enum: &'ast ItemEnum) {
        let name = item_enum.ident.to_string();
        let visibility = item_enum.vis.to_token_stream().to_string();

        let type_info = TypeInfo {
            name,
            kind: "enum".to_string(),
            visibility: if visibility.is_empty() { None } else { Some(visibility) },
        };

        self.types.push(type_info);
    }
    // Implement visit_item_fn to handle function items in the AST
    fn visit_item_fn(&mut self, item_fn: &'ast ItemFn) {
        let name = item_fn.sig.ident.to_string();
        let inputs = item_fn
            .sig
            .inputs
            .iter()
            .map(|arg| arg.to_token_stream().to_string())
            .collect();
        let output = match &item_fn.sig.output {
            syn::ReturnType::Default => "()".to_string(),
            syn::ReturnType::Type(_, ty) => ty.to_token_stream().to_string(),
        };
        let visibility = item_fn.vis.to_token_stream().to_string();
        let is_async = item_fn.sig.asyncness.is_some();

        let function_info = FunctionInfo {
            name,
            is_async,
            inputs,
            output,
            visibility: if visibility.is_empty() { None } else { Some(visibility) },
        };

        self.functions.push(function_info);
    }
}

impl CodeVisitor {
    pub fn create_report(entry: DirEntry, ast: &File) {
        let mut visitor = CodeVisitor{
            functions: Vec::new(),
            types: Vec::new()
        };
        visitor.visit_file(&ast);
        visitor.report(entry);
    }
    pub fn report(self, entry: DirEntry) {
        println!("File: {}", entry.path().display());
        if !self.functions.is_empty() {
            for func_info in self.functions {
                println!("{}\n", func_info);
            }
        }
    
        if !self.types.is_empty() {
            for type_info in self.types {
                println!("{}\n", type_info);
            }
        }
        println!("---");
    }
}

fn main() {
    let args = Cli::from_args();
    let path = args.codebase;

    scan_codebase(&path);
}

fn is_rust_file(entry: &DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s.ends_with(".rs")).unwrap_or(false)
}

fn is_not_target_dir(entry: &DirEntry) -> bool {
    !entry.path().to_str().map(|s| s.contains("target") || s.contains("node_modules")).unwrap_or(false)
}


fn scan_codebase(path: &PathBuf) {
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| is_not_target_dir(e))
        .filter(|e| is_rust_file(e))
    {
        let content = std::fs::read_to_string(entry.path()).unwrap();
        match syn::parse_str(&content) {
            Ok(ast) => CodeVisitor::create_report(entry, &ast),
            Err(err) => {
                eprintln!("{}: {err}", path.display());
            },
        }
    }
}

pub struct TypeInfo {
    name: String,
    kind: String,
    visibility: Option<String>,
}

impl fmt::Display for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let default_private = "private".to_string();
        let visibility = self.visibility.as_ref().unwrap_or(&default_private);
        write!(f, "{} {} {}", visibility, self.kind, self.name)
    }
}

pub struct FunctionInfo {
    name: String,
    is_async: bool,
    inputs: Vec<String>,
    output: String,
    visibility: Option<String>,
}

impl fmt::Display for FunctionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inputs = self.inputs.join(", ");
        let private = &String::from("private");
        let visibility = self.visibility.as_ref().unwrap_or(private);
        let async_keyword = if self.is_async { "async " } else { "" };

        write!(
            f,
            "{} {}fn {}({}) -> {}",
            visibility, async_keyword, self.name, inputs, self.output
        )
    }
}

fn extract_json_from_macro_tokens(macro_tokens: &str) -> Option<String> {
    let json_start = macro_tokens.find('{')?;
    let json_end = macro_tokens.rfind('}')?;
    Some(macro_tokens[json_start..=json_end].to_string())
}

fn is_valid_json(content: &str) -> bool {
    // Replace this with a more robust JSON validation logic if necessary
    content.contains("{") && content.contains("}")
}