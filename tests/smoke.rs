use sable::vm::Vm;

fn num(src: &str) -> f64 {
    let mut vm = Vm::new();
    let v = vm.eval_str(src).expect("eval failed");
    assert!(v.is_number(), "expected a number result");
    v.as_number()
}

fn truthy(src: &str) -> bool {
    let mut vm = Vm::new();
    let v = vm.eval_str(src).expect("eval failed");
    v.is_truthy()
}

#[test]
fn arithmetic() {
    assert_eq!(num("return 1 + 2 * 3;"), 7.0);
    assert_eq!(num("return (1 + 2) * 3;"), 9.0);
    assert_eq!(num("return 17 % 5;"), 2.0);
    assert_eq!(num("return -8 / 2;"), -4.0);
}

#[test]
fn globals_and_assignment() {
    assert_eq!(num("let x = 10; x = x - 3; return x;"), 7.0);
    assert_eq!(num("let a = 1; let b = 2; return a + b;"), 3.0);
}

#[test]
fn control_flow() {
    assert_eq!(num("let i = 0; let s = 0; while i < 5 { s = s + i; i = i + 1; } return s;"), 10.0);
    assert_eq!(num("if 1 < 2 { return 100; } return 0;"), 100.0);
    assert_eq!(num("let i = 0; while true { i = i + 1; if i > 3 { break; } } return i;"), 4.0);
}

#[test]
fn functions_and_recursion() {
    assert_eq!(
        num("fn fib(n) { if n < 2 { return n; } return fib(n - 1) + fib(n - 2); } return fib(10);"),
        55.0
    );
    assert_eq!(num("fn add(a, b) { return a + b; } return add(3, 4);"), 7.0);
}

#[test]
fn closures_capture() {
    assert_eq!(
        num("fn make(n) { return fn() { return n; }; } let f = make(42); return f();"),
        42.0
    );
    assert_eq!(
        num("fn counter() { let c = 0; return fn() { c = c + 1; return c; }; } let c = counter(); c(); c(); return c();"),
        3.0
    );
}

#[test]
fn strings() {
    assert!(truthy("return (\"ab\" + \"cd\") == \"abcd\";"));
    assert!(truthy("let s = \"hi\"; return s == \"hi\";"));
}

#[test]
fn arrays() {
    assert_eq!(num("let a = [10, 20, 30]; return a[1];"), 20.0);
    assert_eq!(num("let a = [1, 2, 3]; a[0] = 99; return a[0];"), 99.0);
    assert_eq!(num("let a = [5]; return a[0] + a[0];"), 10.0);
}

#[test]
fn builtins_basic() {
    assert_eq!(num("let a = array(3); a[0]=1; a[1]=2; a[2]=3; return len(a);"), 3.0);
    assert_eq!(num("let a = array(0); push(a, 5); push(a, 7); return a[1];"), 7.0);
    assert_eq!(num("return abs(0 - 9);"), 9.0);
    assert_eq!(num("return floor(3.7);"), 3.0);
    assert_eq!(num("return sqrt(16);"), 4.0);
    assert_eq!(num("return len(\"hello\");"), 5.0);
}

#[test]
fn builtins_fill_and_join() {
    assert_eq!(num("let a = array(4); fill(a, 1, 2, 9); return a[2];"), 9.0);
    assert!(truthy(
        "let a = array(3); a[0]=\"a\"; a[1]=\"b\"; a[2]=\"c\"; return join(a) == \"abc\";"
    ));
}

#[test]
fn builtins_map() {
    assert_eq!(
        num("let m = map(); m[\"x\"] = 10; m[\"y\"] = 20; return m[\"x\"] + m[\"y\"];"),
        30.0
    );
    assert_eq!(num("let m = map(); m[\"k\"] = 1; return len(keys(m));"), 1.0);
}

#[test]
fn gc_stress() {
    assert_eq!(
        num("let i = 0; while i < 3000 { let s = \"abcdefgh\" + str(i); i = i + 1; } return i;"),
        3000.0
    );
    assert_eq!(
        num("fn f(n){ if n<1 { return 0; } let s = str(n) + str(n); return 1 + f(n-1); } return f(150);"),
        150.0
    );
}
