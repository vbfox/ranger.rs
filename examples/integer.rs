use range_ranger::Range;

fn main() {
    let r: Range<_> = (1..=5).into();

    println!("Range: {:?}", r);
    println!("Contains 4: {}", r.contains(4));
    println!("Contains 10: {}", r.contains(10));
}
