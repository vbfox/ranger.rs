use range_ranger::Range;

fn main() {
    let r: Range<_> = (1..=5).into();

    println!("Range: {:?}", r);
}