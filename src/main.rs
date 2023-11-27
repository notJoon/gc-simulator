use gc_simulator::vm::VirtualMachine;

fn main() {
    let vm = VirtualMachine::new(1025, 10.5);
    println!("{:?}", vm);
}
