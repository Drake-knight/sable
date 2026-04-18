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
fn for_range() {
    assert_eq!(num("let s=0; for i in 0..5 { s=s+i; } return s;"), 10.0);
    assert_eq!(num("let s=0; for i in 0..0 { s=s+1; } return s;"), 0.0);
    assert_eq!(num("let s=1; for i in 1..6 { s=s*i; } return s;"), 120.0);
}

#[test]
fn for_array() {
    assert_eq!(num("let a=[3,4,5]; let s=0; for x in a { s=s+x; } return s;"), 12.0);
    assert_eq!(
        num("let a=array(4); for i in 0..4 { a[i]=i*i; } let s=0; for x in a { s=s+x; } return s;"),
        14.0
    );
}

#[test]
fn for_nested_and_control() {
    assert_eq!(num("let s=0; for i in 0..3 { for j in 0..3 { s=s+1; } } return s;"), 9.0);
    assert_eq!(num("let s=0; for i in 0..10 { if i==5 { break; } s=s+1; } return s;"), 5.0);
    assert_eq!(num("let s=0; for i in 0..5 { if i==2 { continue; } s=s+i; } return s;"), 8.0);
}

#[test]
fn compound_assignment() {
    assert_eq!(num("let x=10; x+=5; return x;"), 15.0);
    assert_eq!(num("let x=10; x-=3; x*=2; return x;"), 14.0);
    assert_eq!(num("let x=20; x/=4; return x;"), 5.0);
    assert_eq!(num("let x=17; x%=5; return x;"), 2.0);
    assert_eq!(num("let a=[1,2]; a[1]+=40; return a[1];"), 42.0);
}

#[test]
fn stdlib_strings() {
    assert!(truthy("return upper(\"aB\") == \"AB\";"));
    assert!(truthy("return lower(\"aB\") == \"ab\";"));
    assert!(truthy("return trim(\"  hi \") == \"hi\";"));
    assert_eq!(num("return index_of(\"hello\", \"ll\");"), 2.0);
    assert!(truthy("return contains(\"hello\", \"ell\");"));
    assert!(truthy("return starts_with(\"hello\", \"he\");"));
    assert!(truthy("return ends_with(\"hello\", \"lo\");"));
    assert!(truthy("return replace(\"aXbXc\", \"X\", \"-\") == \"a-b-c\";"));
    assert!(truthy("return repeat(\"ab\", 3) == \"ababab\";"));
    assert!(truthy("return char_at(\"abc\", 1) == \"b\";"));
    assert_eq!(num("return code_at(\"A\", 0);"), 65.0);
    assert!(truthy("return from_code(66) == \"B\";"));
    assert_eq!(num("let p = split(\"a,b,c\", \",\"); return len(p);"), 3.0);
    assert!(truthy("let p = split(\"a,b,c\", \",\"); return p[1] == \"b\";"));
}

#[test]
fn stdlib_arrays() {
    assert_eq!(num("let a = slice([1,2,3,4], 1, 3); return a[0] + len(a);"), 4.0);
    assert_eq!(num("let a = [1,2,3]; reverse(a); return a[0];"), 3.0);
    assert_eq!(num("let a = [3,1,2]; sort(a); return a[0]*100 + a[2];"), 103.0);
    assert_eq!(num("return sum([1,2,3,4]);"), 10.0);
    assert_eq!(num("return pop([9,8,7]);"), 7.0);
    assert_eq!(num("let a=[1,2,3]; insert(a,1,99); return a[1]*10 + len(a);"), 994.0);
    assert_eq!(num("let a=[1,2,3]; remove(a,0); return a[0]*10 + len(a);"), 22.0);
    assert_eq!(num("let a=concat([1,2],[3,4]); return a[3] + len(a);"), 8.0);
    assert_eq!(num("return array_index_of([5,6,7], 6);"), 1.0);
    assert_eq!(num("return min_of([4,2,9,1]) + max_of([4,2,9,1]);"), 10.0);
}

#[test]
fn stdlib_math_and_types() {
    assert_eq!(num("return min(3,5) + max(3,5);"), 8.0);
    assert_eq!(num("return pow(2, 10);"), 1024.0);
    assert_eq!(num("return round(3.6) + ceil(3.1);"), 8.0);
    assert_eq!(num("return sign(0 - 5);"), -1.0);
    assert_eq!(num("return clamp(10, 0, 5);"), 5.0);
    assert!(truthy("return typeof(5) == \"number\";"));
    assert!(truthy("return typeof(\"a\") == \"string\";"));
    assert!(truthy("return typeof([1]) == \"array\";"));
    assert!(truthy("return typeof(map()) == \"map\";"));
    assert!(truthy("return is_number(5);"));
    assert!(truthy("return is_string(\"x\");"));
    assert_eq!(num("return num(\"42\");"), 42.0);
    assert_eq!(num("return int(3.9);"), 3.0);
}

#[test]
fn maps_iteration() {
    assert_eq!(
        num("let m = map(); for i in 0..5 { m[str(i)] = i * i; } let s = 0; let ks = keys(m); for k in ks { s = s + m[k]; } return s;"),
        30.0
    );
}
