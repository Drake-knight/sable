let a = [5, 3, 8, 1, 9, 2];
sort(a);
let b = map();
for x in a {
  b[str(x)] = x * x;
}
let ks = keys(b);
let total = 0;
for k in ks {
  total = total + b[k];
}
let d = json_parse("{\"nums\":[1,2,3],\"name\":\"test\"}");
return total + sum(d["nums"]);
