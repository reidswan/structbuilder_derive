use structbuilder_derive::*;

#[derive(StructBuilder, Debug)]
struct MyTestStruct {
    name: String,
    age: usize,
    weight: Option<usize>,
    married: Option<bool>,
}

/**
 * Simply successfully compiling is a test case
 */
#[test]
fn test_it_compiles() {}

#[test]
fn test_accessors() {
    let my_struct = MyTestStruct::new(String::from("Reid"), 25);
    assert_eq!(my_struct.name(), "Reid");
    assert_eq!(my_struct.age(), &25);
    assert_eq!(my_struct.weight(), &None);
    assert_eq!(my_struct.married(), &None);
}

#[test]
fn test_builders() {
    let my_struct = MyTestStruct::new(String::from("Reid"), 25).with_weight(100);
    assert_eq!(my_struct.weight(), &Some(100));
    assert_eq!(my_struct.married(), &None);
}

#[test]
fn test_composition_chain() {
    let my_struct = MyTestStruct::new(String::from("Reid"), 25)
        .with_weight(100)
        .with_weight(150)
        .with_weight(75)
        .with_married(false)
        .with_weight(60);

    assert_eq!(my_struct.weight(), &Some(60));
    assert_eq!(my_struct.married(), &Some(false));
    
}

#[cfg(test)]
mod supermod {
    mod innermod {
        use structbuilder_derive::*;
        #[derive(StructBuilder)]
        pub struct InnerStruct {
            pub basic_thing: usize
        }
    }

    use innermod::InnerStruct;

    #[test]
    fn test_accessors_without_trait_import() {
        let my_struct = InnerStruct { basic_thing: 2 };

        my_struct.basic_thing();
    }
}
