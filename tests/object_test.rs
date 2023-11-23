#[cfg(test)]
mod object_tests {
    use gc_simulator::object::{Object, ObjectTrait, TypeValue};
    use gc_simulator::types::TriColor;

    #[test]
    fn test_new() {
        let value = TypeValue::Int(1);
        let object = Object::new(String::from("test"), value);
        assert_eq!(object.ident, "test");
        assert_eq!(object.reference.len(), 0);
        assert_eq!(object.marked, TriColor::White);
    }

    #[test]
    fn test_add_reference() {
        let value1 = TypeValue::Int(1);
        let mut object = Object::new(String::from("test"), value1);

        let value2 = TypeValue::Int(2);
        let object2 = Object::new(String::from("test2"), value2);
        object.add_reference(object2);
        assert_eq!(object.reference.len(), 1);
        println!("{:?}", object)
    }

    #[test]
    fn test_remove_reference() {
        let v1 = TypeValue::Int(1);
        let v2 = TypeValue::Int(2);
        let mut object = Object::new(String::from("test"), v1);
        let object2 = Object::new(String::from("test2"), v2);
        object.add_reference(object2);
        assert_eq!(object.reference.len(), 1);

        object.delete_reference(object.reference[0].clone());
        assert_eq!(object.reference.len(), 0);
    }

    #[test]
    fn test_is_empty() {
        let v1 = TypeValue::Int(1);
        let v2 = TypeValue::Int(2);

        let mut object = Object::new(String::from("test"), v1);
        assert_eq!(object.is_empty(), true);

        let object2 = Object::new(String::from("test2"), v2);
        object.add_reference(object2);
        assert_eq!(object.is_empty(), false);
    }

    #[test]
    fn test_to_string() {
        let ident = String::from("test");
        let value = TypeValue::Int(1000);
        let object = Object::new(ident, value);
        assert_eq!(object.to_string(), "test: White");
    }
}
