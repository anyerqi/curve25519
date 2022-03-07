use std::cmp::Ordering;

fn expand_top_bit(a: u8) -> u8 {
    //if (a >> 7) == 1 { 0xFF } else { 0 }
    0u8.wrapping_sub(a >> 7)
}

fn ct_is_zero(a: u8) -> u8 {
    //if a == 0 { 0xFF } else { 0 }
    expand_top_bit(!a & a.wrapping_sub(1))
}


fn ct_is_eq(a: u8, b: u8) -> u8 {
    //if a == b { 0xFF } else { 0 }
    ct_is_zero(a ^ b)
}

fn ct_is_lt(a: u8, b: u8) -> u8 {
    //if a < b { 0xFF } else { 0 }
    expand_top_bit(a ^ ((a ^ b) | ((a.wrapping_sub(b)) ^ a)))
}

fn ct_select(mask: u8, a: u8, b: u8) -> u8 {
    debug_assert!(mask == 0 || mask == 0xFF);
    //if mask == 0xFF { a } else if mask == 0x00 { b } else { unreachable!(); }
    b ^ (mask & (a ^ b))
}

pub(crate) fn constant_time_cmp(x: &[u8], y: &[u8]) -> Ordering {
    if x.len() < y.len() {
        return Ordering::Less;
    }
    if x.len() > y.len() {
        return Ordering::Greater;
    }

    let mut result: u8 = 0;

    for i in 0..x.len() {
        let a = x[x.len() - 1 - i];
        let b = y[x.len() - 1 - i];

        let is_eq = ct_is_eq(a, b);
        let is_lt = ct_is_lt(a, b);

        result = ct_select(is_eq, result, ct_select(is_lt, 1, 255));
    }

    debug_assert!(result == 0 || result == 1 || result == 255);

    if result == 0 {
        Ordering::Equal
    } else if result == 1 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}
