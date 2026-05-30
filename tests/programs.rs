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
fn fibonacci() {
    assert_eq!(
        num("fn fib(n){ if n<2 { return n; } return fib(n-1)+fib(n-2); } return fib(15);"),
        610.0
    );
}

#[test]
fn bubble_sort() {
    let prog = r#"
        let a = [5, 2, 8, 1, 9, 3];
        let n = len(a);
        for i in 0..n {
            for j in 0..(n - 1) {
                if a[j] > a[j + 1] {
                    let t = a[j];
                    a[j] = a[j + 1];
                    a[j + 1] = t;
                }
            }
        }
        return a[0] * 100 + a[5];
    "#;
    assert_eq!(num(prog), 109.0);
}

#[test]
fn string_reverse() {
    let prog = r#"
        fn reverse(s) {
            let out = "";
            let i = len(s) - 1;
            while i >= 0 {
                out = out + char_at(s, i);
                i = i - 1;
            }
            return out;
        }
        return reverse("hello") == "olleh";
    "#;
    assert!(truthy(prog));
}

#[test]
fn word_count() {
    let prog = r#"
        let text = "the quick brown fox the lazy dog the";
        let words = split(text, " ");
        let counts = map();
        for w in words {
            if has(counts, w) {
                counts[w] = counts[w] + 1;
            } else {
                counts[w] = 1;
            }
        }
        return counts["the"];
    "#;
    assert_eq!(num(prog), 3.0);
}

#[test]
fn closures_accounts() {
    let prog = r#"
        fn make_account(balance) {
            return fn(amount) {
                balance = balance + amount;
                return balance;
            };
        }
        let acct = make_account(100);
        acct(50);
        acct(0 - 30);
        return acct(0);
    "#;
    assert_eq!(num(prog), 120.0);
}

#[test]
fn json_processing() {
    let prog = r#"
        let data = json_parse("[{\"n\":1},{\"n\":2},{\"n\":3}]");
        let total = 0;
        for item in data {
            total = total + item["n"];
        }
        return total;
    "#;
    assert_eq!(num(prog), 6.0);
}

#[test]
fn quicksort_recursive() {
    let prog = r#"
        fn qsort(a) {
            if len(a) < 2 { return a; }
            let pivot = a[0];
            let less = [];
            let more = [];
            let i = 1;
            while i < len(a) {
                if a[i] < pivot {
                    push(less, a[i]);
                } else {
                    push(more, a[i]);
                }
                i = i + 1;
            }
            let left = qsort(less);
            push(left, pivot);
            return concat(left, qsort(more));
        }
        let sorted = qsort([3, 6, 1, 8, 2, 9, 4]);
        return sorted[0] * 10 + sorted[6];
    "#;
    assert_eq!(num(prog), 19.0);
}

#[test]
fn glob_matching() {
    assert!(truthy(r#"return glob("*.txt", "file.txt");"#));
    assert!(truthy(r#"return glob("a?c", "abc");"#));
    assert!(truthy(r#"return !glob("*.txt", "file.md");"#));
    assert!(truthy(r#"return glob_contains("[0-9]", "abc5def");"#));
}

#[test]
fn deterministic_random() {
    assert!(truthy(
        r#"srand(42); let a = rand_int(0, 1000000); srand(42); let b = rand_int(0, 1000000); return a == b;"#
    ));
}
