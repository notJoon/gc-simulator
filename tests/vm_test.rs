#[cfg(test)]
mod vm_tests {
    use gc_simulator::{
        object::TypeValue,
        vm::{VMError, VMTrait, VM},
    };

    #[test]
    fn test_new_vm() {
        let max_stack_size = 10;
        let vm = VM::new(max_stack_size);

        assert_eq!(vm.max_stack_size, max_stack_size);
        assert_eq!(vm.stack, vec![]);
        assert_eq!(vm.first_object, None);
        assert_eq!(vm.num_objects, 0);
    }

    #[test]
    fn test_max_stack_size_exceed_max_int() {
        let max_stack_size = usize::MAX;
        let vm = VM::new(max_stack_size);

        assert_eq!(vm.max_stack_size, max_stack_size);
        assert_eq!(vm.stack, vec![]);
        assert_eq!(vm.first_object, None);
        assert_eq!(vm.num_objects, 0);
    }

    #[test]
    fn test_push() {
        let mut vm = VM::new(10);

        let obj = vm.new_object(String::from("test"), TypeValue::Int(1));
        vm.push(obj).unwrap();
        assert_eq!(vm.stack.len(), 1);

        let obj2 = vm.new_object(String::from("test2"), TypeValue::Int(2));
        vm.push(obj2).unwrap();
        assert_eq!(vm.stack.len(), 2);
    }

    #[test]
    fn test_pop() {
        let mut vm = VM::new(10);

        let obj = vm.new_object(String::from("test"), TypeValue::Int(1));
        vm.push(obj).unwrap();
        assert_eq!(vm.stack.len(), 1);

        let obj2 = vm.new_object(String::from("test2"), TypeValue::Int(2));
        vm.push(obj2).unwrap();
        assert_eq!(vm.stack.len(), 2);

        let obj3 = vm.pop().unwrap();
        assert_eq!(obj3, obj2);
        assert_eq!(vm.stack.len(), 1);

        let obj4 = vm.pop().unwrap();
        assert_eq!(obj4, obj);
        assert_eq!(vm.stack.is_empty(), true);
    }

    #[test]
    fn test_stack_overflow() {
        let mut vm = VM::new(1);

        let obj = vm.new_object(String::from("test"), TypeValue::Int(1));
        vm.push(obj).unwrap();
        assert_eq!(vm.stack.len(), 1);

        let obj2 = vm.new_object(String::from("test2"), TypeValue::Int(2));
        let result = vm.push(obj2);
        assert_eq!(result, Err(VMError::StackOverflow));
    }

    #[test]
    fn test_stack_underflow() {
        let mut vm = VM::new(1);

        let result = vm.pop();
        assert_eq!(result, Err(VMError::StackUnderflow));
    }

    #[test]
    fn test_new_object() {
        let mut vm = VM::new(10);

        let obj = vm.new_object(String::from("test"), TypeValue::Int(1));
        assert_eq!(vm.first_object, Some(obj));
        assert_eq!(vm.num_objects, 1);

        let obj2 = vm.new_object(String::from("test2"), TypeValue::Int(2));
        assert_eq!(vm.first_object, Some(obj2));
        assert_eq!(vm.num_objects, 2);
    }

    #[test]
    fn test_push_int() {
        let mut vm = VM::new(10);

        let result = vm.push_int(1);
        assert_eq!(result, Ok(1));
        assert_eq!(vm.stack.len(), 1);

        let result = vm.push_int(2);
        assert_eq!(result, Ok(2));
        assert_eq!(vm.stack.len(), 2);
    }
}
