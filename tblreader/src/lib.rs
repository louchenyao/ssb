extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(TBLReader)]
pub fn tblreader(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = ast.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct{fields: syn::Fields::Named(syn::FieldsNamed{named, .. }), ..}) = ast.data {
        named
    } else {
        unimplemented!();
    };

    let newvec = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name: Vec::new()
        }
    });

    let parse_push = fields.iter().map(|f| {
        let name = &f.ident;
        let t = if let syn::Type::Path(syn::TypePath{ref path, ..}) = f.ty {
            let ps = path.segments.last().unwrap();
            if ps.ident != "Vec" {
                unimplemented!();
            }
            if let syn::PathArguments::AngleBracketed(ref ar) = ps.arguments {
                ar.args.first().unwrap()
            } else {
                unimplemented!();
            }
        } else {
            unimplemented!();
        };

        quote! {
            t.#name.push(it.next().unwrap().parse::<#t>().unwrap());
        }
    });

    let expended = quote! {
        impl #ident {
            fn load<P>(p: P) -> Self where P: AsRef<std::path::Path> {
                let mut t = Self {
                    // e.g.: suppkey: Vec::new(),
                   #(#newvec,)*
                };

                use std::io::BufRead;
                let reader = {
                    let f =  std::fs::File::open(p).unwrap();
                    std::io::BufReader::new(f).lines()
                };

                for line in reader {
                    if let Ok(l) = line {
                        let mut it = l.split("|");
                        // e.g.: self.suppkey.push(v[0].parse::<i32>().unwrap());
                        #(#parse_push;)*
                    }
                }
                
                t
            }
        }
    };
    TokenStream::from(expended)
}