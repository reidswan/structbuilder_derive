# structbuilder_derive

A simple [rust-lang](https://www.rust-lang.org) macro to derive a `_Builder` trait for your struct. The trait lets you build up a struct in a modular fashion, and provides read-only reference access to members. 

## Usage

Add the crate as a dependency in your Cargo.toml (coming soon).


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


## Notes and warnings

- only supported for non-generic, named structs. 
  - `struct X<T> {...}` will be rejected
  - `struct X(...)` will be rejected
- analysis of `Option` types is hacky. Using any type called `Option` in your struct will result in the macro treating it as `std::option::Option`
- Definitely not production-ready. 