fn main() {
    let a = "abcdefghijklkm012345";
    let b = "012345:;++)))(*(&*^%kadjfklajfklaikadjfkladjfkladjsj";

    if constantTimeCompare(a.as_bytes(), b.as_bytes()) {
        println!("match!");
    } else {
        println!("unmatch!");
    }
}

fn constantTimeCompare(x: &[u8], y: &[u8]) -> bool {
    if x.len() != y.len() {
        false
    }

    let v: [u8; x.len()];

    for (i, _) in x.iter().enumerate() {
        v |= x[i] ^ y[i]
    }

    true
}
