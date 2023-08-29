
pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let len = gcd(str1.len(), str2.len());
    let str1_bytes = str1.as_bytes();
    let str2_bytes = str2.as_bytes();

    if [str1_bytes, str2_bytes]
        .concat()
        .chunks(len)
        .all(|c| c == &str1_bytes[0..len])
    {
        str1[0..len].into()
    } else {
        "".into()
    }
}

fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
