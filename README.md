# structbuilder_derive &emsp; [![Latest Version]][crates.io] 

[Latest Version]: https://img.shields.io/crates/v/structbuilder_derive.svg
[crates.io]: https://crates.io/crates/structbuilder_derive

A simple [rust-lang](https://www.rust-lang.org) macro to derive a `_Builder` trait for your struct. The trait lets you build up a struct in a modular fashion, and provides read-only reference access to members. 

## Usage

Add the crate as a dependency in your Cargo.toml:

```
[dependencies]
structbuilder_derive = "0.2.1"
```


The simply import the deriver and add the `derive(StructBuilder)` directive as follows:

```
use structbuilder_derive::*;

#[derive(StructBuilder)]
pub struct MyStruct {
    name: String,
    age: usize,
    partner_name: Option<String>
}
```

This will create a `<StructName>Builder` (e.g. `MyStructBuilder` trait), which provides a `::new(...)` method, which takes every non-`Option` field as a parameter to create the basic struct. Also included are instance methods `<field_name>()`, which returns a reference to the field called `<field_name>`, and `with_<field_name>(...)` for every field, which sets the value of `<field_name>` to the provided value. For example:

```
<assuming MyStruct as defined above>

fn main() {
    let name = String::from("Reid");
    let age = 24;
    let s = MyStruct::new(name, age);

    println!("{}", s.age()); // -> 24
    println!("{:?}", s.partner_name()); // -> None

    let s2 = MyStruct::new(String::from("Jack"), 21).with_partner_name(String::from("Sally"));

    println!("{}", s2.age()); // -> 21
    println!("{:?}", s.partner_name()); // -> Some("Sally")

}
```

The deriver also provides camelCase fields with snake_case accessors and setters; this is convenient with, for example, JSON and serde, where the external resource might provide a data shape like `{"userId": 12, "userName": "f.mal"}`; you can define your struct to match this, and allow serde to serialize and deserialize this structure, while allowing your code to use Rust-appropriate `.user_id()` and `.with_user_name(...)`.

## Features
- `new(..)` method for non-`Option`al fields
- read-only, borrowing accessor methods for every declared field 
- `with_<field_name>(..)` builder methods that consume and return `self` for every field
- support for `Option<T>` types 
  - not provided in the `new(..)` method
  - `with_<field>` takes a `T` as a parameter if `<field>: Option<T>`
- support for generics and lifetimes 

## Notes and warnings

- only supported for named structs. 
  - `struct X(...)` will be rejected
- analysis of `Option` types is hacky. Using any type called `Option` in your struct will result in the macro treating it as `std::option::Option`
- Definitely not production-ready. 
- The lifetime name `'__sbderive` is used internally in accessor methods; attempting to use this lifetime will result in name conflicts.