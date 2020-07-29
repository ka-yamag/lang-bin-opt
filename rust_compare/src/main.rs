fn main() {
    let a: &'static str = "abcdefghijklkm012345";
    let b: &'static str = "012345:;++)))(*(&*^%kadjfklajfklaikadjfkladjfkladjsj";

    if constant_time_compare(a.as_bytes(), b.as_bytes()) {
        println!("match!");
    } else {
        println!("unmatch!");
    }
}

fn constant_time_compare(x: &[u8], y: &[u8]) -> bool {
    if x.len() != y.len() {
        return false
    }

    let mut _v: u8 = 0x00;

    for (i, _) in x.iter().enumerate() {
        _v |= x[i] ^ y[i];
    }

    if constant_time_byteeq(_v, 0) == 0 {
        return false
    }

    true
}

fn constant_time_byteeq(x: u8, y: u8) -> i32 {
    let binary_xor: i32 = From::from(x ^ y);

    (binary_xor - 1) >> 31
}
