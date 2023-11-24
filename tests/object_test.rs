#[cfg(test)]
mod object_tests {
    use gc_simulator::{
        gc::TriColor,
        object::{Object, ObjectTrait, TypeValue},
    };

    #[test]
    fn test_create_new_object() {
        let obj1 = Object::new(String::from("test"), TypeValue::Int(1));
        assert_eq!(obj1.ident, "test");
        assert_eq!(obj1.value, Some(TypeValue::Int(1)));
        assert_eq!(obj1.header.size, 144);
        assert_eq!(obj1.header.marked, TriColor::White);
        assert_eq!(obj1.references.len(), 0);

        println!("addr: {:?}", obj1.addr);
    }

    #[test]
    fn test_create_random_object() {
        let obj1 = Object::create_random_object();
        println!("{}", obj1.to_string());
        println!("addr: {:?}", obj1.get_address());
    }
}
