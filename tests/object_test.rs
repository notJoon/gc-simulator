#[cfg(test)]
mod object_tests {
    use gc_simulator::object::{Object, ObjectTrait};
    use gc_simulator::types::TriColor;

    #[test]
    fn test_new() {
        let object = Object::new(String::from("test"));
        assert_eq!(object.ident, "test");
        assert_eq!(object.reference.len(), 0);
        assert_eq!(object.marked, TriColor::White);
    }

    #[test]
    fn test_add_reference() {
        let mut object = Object::new(String::from("test"));
        let object2 = Object::new(String::from("test2"));
        object.add_reference(object2);
        assert_eq!(object.reference.len(), 1);
    }

    #[test]
    fn test_remove_reference() {
        let mut object = Object::new(String::from("test"));
        let object2 = Object::new(String::from("test2"));
        object.add_reference(object2);
        assert_eq!(object.reference.len(), 1);

        object.delete_reference(object.reference[0].clone());
        assert_eq!(object.reference.len(), 0);
    }

    #[test]
    fn test_is_empty() {
        let mut object = Object::new(String::from("test"));
        assert_eq!(object.is_empty(), true);

        let object2 = Object::new(String::from("test2"));
        object.add_reference(object2);
        assert_eq!(object.is_empty(), false);
    }

    #[test]
    fn test_to_string() {
        let ident = String::from("test");
        let object = Object::new(ident);
        assert_eq!(object.to_string(), "test: White");
    }
}
