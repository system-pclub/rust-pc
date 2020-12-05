#![allow(unused_variables)]

struct Inner {
    in_a: u8,
    in_b: u8
}
struct Outer1 {
    a: [Inner; 2]
}

struct Outer2 {
    a: (Inner, Inner)
}

fn test_ownership(num: &mut u8, inner: &Inner) {
}

fn main() {
    let mut out1 = Outer1 {
        a: [Inner {in_a: 1, in_b: 2}, Inner {in_a: 3, in_b: 4}]
    };
    let mut out2 = Outer2 {
        a: (Inner {in_a: 1, in_b: 2}, Inner {in_a: 3, in_b: 4})
    };

    if let Some((first,rest)) = out1.a.split_first_mut() {
        test_ownership(&mut first.in_a, &mut rest[0]);
    }

    // But this works!
    test_ownership(&mut out2.a.0.in_a, &out2.a.1); // line 28
}
