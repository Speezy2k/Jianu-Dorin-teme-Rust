fn add_chars_n_p1(mut s: String, c: char, n: i32) -> String {
    for _j in 0..n {
        s.push(c);
    }
    s
}

fn add_chars_n_p2(s: &mut String, c: char, n: i32) {
    for _j in 0..n {
        s.push(c);
    }
}

fn main() {
    let mut s1 = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s1 = add_chars_n_p1(s1, c, 26 - i);
        i += 1;
    }
    println!("{}", s1);

    let mut s2 = String::from("");
    let mut k = 0;
    while k < 26 {
        let c = (k as u8 + b'a') as char;
        add_chars_n_p2(&mut s2, c, 26 - k);
        k += 1;
    }

    println!();
    println!("{}", s2);
}
