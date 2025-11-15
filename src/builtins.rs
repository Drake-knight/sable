use crate::error::{SableError, SableResult};
use crate::object::{NativeFn, Obj, ObjArray, ObjKind, ObjMap, ObjStr};
use crate::value::Value;
use crate::vm::Vm;
use std::alloc::Layout;

pub fn install(vm: &mut Vm) {
    reg(vm, "print", 1, native_print);
    reg(vm, "len", 1, native_len);
    reg(vm, "array", 1, native_array);
    reg(vm, "push", 2, native_push);
    reg(vm, "fill", 4, native_fill);
    reg(vm, "map", 0, native_map);
    reg(vm, "keys", 1, native_keys);
    reg(vm, "str", 1, native_str);
    reg(vm, "substr", 3, native_substr);
    reg(vm, "join", 1, native_join);
    reg(vm, "abs", 1, native_abs);
    reg(vm, "floor", 1, native_floor);
    reg(vm, "sqrt", 1, native_sqrt);
    reg(vm, "upper", 1, native_upper);
    reg(vm, "lower", 1, native_lower);
    reg(vm, "trim", 1, native_trim);
    reg(vm, "index_of", 2, native_index_of);
    reg(vm, "contains", 2, native_contains);
    reg(vm, "starts_with", 2, native_starts_with);
    reg(vm, "ends_with", 2, native_ends_with);
    reg(vm, "replace", 3, native_replace);
    reg(vm, "repeat", 2, native_repeat);
    reg(vm, "char_at", 2, native_char_at);
    reg(vm, "code_at", 2, native_code_at);
    reg(vm, "from_code", 1, native_from_code);
    reg(vm, "split", 2, native_split);
    reg(vm, "pop", 1, native_pop);
    reg(vm, "slice", 3, native_slice);
    reg(vm, "reverse", 1, native_reverse);
    reg(vm, "array_contains", 2, native_array_contains);
    reg(vm, "array_index_of", 2, native_array_index_of);
    reg(vm, "sort", 1, native_sort);
    reg(vm, "sum", 1, native_sum);
    reg(vm, "min_of", 1, native_min_of);
    reg(vm, "max_of", 1, native_max_of);
    reg(vm, "insert", 3, native_insert);
    reg(vm, "remove", 2, native_remove);
    reg(vm, "concat", 2, native_concat);
    reg(vm, "min", 2, native_min);
    reg(vm, "max", 2, native_max);
    reg(vm, "pow", 2, native_pow);
    reg(vm, "round", 1, native_round);
    reg(vm, "ceil", 1, native_ceil);
    reg(vm, "sin", 1, native_sin);
    reg(vm, "cos", 1, native_cos);
    reg(vm, "tan", 1, native_tan);
    reg(vm, "log", 1, native_log);
    reg(vm, "exp", 1, native_exp);
    reg(vm, "sign", 1, native_sign);
    reg(vm, "clamp", 3, native_clamp);
    reg(vm, "typeof", 1, native_typeof);
    reg(vm, "is_number", 1, native_is_number);
    reg(vm, "is_string", 1, native_is_string);
    reg(vm, "is_array", 1, native_is_array);
    reg(vm, "is_map", 1, native_is_map);
    reg(vm, "is_function", 1, native_is_function);
    reg(vm, "is_nil", 1, native_is_nil);
    reg(vm, "num", 1, native_num);
    reg(vm, "int", 1, native_int);
    reg(vm, "bool", 1, native_bool);
    reg(vm, "has", 2, native_has);
    reg(vm, "values", 1, native_values);
    reg(vm, "size", 1, native_size);
    reg(vm, "band", 2, native_band);
    reg(vm, "bor", 2, native_bor);
    reg(vm, "bxor", 2, native_bxor);
    reg(vm, "bnot", 1, native_bnot);
    reg(vm, "shl", 2, native_shl);
    reg(vm, "shr", 2, native_shr);
    reg(vm, "gcd", 2, native_gcd);
    reg(vm, "hypot", 2, native_hypot);
    reg(vm, "atan2", 2, native_atan2);
    reg(vm, "log2", 1, native_log2);
    reg(vm, "hex_encode", 1, native_hex_encode);
    reg(vm, "hex_decode", 1, native_hex_decode);
    reg(vm, "base64_encode", 1, native_base64_encode);
    reg(vm, "base64_decode", 1, native_base64_decode);
    reg(vm, "json_parse", 1, native_json_parse);
    reg(vm, "json_stringify", 1, native_json_stringify);
    reg(vm, "format", 2, native_format);
    reg(vm, "pad_left", 3, native_pad_left);
    reg(vm, "pad_right", 3, native_pad_right);
    reg(vm, "count_sub", 2, native_count_sub);
    reg(vm, "srand", 1, native_srand);
    reg(vm, "rand", 0, native_rand);
    reg(vm, "rand_int", 2, native_rand_int);
    reg(vm, "glob", 2, native_glob);
    reg(vm, "glob_contains", 2, native_glob_contains);
    reg(vm, "is_alpha", 1, native_is_alpha);
    reg(vm, "is_digit", 1, native_is_digit);
    reg(vm, "is_space", 1, native_is_space);
    reg(vm, "title", 1, native_title);
    reg(vm, "find_last", 2, native_find_last);
    reg(vm, "lstrip", 1, native_lstrip);
    reg(vm, "rstrip", 1, native_rstrip);
    reg(vm, "take", 2, native_take);
    reg(vm, "drop", 2, native_drop);
    reg(vm, "count_value", 2, native_count_value);
    reg(vm, "unique", 1, native_unique);
    reg(vm, "flatten", 1, native_flatten);
    reg(vm, "range_arr", 2, native_range_arr);
    reg(vm, "get_or", 3, native_get_or);
    reg(vm, "merge", 2, native_merge);
    reg(vm, "crc32", 1, native_crc32);
    reg(vm, "adler32", 1, native_adler32);
    reg(vm, "fnv32", 1, native_fnv32);
    reg(vm, "fnv64", 1, native_fnv64);
    reg(vm, "djb2", 1, native_djb2);
    reg(vm, "csv_parse", 1, native_csv_parse);
    reg(vm, "csv_write", 1, native_csv_write);
    reg(vm, "sha256", 1, native_sha256);
    reg(vm, "mean", 1, native_mean);
    reg(vm, "median", 1, native_median);
    reg(vm, "variance", 1, native_variance);
    reg(vm, "stddev", 1, native_stddev);
    reg(vm, "product", 1, native_product);
    reg(vm, "capitalize", 1, native_capitalize);
    reg(vm, "center", 3, native_center);
    reg(vm, "zfill", 2, native_zfill);
    reg(vm, "swapcase", 1, native_swapcase);
    reg(vm, "chunk", 2, native_chunk);
    reg(vm, "zip", 2, native_zip);
    reg(vm, "to_pairs", 1, native_to_pairs);
    reg(vm, "from_pairs", 1, native_from_pairs);
    reg(vm, "url_encode", 1, native_url_encode);
    reg(vm, "url_decode", 1, native_url_decode);
    reg(vm, "url_query", 1, native_url_query);
    reg(vm, "first", 1, native_first);
    reg(vm, "last", 1, native_last);
    reg(vm, "lines", 1, native_lines);
    reg(vm, "ord", 1, native_ord);
    reg(vm, "chr", 1, native_chr);
    reg(vm, "is_nan", 1, native_is_nan);
    reg(vm, "is_inf", 1, native_is_inf);
    reg(vm, "rotate", 2, native_rotate);
    reg(vm, "deg", 1, native_deg);
    reg(vm, "rad", 1, native_rad);
    reg(vm, "lerp", 3, native_lerp);
    reg(vm, "reverse_str", 1, native_reverse_str);
    reg(vm, "base32_encode", 1, native_base32_encode);
    reg(vm, "base32_decode", 1, native_base32_decode);
    reg(vm, "percentile", 2, native_percentile);
    reg(vm, "correlation", 2, native_correlation);
    reg(vm, "linreg", 2, native_linreg);
    reg(vm, "word_count", 1, native_word_count);
    reg(vm, "is_palindrome", 1, native_is_palindrome);
    reg(vm, "edit_distance", 2, native_edit_distance);
    reg(vm, "wrap", 2, native_wrap);
    reg(vm, "indent", 2, native_indent);
}

fn native_word_count(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    Ok(Value::number(crate::textutil::count_words(str_bytes(s)) as f64))
}

fn native_is_palindrome(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    Ok(Value::bool(crate::textutil::is_palindrome(str_bytes(s))))
}

fn native_edit_distance(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_str(args[0])?;
    let b = want_str(args[1])?;
    Ok(Value::number(
        crate::textutil::levenshtein(str_bytes(a), str_bytes(b)) as f64,
    ))
}

fn native_wrap(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let w = want_num(args[1])? as usize;
    let out = crate::textutil::word_wrap(str_bytes(s), w);
    Ok(Value::from_obj(vm.new_str(out.as_bytes())))
}

fn native_indent(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let p = want_str(args[1])?;
    let text = String::from_utf8_lossy(str_bytes(s)).into_owned();
    let prefix = String::from_utf8_lossy(str_bytes(p)).into_owned();
    let out = crate::textutil::indent(&text, &prefix);
    Ok(Value::from_obj(vm.new_str(out.as_bytes())))
}

fn native_base32_encode(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let out = crate::base32::encode(str_bytes(s));
    Ok(Value::from_obj(vm.new_str(out.as_bytes())))
}

fn native_base32_decode(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    match crate::base32::decode(str_bytes(s)) {
        Some(bytes) => Ok(Value::from_obj(vm.new_str(&bytes))),
        None => Ok(Value::nil()),
    }
}

fn native_percentile(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let p = want_num(args[1])?;
    let v = collect_numbers(a);
    Ok(Value::number(crate::stats2::percentile(&v, p)))
}

fn native_correlation(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let b = want_array(args[1])?;
    let x = collect_numbers(a);
    let y = collect_numbers(b);
    Ok(Value::number(crate::stats2::correlation(&x, &y)))
}

fn native_linreg(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let b = want_array(args[1])?;
    let x = collect_numbers(a);
    let y = collect_numbers(b);
    let (slope, intercept) = crate::stats2::linear_regression(&x, &y);
    let arr = vm.new_array(2);
    unsafe {
        *(*arr).items.add(0) = Value::number(slope);
        *(*arr).items.add(1) = Value::number(intercept);
    }
    Ok(Value::from_obj(arr as *mut Obj))
}

fn native_url_encode(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let out = crate::url::encode(str_bytes(s));
    Ok(Value::from_obj(vm.new_str(out.as_bytes())))
}

fn native_url_decode(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let out = crate::url::decode(str_bytes(s));
    Ok(Value::from_obj(vm.new_str(&out)))
}

fn native_url_query(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let pairs = crate::url::parse_query(str_bytes(s));
    let outer = vm.new_array(pairs.len());
    vm.push_temp(outer as *mut Obj);
    for (i, kv) in pairs.iter().enumerate() {
        let pair = vm.new_array(2);
        unsafe {
            *(*outer).items.add(i) = Value::from_obj(pair as *mut Obj);
        }
        let ko = vm.new_str(kv.0.as_bytes());
        unsafe {
            *(*pair).items.add(0) = Value::from_obj(ko);
        }
        let vo = vm.new_str(kv.1.as_bytes());
        unsafe {
            *(*pair).items.add(1) = Value::from_obj(vo);
        }
    }
    vm.pop_temp();
    Ok(Value::from_obj(outer as *mut Obj))
}

fn native_first(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    unsafe {
        if (*a).len == 0 {
            Ok(Value::nil())
        } else {
            Ok(*(*a).items)
        }
    }
}

fn native_last(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    unsafe {
        if (*a).len == 0 {
            Ok(Value::nil())
        } else {
            Ok(*(*a).items.add((*a).len - 1))
        }
    }
}

fn native_lines(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let src = str_bytes(s).to_vec();
    let mut parts: Vec<(usize, usize)> = Vec::new();
    let mut start = 0;
    let mut i = 0;
    while i < src.len() {
        if src[i] == b'\n' {
            let mut end = i;
            if end > start && src[end - 1] == b'\r' {
                end -= 1;
            }
            parts.push((start, end));
            start = i + 1;
        }
        i += 1;
    }
    if start < src.len() {
        let mut end = src.len();
        if end > start && src[end - 1] == b'\r' {
            end -= 1;
        }
        parts.push((start, end));
    }
    let arr = vm.new_array(parts.len());
    vm.push_temp(arr as *mut Obj);
    for (idx, seg) in parts.iter().enumerate() {
        let so = vm.new_str(&src[seg.0..seg.1]);
        unsafe {
            *(*arr).items.add(idx) = Value::from_obj(so);
        }
    }
    vm.pop_temp();
    Ok(Value::from_obj(arr as *mut Obj))
}

fn native_ord(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let src = str_bytes(s);
    if src.is_empty() {
        Ok(Value::number(-1.0))
    } else {
        Ok(Value::number(src[0] as f64))
    }
}

fn native_chr(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let n = want_num(args[0])? as i64;
    let b = (n & 0xff) as u8;
    Ok(Value::from_obj(vm.new_str(&[b])))
}

fn native_is_nan(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let v = args[0];
    Ok(Value::bool(v.is_number() && v.as_number().is_nan()))
}

fn native_is_inf(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let v = args[0];
    Ok(Value::bool(v.is_number() && v.as_number().is_infinite()))
}

fn native_rotate(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let len = unsafe { (*a).len };
    if len == 0 {
        return Ok(Value::nil());
    }
    let raw = want_num(args[1])? as i64;
    let n = (((raw % len as i64) + len as i64) as usize) % len;
    if n == 0 {
        return Ok(Value::nil());
    }
    let mut tmp: Vec<Value> = Vec::with_capacity(len);
    unsafe {
        for i in 0..len {
            tmp.push(*(*a).items.add(i));
        }
        for i in 0..len {
            *(*a).items.add(i) = tmp[(i + n) % len];
        }
    }
    Ok(Value::nil())
}

fn native_deg(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::number(want_num(args[0])? * 180.0 / std::f64::consts::PI))
}

fn native_rad(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::number(want_num(args[0])? * std::f64::consts::PI / 180.0))
}

fn native_lerp(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_num(args[0])?;
    let b = want_num(args[1])?;
    let t = want_num(args[2])?;
    Ok(Value::number(a + (b - a) * t))
}

fn native_reverse_str(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let src = str_bytes(s);
    let mut out = Vec::with_capacity(src.len());
    for i in (0..src.len()).rev() {
        out.push(src[i]);
    }
    Ok(Value::from_obj(vm.new_str(&out)))
}

fn native_sha256(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let hex = crate::sha256::sha256_hex(str_bytes(s));
    Ok(Value::from_obj(vm.new_str(hex.as_bytes())))
}

fn native_mean(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let len = unsafe { (*a).len };
    let mut sum = 0.0;
    let mut cnt = 0;
    for i in 0..len {
        let e = unsafe { *(*a).items.add(i) };
        if e.is_number() {
            sum += e.as_number();
            cnt += 1;
        }
    }
    if cnt == 0 {
        return Ok(Value::nil());
    }
    Ok(Value::number(sum / cnt as f64))
}

fn collect_numbers(a: *mut ObjArray) -> Vec<f64> {
    let len = unsafe { (*a).len };
    let mut v = Vec::new();
    for i in 0..len {
        let e = unsafe { *(*a).items.add(i) };
        if e.is_number() {
            v.push(e.as_number());
        }
    }
    v
}

fn native_median(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let mut v = collect_numbers(a);
    if v.is_empty() {
        return Ok(Value::nil());
    }
    v.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    let m = v.len();
    let med = if m % 2 == 1 {
        v[m / 2]
    } else {
        (v[m / 2 - 1] + v[m / 2]) / 2.0
    };
    Ok(Value::number(med))
}

fn native_variance(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let v = collect_numbers(a);
    if v.is_empty() {
        return Ok(Value::nil());
    }
    let mean = v.iter().sum::<f64>() / v.len() as f64;
    let var = v.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / v.len() as f64;
    Ok(Value::number(var))
}

fn native_stddev(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let var = native_variance(vm, args)?;
    if var.is_number() {
        Ok(Value::number(var.as_number().sqrt()))
    } else {
        Ok(var)
    }
}

fn native_product(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let len = unsafe { (*a).len };
    let mut prod = 1.0;
    for i in 0..len {
        let e = unsafe { *(*a).items.add(i) };
        if e.is_number() {
            prod *= e.as_number();
        }
    }
    Ok(Value::number(prod))
}

fn native_capitalize(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let src = str_bytes(s);
    let mut out = Vec::with_capacity(src.len());
    for (i, &b) in src.iter().enumerate() {
        if i == 0 {
            out.push(if b >= b'a' && b <= b'z' { b - 32 } else { b });
        } else {
            out.push(if b >= b'A' && b <= b'Z' { b + 32 } else { b });
        }
    }
    Ok(Value::from_obj(vm.new_str(&out)))
}

fn native_center(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let w = want_num(args[1])? as usize;
    let pad = want_str(args[2])?;
    let src = str_bytes(s).to_vec();
    let padb = str_bytes(pad).to_vec();
    let padc = if padb.is_empty() { b' ' } else { padb[0] };
    if src.len() >= w {
        return Ok(Value::from_obj(vm.new_str(&src)));
    }
    let total = w - src.len();
    let left = total / 2;
    let right = total - left;
    let mut out = Vec::new();
    for _ in 0..left {
        out.push(padc);
    }
    out.extend_from_slice(&src);
    for _ in 0..right {
        out.push(padc);
    }
    Ok(Value::from_obj(vm.new_str(&out)))
}

fn native_zfill(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let w = want_num(args[1])? as usize;
    let src = str_bytes(s).to_vec();
    let mut out = Vec::new();
    if src.len() < w {
        for _ in 0..(w - src.len()) {
            out.push(b'0');
        }
    }
    out.extend_from_slice(&src);
    Ok(Value::from_obj(vm.new_str(&out)))
}

fn native_swapcase(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let src = str_bytes(s);
    let mut out = Vec::with_capacity(src.len());
    for &b in src {
        if b >= b'a' && b <= b'z' {
            out.push(b - 32);
        } else if b >= b'A' && b <= b'Z' {
            out.push(b + 32);
        } else {
            out.push(b);
        }
    }
    Ok(Value::from_obj(vm.new_str(&out)))
}

fn native_chunk(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let n = want_num(args[1])? as usize;
    if n == 0 {
        return Err(SableError::Runtime("chunk size must be positive".to_string()));
    }
    let len = unsafe { (*a).len };
    let nchunks = (len + n - 1) / n;
    let outer = vm.new_array(nchunks);
    vm.push_temp(outer as *mut Obj);
    let mut idx = 0;
    let mut ci = 0;
    while idx < len {
        let this = if len - idx < n { len - idx } else { n };
        let inner = vm.new_array(this);
        unsafe {
            *(*outer).items.add(ci) = Value::from_obj(inner as *mut Obj);
            for j in 0..this {
                *(*inner).items.add(j) = *(*a).items.add(idx + j);
            }
        }
        idx += this;
        ci += 1;
    }
    vm.pop_temp();
    Ok(Value::from_obj(outer as *mut Obj))
}

fn native_zip(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let b = want_array(args[1])?;
    let la = unsafe { (*a).len };
    let lb = unsafe { (*b).len };
    let n = if la < lb { la } else { lb };
    let outer = vm.new_array(n);
    vm.push_temp(outer as *mut Obj);
    for i in 0..n {
        let pair = vm.new_array(2);
        unsafe {
            *(*outer).items.add(i) = Value::from_obj(pair as *mut Obj);
            *(*pair).items.add(0) = *(*a).items.add(i);
            *(*pair).items.add(1) = *(*b).items.add(i);
        }
    }
    vm.pop_temp();
    Ok(Value::from_obj(outer as *mut Obj))
}

fn native_to_pairs(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let m = want_map(args[0])?;
    let mut pairs: Vec<(Value, Value)> = Vec::new();
    unsafe {
        let t = &(*m).table;
        for i in 0..t.cap {
            let e = t.entries.add(i);
            if (*e).used {
                pairs.push(((*e).key, (*e).value));
            }
        }
    }
    let outer = vm.new_array(pairs.len());
    vm.push_temp(outer as *mut Obj);
    for (i, kv) in pairs.iter().enumerate() {
        let pair = vm.new_array(2);
        unsafe {
            *(*outer).items.add(i) = Value::from_obj(pair as *mut Obj);
            *(*pair).items.add(0) = kv.0;
            *(*pair).items.add(1) = kv.1;
        }
    }
    vm.pop_temp();
    Ok(Value::from_obj(outer as *mut Obj))
}

fn native_from_pairs(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let len = unsafe { (*a).len };
    let m = vm.new_map();
    vm.push_temp(m as *mut Obj);
    for i in 0..len {
        let e = unsafe { *(*a).items.add(i) };
        if is_kind(e, ObjKind::Array) {
            let inner = e.as_obj() as *mut ObjArray;
            if unsafe { (*inner).len } >= 2 {
                let k = unsafe { *(*inner).items.add(0) };
                let v = unsafe { *(*inner).items.add(1) };
                unsafe {
                    let t = &mut (*m).table;
                    t.set(k, v);
                }
            }
        }
    }
    vm.pop_temp();
    Ok(Value::from_obj(m as *mut Obj))
}

fn native_csv_parse(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let rows = crate::csv::parse(str_bytes(s));
    let outer = vm.new_array(rows.len());
    vm.push_temp(outer as *mut Obj);
    for (i, row) in rows.iter().enumerate() {
        let inner = vm.new_array(row.len());
        unsafe {
            *(*outer).items.add(i) = Value::from_obj(inner as *mut Obj);
        }
        for (j, field) in row.iter().enumerate() {
            let so = vm.new_str(field.as_bytes());
            unsafe {
                *(*inner).items.add(j) = Value::from_obj(so);
            }
        }
    }
    vm.pop_temp();
    Ok(Value::from_obj(outer as *mut Obj))
}

fn native_csv_write(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let outer = want_array(args[0])?;
    let olen = unsafe { (*outer).len };
    let mut rows: Vec<Vec<String>> = Vec::new();
    unsafe {
        for i in 0..olen {
            let rv = *(*outer).items.add(i);
            let mut fields: Vec<String> = Vec::new();
            if is_kind(rv, ObjKind::Array) {
                let inner = rv.as_obj() as *mut ObjArray;
                let ilen = (*inner).len;
                for j in 0..ilen {
                    let fv = *(*inner).items.add(j);
                    fields.push(format_value(fv, 2));
                }
            }
            rows.push(fields);
        }
    }
    let text = crate::csv::write(&rows);
    Ok(Value::from_obj(vm.new_str(text.as_bytes())))
}

fn native_crc32(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    Ok(Value::number(crate::hashing::crc32(str_bytes(s)) as f64))
}

fn native_adler32(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    Ok(Value::number(crate::hashing::adler32(str_bytes(s)) as f64))
}

fn native_fnv32(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    Ok(Value::number(crate::hashing::fnv1a_32(str_bytes(s)) as f64))
}

fn native_fnv64(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let h = crate::hashing::fnv1a_64(str_bytes(s));
    Ok(Value::number((h & 0xffff_ffff) as f64))
}

fn native_djb2(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let h = crate::hashing::djb2(str_bytes(s));
    Ok(Value::number((h & 0xffff_ffff) as f64))
}

fn native_is_alpha(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let src = str_bytes(s);
    if src.is_empty() {
        return Ok(Value::bool(false));
    }
    for &b in src {
        if !((b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z')) {
            return Ok(Value::bool(false));
        }
    }
    Ok(Value::bool(true))
}

fn native_is_digit(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let src = str_bytes(s);
    if src.is_empty() {
        return Ok(Value::bool(false));
    }
    for &b in src {
        if b < b'0' || b > b'9' {
            return Ok(Value::bool(false));
        }
    }
    Ok(Value::bool(true))
}

fn native_is_space(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let src = str_bytes(s);
    if src.is_empty() {
        return Ok(Value::bool(false));
    }
    for &b in src {
        if !is_ws(b) {
            return Ok(Value::bool(false));
        }
    }
    Ok(Value::bool(true))
}

fn native_title(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let src = str_bytes(s);
    let mut out = Vec::with_capacity(src.len());
    let mut at_start = true;
    for &b in src {
        let alpha = (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z');
        if alpha {
            if at_start && b >= b'a' && b <= b'z' {
                out.push(b - 32);
            } else if !at_start && b >= b'A' && b <= b'Z' {
                out.push(b + 32);
            } else {
                out.push(b);
            }
            at_start = false;
        } else {
            out.push(b);
            at_start = true;
        }
    }
    Ok(Value::from_obj(vm.new_str(&out)))
}

fn native_find_last(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let sub = want_str(args[1])?;
    let hay = str_bytes(s);
    let needle = str_bytes(sub);
    if needle.is_empty() || needle.len() > hay.len() {
        return Ok(Value::number(-1.0));
    }
    let mut last: i64 = -1;
    let mut i = 0;
    while i + needle.len() <= hay.len() {
        if &hay[i..i + needle.len()] == needle {
            last = i as i64;
        }
        i += 1;
    }
    Ok(Value::number(last as f64))
}

fn native_lstrip(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let src = str_bytes(s);
    let mut start = 0;
    while start < src.len() && is_ws(src[start]) {
        start += 1;
    }
    let out = src[start..].to_vec();
    Ok(Value::from_obj(vm.new_str(&out)))
}

fn native_rstrip(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let src = str_bytes(s);
    let mut end = src.len();
    while end > 0 && is_ws(src[end - 1]) {
        end -= 1;
    }
    let out = src[..end].to_vec();
    Ok(Value::from_obj(vm.new_str(&out)))
}

fn native_take(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let len = unsafe { (*a).len };
    let n = clamp_idx(want_num(args[1])?, len);
    let arr = vm.new_array(n);
    unsafe {
        for i in 0..n {
            *(*arr).items.add(i) = *(*a).items.add(i);
        }
    }
    Ok(Value::from_obj(arr as *mut Obj))
}

fn native_drop(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let len = unsafe { (*a).len };
    let start = clamp_idx(want_num(args[1])?, len);
    let n = len - start;
    let arr = vm.new_array(n);
    unsafe {
        for i in 0..n {
            *(*arr).items.add(i) = *(*a).items.add(start + i);
        }
    }
    Ok(Value::from_obj(arr as *mut Obj))
}

fn native_count_value(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let target = args[1];
    let len = unsafe { (*a).len };
    let mut c = 0;
    for i in 0..len {
        let e = unsafe { *(*a).items.add(i) };
        if val_eq(e, target) {
            c += 1;
        }
    }
    Ok(Value::number(c as f64))
}

fn native_unique(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let len = unsafe { (*a).len };
    let mut result: Vec<Value> = Vec::new();
    for i in 0..len {
        let e = unsafe { *(*a).items.add(i) };
        let mut found = false;
        for r in &result {
            if val_eq(*r, e) {
                found = true;
                break;
            }
        }
        if !found {
            result.push(e);
        }
    }
    let arr = vm.new_array(result.len());
    unsafe {
        for (i, v) in result.iter().enumerate() {
            *(*arr).items.add(i) = *v;
        }
    }
    Ok(Value::from_obj(arr as *mut Obj))
}

fn native_flatten(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let len = unsafe { (*a).len };
    let mut result: Vec<Value> = Vec::new();
    for i in 0..len {
        let e = unsafe { *(*a).items.add(i) };
        if is_kind(e, ObjKind::Array) {
            let inner = e.as_obj() as *mut ObjArray;
            let ilen = unsafe { (*inner).len };
            for j in 0..ilen {
                result.push(unsafe { *(*inner).items.add(j) });
            }
        } else {
            result.push(e);
        }
    }
    let arr = vm.new_array(result.len());
    unsafe {
        for (i, v) in result.iter().enumerate() {
            *(*arr).items.add(i) = *v;
        }
    }
    Ok(Value::from_obj(arr as *mut Obj))
}

fn native_range_arr(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let lo = want_num(args[0])? as i64;
    let hi = want_num(args[1])? as i64;
    let n = if hi > lo { (hi - lo) as usize } else { 0 };
    if n > 10_000_000 {
        return Err(SableError::Runtime("range too large".to_string()));
    }
    let arr = vm.new_array(n);
    unsafe {
        for i in 0..n {
            *(*arr).items.add(i) = Value::number((lo + i as i64) as f64);
        }
    }
    Ok(Value::from_obj(arr as *mut Obj))
}

fn native_get_or(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let m = want_map(args[0])?;
    let t = unsafe { &(*m).table };
    if t.has(args[1]) {
        Ok(t.get(args[1]))
    } else {
        Ok(args[2])
    }
}

fn native_merge(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let ma = want_map(args[0])?;
    let mb = want_map(args[1])?;
    let res = vm.new_map();
    vm.push_temp(res as *mut Obj);
    unsafe {
        let ta = &(*ma).table;
        for i in 0..ta.cap {
            let e = ta.entries.add(i);
            if (*e).used {
                let t = &mut (*res).table;
                t.set((*e).key, (*e).value);
            }
        }
        let tb = &(*mb).table;
        for i in 0..tb.cap {
            let e = tb.entries.add(i);
            if (*e).used {
                let t = &mut (*res).table;
                t.set((*e).key, (*e).value);
            }
        }
    }
    vm.pop_temp();
    Ok(Value::from_obj(res as *mut Obj))
}

fn native_glob(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let pat = want_str(args[0])?;
    let text = want_str(args[1])?;
    Ok(Value::bool(crate::pattern::is_match(
        str_bytes(pat),
        str_bytes(text),
    )))
}

fn native_glob_contains(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let pat = want_str(args[0])?;
    let text = want_str(args[1])?;
    Ok(Value::bool(crate::pattern::contains_match(
        str_bytes(pat),
        str_bytes(text),
    )))
}

fn native_srand(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_num(args[0])? as i64 as u64;
    vm.rng = if s == 0 { 1 } else { s };
    Ok(Value::nil())
}

fn native_rand(vm: &mut Vm, _args: &[Value]) -> SableResult<Value> {
    vm.rng = crate::random::step(vm.rng);
    Ok(Value::number(crate::random::to_unit(vm.rng)))
}

fn native_rand_int(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let lo = want_num(args[0])? as i64;
    let hi = want_num(args[1])? as i64;
    if hi <= lo {
        return Ok(Value::number(lo as f64));
    }
    vm.rng = crate::random::step(vm.rng);
    let span = (hi - lo) as u64;
    let r = lo + crate::random::bounded(vm.rng, span) as i64;
    Ok(Value::number(r as f64))
}

fn json_to_value(vm: &mut Vm, j: &crate::json::Json) -> Value {
    match j {
        crate::json::Json::Null => Value::nil(),
        crate::json::Json::Bool(b) => Value::bool(*b),
        crate::json::Json::Num(n) => Value::number(*n),
        crate::json::Json::Str(s) => Value::from_obj(vm.new_str(s.as_bytes())),
        crate::json::Json::Array(items) => {
            let arr = vm.new_array(items.len());
            vm.push_temp(arr as *mut Obj);
            for (i, it) in items.iter().enumerate() {
                let v = json_to_value(vm, it);
                unsafe {
                    *(*arr).items.add(i) = v;
                }
            }
            vm.pop_temp();
            Value::from_obj(arr as *mut Obj)
        }
        crate::json::Json::Object(entries) => {
            let m = vm.new_map();
            vm.push_temp(m as *mut Obj);
            for (k, val) in entries {
                let key = Value::from_obj(vm.new_str(k.as_bytes()));
                vm.push_temp(key.as_obj());
                let v = json_to_value(vm, val);
                unsafe {
                    let t = &mut (*m).table;
                    t.set(key, v);
                }
                vm.pop_temp();
            }
            vm.pop_temp();
            Value::from_obj(m as *mut Obj)
        }
    }
}

fn value_to_json(v: Value, depth: i32) -> crate::json::Json {
    if depth <= 0 {
        return crate::json::Json::Null;
    }
    if v.is_nil() {
        return crate::json::Json::Null;
    }
    if v.is_bool() {
        return crate::json::Json::Bool(v.as_bool());
    }
    if v.is_number() {
        return crate::json::Json::Num(v.as_number());
    }
    match unsafe { (*v.as_obj()).kind } {
        ObjKind::Str => {
            let s = v.as_obj() as *mut ObjStr;
            crate::json::Json::Str(String::from_utf8_lossy(str_bytes(s)).into_owned())
        }
        ObjKind::Array => {
            let a = v.as_obj() as *mut ObjArray;
            let len = unsafe { (*a).len };
            let mut items = Vec::with_capacity(len);
            for i in 0..len {
                let e = unsafe { *(*a).items.add(i) };
                items.push(value_to_json(e, depth - 1));
            }
            crate::json::Json::Array(items)
        }
        ObjKind::Map => {
            let m = v.as_obj() as *mut ObjMap;
            let mut entries = Vec::new();
            unsafe {
                let t = &(*m).table;
                for i in 0..t.cap {
                    let e = t.entries.add(i);
                    if !(*e).used {
                        continue;
                    }
                    let key = (*e).key;
                    let keystr = if is_kind(key, ObjKind::Str) {
                        String::from_utf8_lossy(str_bytes(key.as_obj() as *mut ObjStr)).into_owned()
                    } else if key.is_number() {
                        let n = key.as_number();
                        if n.is_finite() && n == n.trunc() {
                            format!("{}", n as i64)
                        } else {
                            format!("{}", n)
                        }
                    } else {
                        continue;
                    };
                    entries.push((keystr, value_to_json((*e).value, depth - 1)));
                }
            }
            crate::json::Json::Object(entries)
        }
        _ => crate::json::Json::Null,
    }
}

fn clamp_idx(f: f64, len: usize) -> usize {
    if f <= 0.0 {
        0
    } else if f as usize >= len {
        len
    } else {
        f as usize
    }
}

fn val_eq(a: Value, b: Value) -> bool {
    if a.is_number() && b.is_number() {
        return a.as_number() == b.as_number();
    }
    if a.is_object() && b.is_object() {
        return crate::table::key_eq(a, b);
    }
    a.bits() == b.bits()
}

fn is_kind(v: Value, k: ObjKind) -> bool {
    v.is_object() && unsafe { (*v.as_obj()).kind == k }
}

unsafe fn array_ensure_cap(a: *mut ObjArray, needed: usize) {
    if (*a).cap >= needed {
        return;
    }
    let mut newcap = if (*a).cap < 8 { 8 } else { (*a).cap * 2 };
    while newcap < needed {
        newcap *= 2;
    }
    let new_items = std::alloc::alloc(Layout::array::<Value>(newcap).unwrap()) as *mut Value;
    for i in 0..(*a).len {
        *new_items.add(i) = *(*a).items.add(i);
    }
    for i in (*a).len..newcap {
        *new_items.add(i) = Value::nil();
    }
    std::alloc::dealloc(
        (*a).items as *mut u8,
        Layout::array::<Value>((*a).cap).unwrap(),
    );
    (*a).items = new_items;
    (*a).cap = newcap;
}

fn str_bytes<'a>(s: *mut ObjStr) -> &'a [u8] {
    unsafe { std::slice::from_raw_parts((*s).data, (*s).len) }
}

fn is_ws(b: u8) -> bool {
    b == b' ' || b == b'\t' || b == b'\n' || b == b'\r'
}

fn find_sub(hay: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    if needle.len() > hay.len() {
        return None;
    }
    let mut i = 0;
    while i + needle.len() <= hay.len() {
        if &hay[i..i + needle.len()] == needle {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn reg(vm: &mut Vm, name: &str, arity: i32, func: NativeFn) {
    let name_obj = vm.new_str(name.as_bytes());
    vm.push_temp(name_obj);
    let nat = vm.new_native(arity, func, name_obj);
    vm.pop_temp();
    vm.define_global(name, Value::from_obj(nat));
}

fn want_array(v: Value) -> SableResult<*mut ObjArray> {
    if v.is_object() && unsafe { (*v.as_obj()).kind } == ObjKind::Array {
        Ok(v.as_obj() as *mut ObjArray)
    } else {
        Err(SableError::Type("expected an array".to_string()))
    }
}

fn want_str(v: Value) -> SableResult<*mut ObjStr> {
    if v.is_object() && unsafe { (*v.as_obj()).kind } == ObjKind::Str {
        Ok(v.as_obj() as *mut ObjStr)
    } else {
        Err(SableError::Type("expected a string".to_string()))
    }
}

fn want_map(v: Value) -> SableResult<*mut ObjMap> {
    if v.is_object() && unsafe { (*v.as_obj()).kind } == ObjKind::Map {
        Ok(v.as_obj() as *mut ObjMap)
    } else {
        Err(SableError::Type("expected a map".to_string()))
    }
}

fn want_num(v: Value) -> SableResult<f64> {
    if v.is_number() {
        Ok(v.as_number())
    } else {
        Err(SableError::Type("expected a number".to_string()))
    }
}

fn format_value(v: Value, depth: i32) -> String {
    if v.is_nil() {
        return "nil".to_string();
    }
    if v.is_bool() {
        return if v.as_bool() { "true" } else { "false" }.to_string();
    }
    if v.is_number() {
        let n = v.as_number();
        if n.is_finite() && n == n.trunc() {
            return format!("{}", n as i64);
        }
        return format!("{}", n);
    }
    let o = v.as_obj();
    match unsafe { (*o).kind } {
        ObjKind::Str => {
            let s = o as *mut ObjStr;
            let bytes = unsafe { std::slice::from_raw_parts((*s).data, (*s).len) };
            String::from_utf8_lossy(bytes).into_owned()
        }
        ObjKind::Array => {
            if depth <= 0 {
                return "[...]".to_string();
            }
            let a = o as *mut ObjArray;
            let len = unsafe { (*a).len };
            let mut out = String::from("[");
            for i in 0..len {
                if i > 0 {
                    out.push_str(", ");
                }
                let e = unsafe { *(*a).items.add(i) };
                out.push_str(&format_value(e, depth - 1));
            }
            out.push(']');
            out
        }
        ObjKind::Map => "{map}".to_string(),
        ObjKind::Closure | ObjKind::Proto => "{function}".to_string(),
        ObjKind::Native => "{native}".to_string(),
        ObjKind::Upvalue => "{upvalue}".to_string(),
    }
}

fn native_print(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = format_value(args[0], 6);
    println!("{}", s);
    Ok(Value::nil())
}

fn native_len(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let v = args[0];
    let n = if v.is_object() {
        match unsafe { (*v.as_obj()).kind } {
            ObjKind::Str => unsafe { (*(v.as_obj() as *mut ObjStr)).len },
            ObjKind::Array => unsafe { (*(v.as_obj() as *mut ObjArray)).len },
            ObjKind::Map => unsafe { (*(v.as_obj() as *mut ObjMap)).table.count },
            _ => 0,
        }
    } else {
        0
    };
    Ok(Value::number(n as f64))
}

fn native_array(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let n = want_num(args[0])?;
    if n < 0.0 || n > 1.0e7 {
        return Err(SableError::Runtime("invalid array size".to_string()));
    }
    let a = vm.new_array(n as usize);
    Ok(Value::from_obj(a as *mut Obj))
}

fn native_push(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let v = args[1];
    unsafe {
        if (*a).len == (*a).cap {
            let new_cap = if (*a).cap < 8 { 8 } else { (*a).cap * 2 };
            let new_items =
                std::alloc::alloc(std::alloc::Layout::array::<Value>(new_cap).unwrap())
                    as *mut Value;
            for i in 0..(*a).len {
                *new_items.add(i) = *(*a).items.add(i);
            }
            for i in (*a).len..new_cap {
                *new_items.add(i) = Value::nil();
            }
            std::alloc::dealloc(
                (*a).items as *mut u8,
                std::alloc::Layout::array::<Value>((*a).cap).unwrap(),
            );
            (*a).items = new_items;
            (*a).cap = new_cap;
        }
        *(*a).items.add((*a).len) = v;
        (*a).len += 1;
    }
    Ok(Value::nil())
}

fn native_fill(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let start = want_num(args[1])? as usize;
    let count = want_num(args[2])? as usize;
    let value = args[3];
    let len = unsafe { (*a).len };
    if start <= len && count <= len {
        let items = unsafe { (*a).items };
        for i in 0..count {
            unsafe {
                *items.add(start + i) = value;
            }
        }
    }
    Ok(Value::nil())
}

fn native_map(vm: &mut Vm, _args: &[Value]) -> SableResult<Value> {
    let m = vm.new_map();
    Ok(Value::from_obj(m as *mut Obj))
}

fn native_keys(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let m = want_map(args[0])?;
    let mut ks: Vec<Value> = Vec::new();
    unsafe {
        let t = &(*m).table;
        for i in 0..t.cap {
            let e = t.entries.add(i);
            if (*e).used {
                ks.push((*e).key);
            }
        }
    }
    let arr = vm.new_array(ks.len());
    unsafe {
        for (i, k) in ks.iter().enumerate() {
            *(*arr).items.add(i) = *k;
        }
    }
    Ok(Value::from_obj(arr as *mut Obj))
}

fn native_str(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = format_value(args[0], 4);
    let obj = vm.new_str(s.as_bytes());
    Ok(Value::from_obj(obj))
}

fn native_substr(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let start = want_num(args[1])? as usize;
    let count = want_num(args[2])? as usize;
    let slen = unsafe { (*s).len };
    let st = if start > slen { slen } else { start };
    let en = if count > slen - st { slen } else { st + count };
    let bytes = unsafe { std::slice::from_raw_parts((*s).data.add(st), en - st) };
    let obj = vm.new_str(bytes);
    Ok(Value::from_obj(obj))
}

fn native_join(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let len = unsafe { (*a).len };
    let mut acc = vm.new_str_uninit(0);
    unsafe {
        (*acc).len = 0;
    }
    let mut i = 0;
    while i < len {
        let elem = unsafe { *(*a).items.add(i) };
        if elem.is_object() && unsafe { (*elem.as_obj()).kind } == ObjKind::Str {
            let es = elem.as_obj() as *mut ObjStr;
            let alen = unsafe { (*acc).len };
            let elen = unsafe { (*es).len };
            let joined = vm.new_str_uninit(alen + elen);
            unsafe {
                std::ptr::copy_nonoverlapping((*acc).data, (*joined).data, alen);
                std::ptr::copy_nonoverlapping((*es).data, (*joined).data.add(alen), elen);
            }
            acc = joined;
        }
        i += 1;
    }
    Ok(Value::from_obj(acc as *mut Obj))
}

fn native_abs(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::number(want_num(args[0])?.abs()))
}

fn native_floor(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::number(want_num(args[0])?.floor()))
}

fn native_sqrt(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::number(want_num(args[0])?.sqrt()))
}

fn native_upper(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let src = str_bytes(s);
    let mut out = Vec::with_capacity(src.len());
    for &b in src {
        out.push(if b >= b'a' && b <= b'z' { b - 32 } else { b });
    }
    Ok(Value::from_obj(vm.new_str(&out)))
}

fn native_lower(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let src = str_bytes(s);
    let mut out = Vec::with_capacity(src.len());
    for &b in src {
        out.push(if b >= b'A' && b <= b'Z' { b + 32 } else { b });
    }
    Ok(Value::from_obj(vm.new_str(&out)))
}

fn native_trim(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let src = str_bytes(s);
    let mut start = 0;
    let mut end = src.len();
    while start < end && is_ws(src[start]) {
        start += 1;
    }
    while end > start && is_ws(src[end - 1]) {
        end -= 1;
    }
    let out = src[start..end].to_vec();
    Ok(Value::from_obj(vm.new_str(&out)))
}

fn native_index_of(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let sub = want_str(args[1])?;
    let idx = find_sub(str_bytes(s), str_bytes(sub));
    Ok(Value::number(match idx {
        Some(i) => i as f64,
        None => -1.0,
    }))
}

fn native_contains(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let sub = want_str(args[1])?;
    Ok(Value::bool(find_sub(str_bytes(s), str_bytes(sub)).is_some()))
}

fn native_starts_with(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let pre = want_str(args[1])?;
    let hay = str_bytes(s);
    let p = str_bytes(pre);
    let r = hay.len() >= p.len() && &hay[..p.len()] == p;
    Ok(Value::bool(r))
}

fn native_ends_with(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let suf = want_str(args[1])?;
    let hay = str_bytes(s);
    let p = str_bytes(suf);
    let r = hay.len() >= p.len() && &hay[hay.len() - p.len()..] == p;
    Ok(Value::bool(r))
}

fn native_replace(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let from = want_str(args[1])?;
    let to = want_str(args[2])?;
    let hay = str_bytes(s).to_vec();
    let fromb = str_bytes(from).to_vec();
    let tob = str_bytes(to).to_vec();
    if fromb.is_empty() {
        return Ok(Value::from_obj(vm.new_str(&hay)));
    }
    let mut out: Vec<u8> = Vec::new();
    let mut i = 0;
    while i < hay.len() {
        if i + fromb.len() <= hay.len() && &hay[i..i + fromb.len()] == &fromb[..] {
            out.extend_from_slice(&tob);
            i += fromb.len();
        } else {
            out.push(hay[i]);
            i += 1;
        }
    }
    Ok(Value::from_obj(vm.new_str(&out)))
}

fn native_repeat(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let n = want_num(args[1])?;
    if n < 0.0 {
        return Err(SableError::Runtime("negative repeat count".to_string()));
    }
    let count = n as usize;
    let src = str_bytes(s).to_vec();
    let total = match src.len().checked_mul(count) {
        Some(t) => t,
        None => return Err(SableError::Runtime("repeat overflow".to_string())),
    };
    if total > 10_000_000 {
        return Err(SableError::Runtime("repeat result too large".to_string()));
    }
    let mut out = Vec::with_capacity(total);
    for _ in 0..count {
        out.extend_from_slice(&src);
    }
    Ok(Value::from_obj(vm.new_str(&out)))
}

fn native_char_at(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let i = want_num(args[1])? as i64;
    let src = str_bytes(s);
    if i < 0 || i as usize >= src.len() {
        return Ok(Value::from_obj(vm.new_str(b"")));
    }
    let b = src[i as usize];
    Ok(Value::from_obj(vm.new_str(&[b])))
}

fn native_code_at(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let i = want_num(args[1])? as i64;
    let src = str_bytes(s);
    if i < 0 || i as usize >= src.len() {
        return Ok(Value::number(-1.0));
    }
    Ok(Value::number(src[i as usize] as f64))
}

fn native_from_code(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let n = want_num(args[0])? as i64;
    let b = (n & 0xff) as u8;
    Ok(Value::from_obj(vm.new_str(&[b])))
}

fn native_split(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let sep = want_str(args[1])?;
    let hay = str_bytes(s).to_vec();
    let sepb = str_bytes(sep).to_vec();
    let mut parts: Vec<(usize, usize)> = Vec::new();
    if sepb.is_empty() {
        let mut i = 0;
        while i < hay.len() {
            parts.push((i, i + 1));
            i += 1;
        }
        if hay.is_empty() {
            parts.push((0, 0));
        }
    } else {
        let mut start = 0;
        let mut i = 0;
        while i + sepb.len() <= hay.len() {
            if &hay[i..i + sepb.len()] == &sepb[..] {
                parts.push((start, i));
                i += sepb.len();
                start = i;
            } else {
                i += 1;
            }
        }
        parts.push((start, hay.len()));
    }
    let n = parts.len();
    let arr = vm.new_array(n);
    vm.push_temp(arr as *mut Obj);
    for (idx, &(a, b)) in parts.iter().enumerate() {
        let so = vm.new_str(&hay[a..b]);
        unsafe {
            *(*arr).items.add(idx) = Value::from_obj(so);
        }
    }
    vm.pop_temp();
    Ok(Value::from_obj(arr as *mut Obj))
}

fn native_pop(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    unsafe {
        if (*a).len == 0 {
            return Ok(Value::nil());
        }
        (*a).len -= 1;
        Ok(*(*a).items.add((*a).len))
    }
}

fn native_slice(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let len = unsafe { (*a).len };
    let mut s = clamp_idx(want_num(args[1])?, len);
    let e = clamp_idx(want_num(args[2])?, len);
    if s > e {
        s = e;
    }
    let n = e - s;
    let res = vm.new_array(n);
    unsafe {
        for i in 0..n {
            *(*res).items.add(i) = *(*a).items.add(s + i);
        }
    }
    Ok(Value::from_obj(res as *mut Obj))
}

fn native_reverse(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    unsafe {
        let len = (*a).len;
        if len > 1 {
            let mut i = 0;
            let mut j = len - 1;
            while i < j {
                let t = *(*a).items.add(i);
                *(*a).items.add(i) = *(*a).items.add(j);
                *(*a).items.add(j) = t;
                i += 1;
                j -= 1;
            }
        }
    }
    Ok(Value::nil())
}

fn native_array_contains(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let target = args[1];
    let len = unsafe { (*a).len };
    for i in 0..len {
        let e = unsafe { *(*a).items.add(i) };
        if val_eq(e, target) {
            return Ok(Value::bool(true));
        }
    }
    Ok(Value::bool(false))
}

fn native_array_index_of(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let target = args[1];
    let len = unsafe { (*a).len };
    for i in 0..len {
        let e = unsafe { *(*a).items.add(i) };
        if val_eq(e, target) {
            return Ok(Value::number(i as f64));
        }
    }
    Ok(Value::number(-1.0))
}

fn native_sort(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let len = unsafe { (*a).len };
    let mut vals: Vec<f64> = Vec::with_capacity(len);
    for i in 0..len {
        let e = unsafe { *(*a).items.add(i) };
        if !e.is_number() {
            return Err(SableError::Type("sort requires numbers".to_string()));
        }
        vals.push(e.as_number());
    }
    vals.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    for i in 0..len {
        unsafe {
            *(*a).items.add(i) = Value::number(vals[i]);
        }
    }
    Ok(Value::nil())
}

fn native_sum(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let len = unsafe { (*a).len };
    let mut total = 0.0;
    for i in 0..len {
        let e = unsafe { *(*a).items.add(i) };
        if e.is_number() {
            total += e.as_number();
        }
    }
    Ok(Value::number(total))
}

fn native_min_of(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let len = unsafe { (*a).len };
    let mut best: Option<f64> = None;
    for i in 0..len {
        let e = unsafe { *(*a).items.add(i) };
        if e.is_number() {
            let n = e.as_number();
            best = Some(match best {
                Some(b) if b <= n => b,
                _ => n,
            });
        }
    }
    Ok(match best {
        Some(b) => Value::number(b),
        None => Value::nil(),
    })
}

fn native_max_of(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let len = unsafe { (*a).len };
    let mut best: Option<f64> = None;
    for i in 0..len {
        let e = unsafe { *(*a).items.add(i) };
        if e.is_number() {
            let n = e.as_number();
            best = Some(match best {
                Some(b) if b >= n => b,
                _ => n,
            });
        }
    }
    Ok(match best {
        Some(b) => Value::number(b),
        None => Value::nil(),
    })
}

fn native_insert(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let v = args[2];
    unsafe {
        let len = (*a).len;
        let idx = clamp_idx(want_num(args[1])?, len);
        array_ensure_cap(a, len + 1);
        let mut j = len;
        while j > idx {
            *(*a).items.add(j) = *(*a).items.add(j - 1);
            j -= 1;
        }
        *(*a).items.add(idx) = v;
        (*a).len = len + 1;
    }
    Ok(Value::nil())
}

fn native_remove(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    unsafe {
        let len = (*a).len;
        if len == 0 {
            return Ok(Value::nil());
        }
        let idx = clamp_idx(want_num(args[1])?, len - 1);
        let removed = *(*a).items.add(idx);
        let mut j = idx;
        while j + 1 < len {
            *(*a).items.add(j) = *(*a).items.add(j + 1);
            j += 1;
        }
        (*a).len = len - 1;
        Ok(removed)
    }
}

fn native_concat(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_array(args[0])?;
    let b = want_array(args[1])?;
    let la = unsafe { (*a).len };
    let lb = unsafe { (*b).len };
    let res = vm.new_array(la + lb);
    unsafe {
        for i in 0..la {
            *(*res).items.add(i) = *(*a).items.add(i);
        }
        for i in 0..lb {
            *(*res).items.add(la + i) = *(*b).items.add(i);
        }
    }
    Ok(Value::from_obj(res as *mut Obj))
}

fn native_min(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_num(args[0])?;
    let b = want_num(args[1])?;
    Ok(Value::number(if a < b { a } else { b }))
}

fn native_max(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_num(args[0])?;
    let b = want_num(args[1])?;
    Ok(Value::number(if a > b { a } else { b }))
}

fn native_pow(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::number(want_num(args[0])?.powf(want_num(args[1])?)))
}

fn native_round(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::number(want_num(args[0])?.round()))
}

fn native_ceil(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::number(want_num(args[0])?.ceil()))
}

fn native_sin(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::number(want_num(args[0])?.sin()))
}

fn native_cos(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::number(want_num(args[0])?.cos()))
}

fn native_tan(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::number(want_num(args[0])?.tan()))
}

fn native_log(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::number(want_num(args[0])?.ln()))
}

fn native_exp(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::number(want_num(args[0])?.exp()))
}

fn native_sign(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let n = want_num(args[0])?;
    let s = if n > 0.0 {
        1.0
    } else if n < 0.0 {
        -1.0
    } else {
        0.0
    };
    Ok(Value::number(s))
}

fn native_clamp(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let x = want_num(args[0])?;
    let lo = want_num(args[1])?;
    let hi = want_num(args[2])?;
    let r = if x < lo {
        lo
    } else if x > hi {
        hi
    } else {
        x
    };
    Ok(Value::number(r))
}

fn native_typeof(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let v = args[0];
    let name = if v.is_number() {
        "number"
    } else if v.is_bool() {
        "boolean"
    } else if v.is_nil() {
        "nil"
    } else {
        match unsafe { (*v.as_obj()).kind } {
            ObjKind::Str => "string",
            ObjKind::Array => "array",
            ObjKind::Map => "map",
            ObjKind::Closure | ObjKind::Proto | ObjKind::Native => "function",
            ObjKind::Upvalue => "upvalue",
        }
    };
    Ok(Value::from_obj(vm.new_str(name.as_bytes())))
}

fn native_is_number(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::bool(args[0].is_number()))
}

fn native_is_string(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::bool(is_kind(args[0], ObjKind::Str)))
}

fn native_is_array(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::bool(is_kind(args[0], ObjKind::Array)))
}

fn native_is_map(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::bool(is_kind(args[0], ObjKind::Map)))
}

fn native_is_function(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let v = args[0];
    let r = is_kind(v, ObjKind::Closure) || is_kind(v, ObjKind::Native);
    Ok(Value::bool(r))
}

fn native_is_nil(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::bool(args[0].is_nil()))
}

fn native_num(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let v = args[0];
    if v.is_number() {
        return Ok(v);
    }
    if is_kind(v, ObjKind::Str) {
        let s = v.as_obj() as *mut ObjStr;
        let bytes = str_bytes(s);
        if let Ok(txt) = std::str::from_utf8(bytes) {
            if let Ok(n) = txt.trim().parse::<f64>() {
                return Ok(Value::number(n));
            }
        }
    }
    Ok(Value::nil())
}

fn native_int(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let v = args[0];
    if v.is_number() {
        return Ok(Value::number(v.as_number().trunc()));
    }
    Ok(Value::nil())
}

fn native_bool(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::bool(args[0].is_truthy()))
}

fn native_has(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let m = want_map(args[0])?;
    let t = unsafe { &(*m).table };
    Ok(Value::bool(t.has(args[1])))
}

fn native_size(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let m = want_map(args[0])?;
    let n = unsafe { (*m).table.count };
    Ok(Value::number(n as f64))
}

fn native_values(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let m = want_map(args[0])?;
    let mut vals: Vec<Value> = Vec::new();
    unsafe {
        let t = &(*m).table;
        for i in 0..t.cap {
            let e = t.entries.add(i);
            if (*e).used {
                vals.push((*e).value);
            }
        }
    }
    let arr = vm.new_array(vals.len());
    unsafe {
        for (i, v) in vals.iter().enumerate() {
            *(*arr).items.add(i) = *v;
        }
    }
    Ok(Value::from_obj(arr as *mut Obj))
}

fn native_band(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_num(args[0])? as i64;
    let b = want_num(args[1])? as i64;
    Ok(Value::number((a & b) as f64))
}

fn native_bor(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_num(args[0])? as i64;
    let b = want_num(args[1])? as i64;
    Ok(Value::number((a | b) as f64))
}

fn native_bxor(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_num(args[0])? as i64;
    let b = want_num(args[1])? as i64;
    Ok(Value::number((a ^ b) as f64))
}

fn native_bnot(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_num(args[0])? as i64;
    Ok(Value::number((!a) as f64))
}

fn native_shl(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_num(args[0])? as i64;
    let n = (want_num(args[1])? as i64 & 63) as u32;
    Ok(Value::number(a.wrapping_shl(n) as f64))
}

fn native_shr(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let a = want_num(args[0])? as i64;
    let n = (want_num(args[1])? as i64 & 63) as u32;
    Ok(Value::number(a.wrapping_shr(n) as f64))
}

fn native_gcd(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let mut a = (want_num(args[0])? as i64).unsigned_abs();
    let mut b = (want_num(args[1])? as i64).unsigned_abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    Ok(Value::number(a as f64))
}

fn native_hypot(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::number(want_num(args[0])?.hypot(want_num(args[1])?)))
}

fn native_atan2(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::number(want_num(args[0])?.atan2(want_num(args[1])?)))
}

fn native_log2(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    Ok(Value::number(want_num(args[0])?.log2()))
}

fn native_hex_encode(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let out = crate::encoding::hex_encode(str_bytes(s));
    Ok(Value::from_obj(vm.new_str(out.as_bytes())))
}

fn native_hex_decode(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    match crate::encoding::hex_decode(str_bytes(s)) {
        Some(bytes) => Ok(Value::from_obj(vm.new_str(&bytes))),
        None => Ok(Value::nil()),
    }
}

fn native_base64_encode(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let out = crate::encoding::base64_encode(str_bytes(s));
    Ok(Value::from_obj(vm.new_str(out.as_bytes())))
}

fn native_base64_decode(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    match crate::encoding::base64_decode(str_bytes(s)) {
        Some(bytes) => Ok(Value::from_obj(vm.new_str(&bytes))),
        None => Ok(Value::nil()),
    }
}

fn native_json_parse(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let parsed = crate::json::parse(str_bytes(s));
    match parsed {
        Ok(j) => Ok(json_to_value(vm, &j)),
        Err(_) => Ok(Value::nil()),
    }
}

fn native_json_stringify(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let j = value_to_json(args[0], 64);
    let s = crate::json::stringify(&j);
    Ok(Value::from_obj(vm.new_str(s.as_bytes())))
}

fn native_format(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let fmt = want_str(args[0])?;
    let arr = want_array(args[1])?;
    let fbytes = str_bytes(fmt).to_vec();
    let n = unsafe { (*arr).len };
    let mut out: Vec<u8> = Vec::new();
    let mut argi = 0;
    let mut i = 0;
    while i < fbytes.len() {
        if i + 1 < fbytes.len() && fbytes[i] == b'{' && fbytes[i + 1] == b'}' {
            if argi < n {
                let v = unsafe { *(*arr).items.add(argi) };
                let rendered = format_value(v, 4);
                out.extend_from_slice(rendered.as_bytes());
                argi += 1;
            }
            i += 2;
        } else if i + 1 < fbytes.len() && fbytes[i] == b'{' && fbytes[i + 1] == b'{' {
            out.push(b'{');
            i += 2;
        } else if i + 1 < fbytes.len() && fbytes[i] == b'}' && fbytes[i + 1] == b'}' {
            out.push(b'}');
            i += 2;
        } else {
            out.push(fbytes[i]);
            i += 1;
        }
    }
    Ok(Value::from_obj(vm.new_str(&out)))
}

fn native_pad_left(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let width = want_num(args[1])? as usize;
    let pad = want_str(args[2])?;
    let src = str_bytes(s).to_vec();
    let padb = str_bytes(pad).to_vec();
    let padc = if padb.is_empty() { b' ' } else { padb[0] };
    let mut out: Vec<u8> = Vec::new();
    if src.len() < width {
        for _ in 0..(width - src.len()) {
            out.push(padc);
        }
    }
    out.extend_from_slice(&src);
    Ok(Value::from_obj(vm.new_str(&out)))
}

fn native_pad_right(vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let width = want_num(args[1])? as usize;
    let pad = want_str(args[2])?;
    let src = str_bytes(s).to_vec();
    let padb = str_bytes(pad).to_vec();
    let padc = if padb.is_empty() { b' ' } else { padb[0] };
    let mut out: Vec<u8> = src.clone();
    if src.len() < width {
        for _ in 0..(width - src.len()) {
            out.push(padc);
        }
    }
    Ok(Value::from_obj(vm.new_str(&out)))
}

fn native_count_sub(_vm: &mut Vm, args: &[Value]) -> SableResult<Value> {
    let s = want_str(args[0])?;
    let sub = want_str(args[1])?;
    let hay = str_bytes(s);
    let needle = str_bytes(sub);
    if needle.is_empty() {
        return Ok(Value::number(0.0));
    }
    let mut count = 0;
    let mut i = 0;
    while i + needle.len() <= hay.len() {
        if &hay[i..i + needle.len()] == needle {
            count += 1;
            i += needle.len();
        } else {
            i += 1;
        }
    }
    Ok(Value::number(count as f64))
}
