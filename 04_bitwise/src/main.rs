fn main() {
    let a = 0b1111_0101u8; // Binary representation of number, starts with 0b
    let mask_2 = 0b0000_0100u8;

    println!("{:>16} = {}", "a", a);
    println!("{:>16} = {:08b}", "a", a); // >16 left-aligns the value, with a width of 16
    println!("{:>16} = {:08b}", "!a", !a); // b suffix prints binary representation of the value
    println!("{:>16} = {:08b}", "a & mask_2", a & mask_2);
    println!("{:>16} = {:08b}", "a & !mask_2", a & !mask_2);
    println!("{:>16} = {:08b}", "a | 0x0c", a | 0x0c);
    println!("{:>16} = {:08b}", "a ^ 0x0f", a ^ 0x0f);
    println!("{:>16} = {:08b}", "a << 2", a << 2);
    println!("{:>16} = {:08b}", "a >> 4", a >> 4);
}
