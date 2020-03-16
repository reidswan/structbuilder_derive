use structbuilder_derive::*;

#[derive(StructBuilder)]
struct TestStructMultiGeneric<T, U, V> {
    t: T,
    u: U,
    v: Option<V>
}

#[test]
fn test_it_compiles() {}


#[test]
fn test_accessors() {
    let my_struct: TestStructMultiGeneric<_, _, Box<String>> = TestStructMultiGeneric::new(String::from("Reid"), 25usize);
    assert_eq!(my_struct.t(), "Reid");
    assert_eq!(my_struct.u(), &25);
    assert_eq!(my_struct.v(), &None);
}

#[test]
fn test_builders() {
    let my_struct = TestStructMultiGeneric::new(String::from("Reid"), String::from("Swan")).with_v(String::from("Yellow"));
    assert_eq!(my_struct.v(), &Some(String::from("Yellow")));
}