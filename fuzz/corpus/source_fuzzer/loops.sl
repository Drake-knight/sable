let total = 0;
for i in 0..20 {
  if i % 2 == 0 {
    total += i;
  } else {
    total -= 1;
  }
}
let j = 0;
while j < 5 {
  total = total + j;
  j = j + 1;
}
return total;
