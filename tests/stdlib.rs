use sable::vm::Vm;

fn num(src: &str) -> f64 {
    let mut vm = Vm::new();
    vm.eval_str(src).expect("eval failed").as_number()
}

fn truthy(src: &str) -> bool {
    let mut vm = Vm::new();
    vm.eval_str(src).expect("eval failed").is_truthy()
}

#[test]
fn encoding_hex() {
    assert!(truthy("return hex_encode(\"AB\") == \"4142\";"));
    assert!(truthy("return hex_decode(\"6869\") == \"hi\";"));
    assert!(truthy("return hex_decode(hex_encode(\"hello world\")) == \"hello world\";"));
}

#[test]
fn encoding_base64() {
    assert!(truthy("return base64_encode(\"abc\") == \"YWJj\";"));
    assert!(truthy("return base64_decode(\"YWJj\") == \"abc\";"));
    assert!(truthy("return base64_decode(base64_encode(\"hello world!\")) == \"hello world!\";"));
}

#[test]
fn bitwise() {
    assert_eq!(num("return band(12, 10);"), 8.0);
    assert_eq!(num("return bor(12, 10);"), 14.0);
    assert_eq!(num("return bxor(12, 10);"), 6.0);
    assert_eq!(num("return shl(1, 4);"), 16.0);
    assert_eq!(num("return shr(256, 2);"), 64.0);
    assert_eq!(num("return gcd(48, 36);"), 12.0);
}

#[test]
fn json_roundtrip() {
    assert_eq!(num("let a = json_parse(\"[1,2,3,4]\"); return sum(a);"), 10.0);
    assert!(truthy("return json_parse(\"{\\\"x\\\":true}\")[\"x\"];"));
    assert!(truthy("return json_stringify([1,2,3]) == \"[1,2,3]\";"));
    assert!(truthy(
        "let m = map(); m[\"a\"]=1; m[\"b\"]=2; let s = json_stringify(m); let m2 = json_parse(s); return (m2[\"a\"] + m2[\"b\"]) == 3;"
    ));
    assert_eq!(
        num("let d = json_parse(\"{\\\"items\\\":[10,20,30]}\"); return sum(d[\"items\"]);"),
        60.0
    );
}

#[test]
fn format_and_pad() {
    assert!(truthy("return format(\"{} + {} = {}\", [2, 3, 5]) == \"2 + 3 = 5\";"));
    assert!(truthy("return pad_left(\"7\", 3, \"0\") == \"007\";"));
    assert!(truthy("return pad_right(\"7\", 3, \".\") == \"7..\";"));
    assert_eq!(num("return count_sub(\"abababab\", \"ab\");"), 4.0);
}

#[test]
fn stdlib_gc_pressure() {
    assert!(truthy(
        "let total = 0; for i in 0..500 { let s = base64_encode(str(i)); total = total + len(s); } return total > 0;"
    ));
}
