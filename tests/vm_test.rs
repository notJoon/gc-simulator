#[cfg(test)]
mod vm_tests {
    use gc_simulator::{
        object::{TypeValue, Object, ObjectTrait},
        vm::{VMError, VMTrait, VM, OpCode},
    };

    static THRESHOLD: f64 = 0.75;

    #[test]
    fn test_new_vm() {
        let max_stack_size = 10;
        let vm = VM::new(max_stack_size, THRESHOLD).unwrap();

        assert_eq!(vm.max_stack_size, max_stack_size);
        assert_eq!(vm.stack, vec![]);
        assert_eq!(vm.first_object, None);
        assert_eq!(vm.num_objects, 0);
    }

    #[test]
    fn test_max_stack_size_exceed_max_int() {
        let max_stack_size = usize::MAX;
        let vm = VM::new(max_stack_size, THRESHOLD).unwrap();

        assert_eq!(vm.max_stack_size, max_stack_size);
        assert_eq!(vm.stack, vec![]);
        assert_eq!(vm.first_object, None);
        assert_eq!(vm.num_objects, 0);
    }

    #[test]
    fn test_push_objects_to_vm() {
        let max_stack_size = 10;
        let mut vm = VM::new(max_stack_size, THRESHOLD).unwrap();

        for i in 0..max_stack_size-1 {
            let value = Object::new(
                String::from(format!("test{}", i)), 
                TypeValue::Int(i as i32)
            );
            vm.push(value).unwrap();
        }

        assert_eq!(vm.len(), max_stack_size-1);
    }

    #[test]
    fn test_stack_overflow() {
        let max_stack_size = 10;
        let mut vm = VM::new(max_stack_size, THRESHOLD).unwrap();

        for i in 0..max_stack_size {
            let value = Object::new(
                String::from(format!("test{}", i)), 
                TypeValue::Int(i as i32)
            );
            vm.push(value).unwrap();
        }

        assert_eq!(vm.len(), max_stack_size);
        assert_eq!(
            vm.push(Object::new(String::from("test"), 
            TypeValue::Int(1))).unwrap_err(), 
            VMError::StackOverflow
        );
    }

    #[test]
    fn test_pop() {
        let max_stack_size = 10;
        let mut vm = VM::new(max_stack_size, THRESHOLD).unwrap();

        for i in 0..max_stack_size {
            let value = Object::new(
                String::from(format!("test{}", i)), 
                TypeValue::Int(i as i32)
            );
            vm.push(value).unwrap();
        }

        assert_eq!(vm.len(), max_stack_size);

        for i in (0..max_stack_size).rev() {
            let value = Object::new(
                String::from(format!("test{}", i)), 
                TypeValue::Int(i as i32)
            );
            assert_eq!(vm.pop().unwrap(), value);
        }

        assert_eq!(vm.len(), 0);
    }

    #[test]
    fn stack_underflow() {
        let max_stack_size = 10;
        let mut vm = VM::new(max_stack_size, THRESHOLD).unwrap();

        assert_eq!(
            vm.pop().unwrap_err(), 
            VMError::StackUnderflow
        );
    }

    #[test]
    fn test_op_code() {
        let max_stack_size = 10;
        let mut vm = VM::new(max_stack_size, THRESHOLD).unwrap();

        let value = Object::new(
            String::from("test"), 
            TypeValue::Int(1)
        );
        vm.push(value).unwrap();
        vm.pop().unwrap();

        assert_eq!(vm.op_codes.len(), 2);
        assert_eq!(vm.op_codes[0], OpCode::Push(TypeValue::Int(1)));
        assert_eq!(vm.op_codes[1], OpCode::Pop);
    }

    #[test]
    fn test_vm_debug_string() {
        let max_stack_size = 10;
        let mut vm = VM::new(max_stack_size, THRESHOLD).unwrap();

        let value = Object::new(
            String::from("test"), 
            TypeValue::Int(1)
        );
        vm.push(value).unwrap();
        vm.pop().unwrap();
        vm.pop().unwrap_err();

        assert_eq!(
            vm.to_string(), 
            "VM: {\nstack: [],\nop_codes: [Push(Int(1)), Pop, Halt],\nmax_stack_size: 10,\nthreshold: 7,\nnum_objects: 0,\nfirst_object: None,\ngc_confidence: 0,\ntrigger_gc: false,\ngc_status: Idle\n}");
    }
}
