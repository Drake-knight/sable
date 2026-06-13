let s = upper("hello") + " " + lower("WORLD");
let parts = split("a,b,c,d", ",");
let joined = join(parts);
let t = trim("   spaced   ");
let r = replace("banana", "a", "o");
return len(s) + len(joined) + len(t) + len(r);
