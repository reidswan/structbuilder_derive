use structbuilder_derive::*;

#[derive(StructBuilder)]
struct TestStruct<T> {
    thing1: String,
    thing2: T,
    thing3: Option<T>
}

#[test]
fn test_it_compiles() {}

#[test]
fn test_accessors() {
    let my_struct = TestStruct::new(String::from("Reid"), 25usize);
    assert_eq!(my_struct.thing1(), "Reid");
    assert_eq!(my_struct.thing2(), &25);
    assert_eq!(my_struct.thing3(), &None);
}

#[test]
fn test_builders() {
    let my_struct = TestStruct::new(String::from("Reid"), String::from("Swan")).with_thing3(String::from("Yellow"));
    assert_eq!(my_struct.thing3(), &Some(String::from("Yellow")));
}