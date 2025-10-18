use crate::object::ObjUpvalue;
use crate::vm::Vm;

pub fn collect(vm: &mut Vm) {
    for i in 0..vm.top {
        let v = vm.stack[i];
        vm.heap.mark_value(v);
    }
    for i in 0..vm.frames.len() {
        let c = vm.frames[i].closure;
        vm.heap.mark_obj(c);
    }
    let roots = vm.heap.temp_roots.clone();
    for r in roots {
        vm.heap.mark_obj(r);
    }
    let mut uv = vm.open_upvalues;
    while !uv.is_null() {
        vm.heap.mark_obj(uv);
        unsafe {
            uv = (*(uv as *mut ObjUpvalue)).next_open;
        }
    }
    vm.globals.mark(&mut vm.heap);
    vm.heap.trace();
    vm.heap.sweep();
}
