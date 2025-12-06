#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut vm = sable::vm::Vm::new();
    let _ = vm.run_chunk(data);
});
