#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        let mut vm = sable::vm::Vm::new();
        let _ = vm.eval_str(text);
    }
});
