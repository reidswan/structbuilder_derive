extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{GenericArgument, PathArguments, Type, Lifetime};

#[proc_macro_derive(StructBuilder)]
pub fn structbuilder_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_structbuilder(&ast)
}

fn impl_structbuilder(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let interface_name = format_ident!("{}Builder", name);

    let mut non_optional_field_name = vec![];
    let mut non_optional_field_type = vec![];
    let mut optional_field_name = vec![];
    let mut optional_field_type = vec![];
    let mut field_name = vec![];
    let mut field_type = vec![];
    let mut owned_field_name = vec![];
    let mut owned_field_type = vec![];
    let mut borrowed_field_name = vec![];
    let mut borrowed_field_type = vec![];

    match &ast.data {
        syn::Data::Struct(d) => {
            d.fields.iter().for_each(|f| {
                // get the field's declared name
                let ident = f
                    .ident
                    .as_ref()
                    .expect("Tuple-style structs are not supported");

                // check if the field is an Option type for constructors
                if let Some(ty) = get_option_type(&f.ty) {
                    optional_field_type.push(ty);
                    optional_field_name.push(ident);
                } else {
                    non_optional_field_type.push(&f.ty);
                    non_optional_field_name.push(ident);
                }

                // check if the field is borrowed for accessors
                if let Some(_lifetime) = get_lifetime(&f.ty) {
                    borrowed_field_name.push(ident);
                    borrowed_field_type.push(&f.ty);
                } else {
                    owned_field_name.push(ident);
                    owned_field_type.push(&f.ty);
                }

                // for the new method, store all field names and types
                field_name.push(ident);
                field_type.push(&f.ty);
            });
        }
        _ => panic!("Only supported for structs"),
    };

    // convert to a camel case identifier 
    let exposed_optional_field_name = optional_field_name
        .iter()
        .map(|n| format_ident!("{}", camel_to_snake(&n.to_string())))
        .collect::<Vec<_>>();
    let exposed_non_optional_field_name = non_optional_field_name
        .iter()
        .map(|n| format_ident!("{}", camel_to_snake(&n.to_string())))
        .collect::<Vec<_>>();

    // add the with_ prefix for the builder methods
    let builder_method_optional = exposed_optional_field_name
        .iter()
        .map(|n| format_ident!("with_{}", n))
        .collect::<Vec<_>>();
    let builder_method_non_optional = exposed_non_optional_field_name
        .iter()
        .map(|n| format_ident!("with_{}", n))
        .collect::<Vec<_>>();

    // convert to camel case for the accessor
    let exposed_owned_field_name = owned_field_name
        .iter()
        .map(|n| format_ident!("{}", camel_to_snake(&n.to_string())))
        .collect::<Vec<_>>();
    let exposed_borrowed_field_name = borrowed_field_name
        .iter()
        .map(|n| format_ident!("{}", camel_to_snake(&n.to_string())))
        .collect::<Vec<_>>();
    

    let gen = quote! {

        impl #impl_generics #name #ty_generics #where_clause {
            #(pub fn #exposed_owned_field_name <'__sbderive> (&'__sbderive self)-> &'__sbderive #owned_field_type {
                &self. #owned_field_name
            })*

            #(pub fn #exposed_borrowed_field_name(&self)-> #borrowed_field_type {
                self. #borrowed_field_name
            })*
        }

        pub trait #interface_name #ty_generics #where_clause {
            fn new( #(#non_optional_field_name : #non_optional_field_type),* )-> Self;
            
            #(fn #builder_method_optional(self, #optional_field_name: #optional_field_type)-> Self;)*
            
            #(fn #builder_method_non_optional(self, #non_optional_field_name: #non_optional_field_type)-> Self;)*
        }

        impl #impl_generics #interface_name #ty_generics for #name #ty_generics #where_clause {
            fn new(#(#non_optional_field_name : #non_optional_field_type),*)-> Self {
                Self {
                    #( #non_optional_field_name , )*
                    #( #optional_field_name : None ),*
                }
            }

            #(fn #builder_method_optional(mut self, #optional_field_name: #optional_field_type)-> Self {
                self. #optional_field_name = Some( #optional_field_name );
                self
            })*

            #(fn #builder_method_non_optional(mut self, #non_optional_field_name: #non_optional_field_type)-> Self {
                self. #non_optional_field_name = #non_optional_field_name;
                self
            })*
        }
    };

    gen.into()
}

fn camel_to_snake(src: &String)-> String {
    let mut result = String::with_capacity(src.len());
    let mut prev_char = None;
    let mut prev_was_upper = false;
    let mut prev_changed_from_upper = false;
    for ch in src.chars() {
        match prev_char {
            None => {
                prev_char = Some(ch);
                result.push(ch);
                prev_was_upper = ch.is_ascii_uppercase();
                prev_changed_from_upper = false; 
            }
            Some(prev) => {
                if ch.is_ascii_uppercase() {
                    if prev != '_' && !prev_was_upper {
                        result.push('_');
                        result.push(ch.to_ascii_lowercase());
                        prev_changed_from_upper = true;
                    } else if prev_changed_from_upper {
                        let p = result.pop().unwrap().to_ascii_uppercase();
                        result.push(p);
                        result.push(ch);
                        prev_changed_from_upper = false;
                    } else {
                        // prev == '_'
                        result.push(ch);
                        prev_changed_from_upper = false;
                    }
                    prev_was_upper = true;
                } else {
                    prev_changed_from_upper = false;
                    prev_was_upper = false;
                    result.push(ch);
                }
                prev_char = Some(ch)
            }
        }
        
    }

    result
}

// ugly hack to guess if the type is an Option<A> for some concrete A
fn get_option_type(ty: &Type) -> Option<Type> {
    let path_type = match ty {
        Type::Path(pty) => pty,
        _ => return None,
    };
    let last_type = match path_type.path.segments.iter().last() {
        Some(t) => t,
        _ => return None,
    };

    let mut type_args = if last_type.ident == String::from("Option") {
        match &last_type.arguments {
            PathArguments::AngleBracketed(a) => a.args.iter(),
            _ => return None,
        }
    } else {
        return None;
    };

    let optional_first_ty = type_args.next();
    if let None = type_args.next() {
        // there were 0 or 1 type args
        if let Some(GenericArgument::Type(ty)) = optional_first_ty {
            // there was exactly one concrete type arg
            Some(ty.to_owned())
        } else {
            // there was a generic type arg or no type arg or something
            None
        }
    } else {
        // there was a second type argument
        None
    }
}


fn get_lifetime(ty: &Type)-> Option<Lifetime> {
    if let Type::Reference(ty_ref) = ty {
        ty_ref.lifetime.to_owned()
    } else {
        None
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let src = String::from("");
        let expected = src.clone();
        let actual = camel_to_snake(&src);
        assert_eq!(expected, actual); 
    }

    #[test]
    fn test_regular_snake() {
        let src = String::from("abc_def");
        let expected = src.clone();
        let actual = camel_to_snake(&src);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_only_caps() {
        let src = String::from("ABC");
        let expected = src.clone();
        let actual = camel_to_snake(&src);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_screaming_snake() {
        let src = String::from("SCREAMING_SNAKE");
        let expected = src.clone();
        let actual = camel_to_snake(&src);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_camel() {
        let src = String::from("theThingIs");
        let expected = String::from("the_thing_is");
        let actual = camel_to_snake(&src);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_snake_with_some_upper() {
        let src = String::from("is_USB_connected");
        let expected = src.clone();
        let actual = camel_to_snake(&src);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_snake_with_one_upper() {
        let src = String::from("can_I_touch");
        let expected = src.clone();
        let actual = camel_to_snake(&src);
        assert_eq!(expected, actual);
    }
}
