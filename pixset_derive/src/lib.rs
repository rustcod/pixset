#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use std::iter;

#[proc_macro_derive(PixLike, attributes(size, total))]
pub fn pix_like(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    validate_input(&ast);
    let gen = impl_pix(&ast);
    gen.parse().unwrap()
}

fn validate_input(ast: &syn::MacroInput) {
    if let syn::Body::Struct(_) = ast.body {
        panic!("PixLike only works on Enums, not Structs.")
    }

    if *get_variants(&ast).last().unwrap() != syn::Ident::new("Empty") {
        panic!("Last Enum must be the variant `Empty`, please add it.");
    }

    let _ = get_size_attr(&ast.attrs);
    let _ = get_total_attr(&ast.attrs);
}

fn impl_pix(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let names = iter::repeat(name);
    let names_2 = iter::repeat(name);
    let variants = get_variants(ast);
    let variants_2 = get_variants(ast);
    let size = get_size_attr(&ast.attrs);
    let coords = get_coords(variants.len(), get_total_attr(&ast.attrs));

    quote! {
        impl PixLike for #name {
            fn pix_order() -> Vec<#name> {
                vec![
                    #(#names::#variants),*
                ]
            }

            fn get(&self) -> (f32, f32, f32, f32) {
                match *self {
                    #(#names_2::#variants_2 => #coords),*
                }
            }

            fn tile_size(&self) -> i32 {
                #size
            }
        }

        impl std::default::Default for #name {
            fn default() -> #name {
                #name::Empty
            }
        }
    }
}

fn get_size_attr(attrs: &[syn::Attribute]) -> i32 {
    get_attr(attrs, "size")
}

fn get_total_attr(attrs: &[syn::Attribute]) -> i32 {
    get_attr(attrs, "total")
}

fn get_attr<T>(attrs: &[syn::Attribute], label: &str) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    T: std::str::FromStr,
{
    use syn::{Attribute, Ident, MetaItem};

    match attrs
              .iter()
              .find(
        |attr| match attr.value {
            MetaItem::NameValue(ref ident, _) => *ident == Ident::new(label),
            _ => false,
        },
    ) {
        Some(attr) => {
            if let Attribute { value: MetaItem::NameValue(_, ref s), .. } = *attr {
                if let syn::Lit::Str(ref s, _) = *s {
                    s.parse()
                        .expect(
                            &format!(
                                "Parsing to i32 failed on '{}'. Only Integers are supported",
                                s
                            ),
                        )
                } else {
                    panic!("Only String values are supported on Attributes e.g. `#[size = \"16\"]`");
                }
            } else {
                panic!("Only NameValue Attributes are supported e.g. `#[size = \"16\"]`");
            }
        }
        None => {
            panic!(
                "Must supply a {} attribute e.g. `#[{} = \"16\"]`",
                label,
                label
            )
        }
    }
}

fn get_variants(ast: &syn::MacroInput) -> Vec<syn::Ident> {
    match ast.body {
        syn::Body::Enum(ref variants) => variants.iter().cloned().map(|v| v.ident).collect(),
        _ => unreachable!(),
    }
}

fn get_coords(enum_count: usize, tileset_size: i32) -> Vec<(f32, f32, f32, f32)> {
    let tile_dim = (tileset_size as f32).sqrt() as i32;
    (0..enum_count)
        .map(
            |i| {
                let tex_coords = (i as i32 % tile_dim, i as i32 / tile_dim);
                get_tex_coords(tileset_size, tex_coords)
            },
        )
        .collect::<Vec<_>>()
}

fn get_tex_coords(total_tiles: i32, loc: (i32, i32)) -> (f32, f32, f32, f32) {
    let tile_dim: f32 = (total_tiles as f32).sqrt();
    let per_tile: f32 = 1.0 / tile_dim;

    let top = 1.0 - loc.1 as f32 * per_tile;
    let right = (loc.0 + 1) as f32 * per_tile;
    let bottom = 1.0 - (loc.1 + 1) as f32 * per_tile;
    let left = loc.0 as f32 * per_tile;

    (top, right, bottom, left)
}
