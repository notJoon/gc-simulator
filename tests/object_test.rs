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
        let obj1 = Object::create_random_object(Some("obj1"));
        println!("{}", obj1.to_string());
        println!("addr: {:?}\n", obj1.get_address());

        println!("--------");

        let obj2 = Object::create_random_object(Some("obj2"));
        println!("{}", obj2.to_string());
        println!("addr: {:?}\n", obj2.get_address());

        println!("--------");

        let obj3 = Object::create_random_object(Some("obj3"));
        println!("{}", obj3.to_string());
        println!("addr: {:?}\n", obj3.get_address());
    }

    #[test]
    fn test_inject_address() {
        let mut obj1 = Object::create_random_object(Some("obj1"));
        let obj2 = Object::create_random_object(Some("obj2"));

        obj1.inject_address(obj2.get_address());
        assert_eq!(obj1.addr, obj2.get_address());
    }
}
