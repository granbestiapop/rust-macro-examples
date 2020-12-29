use proc_macro::TokenStream;
use quote::quote;
use quote::TokenStreamExt;
use std::path::PathBuf;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parenthesized, parse_macro_input, Fields, Ident, Item, LitStr, Token};

#[proc_macro]
pub fn ast_parse_sample(_item: TokenStream) -> TokenStream {
    let token = "struct A {a:String}".parse().unwrap();
    token
}

struct InputParse {
    path: String,
    file: String,
}

impl Parse for InputParse {
    fn parse(stream: ParseStream) -> Result<Self> {
        if stream.is_empty() {
            panic!("missing arguments")
        }
        if stream.peek(LitStr){
            let lit: LitStr = stream.parse().unwrap();
            return Ok(InputParse {
                path: ".".to_string(),
                file: lit.value(),
            })
        }
        let content;
        let _env: Ident = stream.parse().unwrap();
        let _: Token!(!) = stream.parse().unwrap();
        let _parent: syn::token::Paren = parenthesized!(content in stream);
        let env_value: LitStr = content.parse().unwrap();
        let _: Token!(/) = stream.parse().unwrap();
        let file: LitStr = stream.parse().unwrap();
        let env_var = std::env::var(env_value.value()).unwrap();
        Ok(InputParse {
            path: env_var,
            file: file.value(),
        })
    }
}

#[proc_macro]
pub fn add_serde(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as InputParse);
    // Read file
    let file = input.file;
    let path = input.path;
    let path = format!("{}/{}", path, file);
    println!("path: {}", path);
    let path: PathBuf = path.parse().unwrap();
    let code = std::fs::read_to_string(path).unwrap();

    // Parse AST
    let ast = syn::parse_file(&code).unwrap();
    let mut q = quote! {};
    let structs = ast.items.iter().filter_map(|item| match item {
        Item::Struct(ref struct_item) => {
            iterate_over(struct_item);
            Some(quote! {
                #[derive(Serialize, Deserialize)]
                #item
            })
        }
        _ => Some(quote! {
            #item
        }),
    });
    q.append_all(structs);

    let out = quote! {
        #q
    };
    out.into()
}

#[proc_macro_attribute]
pub fn makrolead(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item: Item = syn::parse(input).expect("failed to parse input into `syn::Item`");
    match item {
        Item::Struct(ref struct_item) => {
            iterate_over(struct_item);
        }
        Item::Mod(ref module) => {
            let (_b, items) = module.content.as_ref().unwrap();
            items.iter().for_each(|item| match item {
                Item::Struct(ref struct_item) => {
                    iterate_over(struct_item);
                }
                _ => (),
            });
        }
        _ => (),
    }

    let output = quote! {
        #[derive(Serialize, Deserialize)]
        #item
    };
    println!("{}", output.to_string());
    output.into()
}

fn iterate_over(struct_: &syn::ItemStruct) -> bool {
    println!("iterating over: {}", struct_.ident);
    match struct_.fields {
        Fields::Named(ref fields) => {
            fields.named.iter().for_each(|a| {
                println!("Field: {}", a.ident.as_ref().unwrap());
            });
            true
        }
        _ => false,
    }
}
