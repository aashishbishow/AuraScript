extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    LitStr, Result, Token,
};

// Define a custom struct to parse function names
// #[derive(Debug)]
struct FunctionList {
    functions: Vec<LitStr>,
}

impl Parse for FunctionList {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed: Punctuated<LitStr, Token![,]> = 
            Punctuated::parse_terminated(input)?;
        Ok(FunctionList {
            functions: parsed.into_iter().collect()
        })
    }
}

// Improved print macro with error handling
#[proc_macro]
pub fn print(input: TokenStream) -> TokenStream {
    let value = parse_macro_input!(input as LitStr);
    let expanded = quote! {
        fn print_message() -> Result<(), std::io::Error> {
            println!("{}!", #value);
            Ok(())
        }
    };
    TokenStream::from(expanded)
}

// Enhanced input macro with better error handling
#[proc_macro]
pub fn input(input: TokenStream) -> TokenStream {
    let prompt = parse_macro_input!(input as LitStr);
    let expanded = quote! {
        fn get_input() -> Result<String, std::io::Error> {
            use std::io::{self, Write};
            
            print!("{}", #prompt);
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            Ok(input.trim().to_string())
        }
    };
    TokenStream::from(expanded)
}

// Updated execute macro with error handling and improved function execution
#[proc_macro]
pub fn execute(input: TokenStream) -> TokenStream {
    let function_list = parse_macro_input!(input as FunctionList);
    let function_calls = function_list.functions.iter().map(|func| {
        let func_str = func.value();
        match func_str.as_str() {
            "print" => quote! { 
                print_message().expect("Failed to print message"); 
            },
            "input" => quote! { 
                let user_input = get_input().expect("Failed to get input");
                println!("You entered: {}", user_input);
            },
            _ => quote! { 
                compile_error!("Unknown function: {}", #func_str); 
            }
        }
    });

    let expanded = quote! {
        fn main() -> Result<(), Box<dyn std::error::Error>> {
            #(#function_calls)*
            Ok(())
        }
    };
    TokenStream::from(expanded)
}