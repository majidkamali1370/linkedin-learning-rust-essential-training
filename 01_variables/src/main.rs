fn main() {
    let v1: i8 = 42;
    let v2: i16 = 42;
    let v3: i32 = 42;
    let v4: i64 = 42;
    let v5: i128 = 42;

    println!(
        "v1 = {}, v2 = {}, v3 = {}, v4 = {}, v5 = {}",
        v1, v2, v3, v4, v5
    );

    let mut muv1: u8 = 1;
    let mut muv2: u16 = 1;
    let mut muv3: u32 = 1;
    let mut muv4: u64 = 1;
    let mut muv5: u128 = 1;

    println!(
        "muv1 = {}, muv2 = {}, muv3 = {}, muv4 = {}, muv5 = {}",
        muv1, muv2, muv3, muv4, muv5
    );

    muv1 += 1;
    muv2 += 10;
    muv3 += 100;
    muv4 += 1000;
    muv5 += 10000;
    muv1 += v1 as u8;

    println!(
        "muv1 = {}, muv2 = {}, muv3 = {}, muv4 = {}, muv5 = {}",
        muv1, muv2, muv3, muv4, muv5
    );

    let float: f32 = 1.234;
    let double: f64 = 1.23456789;

    println!("float = {}, double = {}", float, double);
}
