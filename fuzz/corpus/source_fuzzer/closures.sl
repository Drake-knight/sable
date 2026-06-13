fn make_counter(start) {
  let n = start;
  return fn() {
    n = n + 1;
    return n;
  };
}
let c = make_counter(10);
c();
c();
fn fib(n) {
  if n < 2 { return n; }
  return fib(n - 1) + fib(n - 2);
}
return c() + fib(9);
