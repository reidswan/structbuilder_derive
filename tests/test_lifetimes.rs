use structbuilder_derive::*;

#[derive(StructBuilder)]
struct LifetimesStruct<'a> {
    thing1: &'a str,
}

#[test]
fn test_it_compiles() {}

#[test]
fn test_accessors() {
    let ltstruct = LifetimesStruct::new("this is a string!");

    assert_eq!(ltstruct.thing1(), "this is a string!");
}

#[derive(StructBuilder)]
struct LifetimesStructMulti<'a, 'b> {
    a_str: &'a str,
    a_borrowed_usize: &'b usize,
    an_owned_string: String,
    an_optional_str: Option<&'a str>
}

#[test]
fn test_multi() {
    let the_usize = 100usize;
    let ltstruct = LifetimesStructMulti::new("a", &the_usize, String::from("c"));

    assert_eq!(ltstruct.a_str(), "a");
    assert_eq!(ltstruct.a_borrowed_usize, &100);
    assert_eq!(ltstruct.an_optional_str().as_ref(), None);
}
