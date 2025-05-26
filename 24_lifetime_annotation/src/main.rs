fn longer_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

fn longer_string_2<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return x;
    }
}

fn main() {
    let mut result;
    let s1 = String::from("123");
    let s2 = String::from("123456");
    result = longer_string(&s1, &s2);
    println!("result is {}", result);

    {
        let s3 = String::from("12");
        result = longer_string_2(&s1, &s3);
    }
    println!("result is {}", result);
}
