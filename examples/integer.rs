use range_ranger::Range;

fn stdlib_range() {
    let r: Range<_> = (1..5).into();

    println!("Range: {r:?}");
    println!("Contains 1: {}", r.contains(1));
    println!("Contains 4: {}", r.contains(4));
    println!("Contains 5: {}", r.contains(5));
    println!("Contains 10: {}", r.contains(10));
}

fn stdlib_rangeinclusive() {
    let r: Range<_> = (1..=5).into();

    println!("Range: {r:?}");
    println!("Contains 1: {}", r.contains(1));
    println!("Contains 4: {}", r.contains(4));
    println!("Contains 5: {}", r.contains(5));
    println!("Contains 10: {}", r.contains(10));
}

fn stdlib_rangeto() {
    let r: Range<_> = (..5).into();

    println!("Range: {r:?}");
    println!("Contains 1: {}", r.contains(1));
    println!("Contains 4: {}", r.contains(4));
    println!("Contains 5: {}", r.contains(5));
    println!("Contains 10: {}", r.contains(10));
}

fn stdlib_rangetoinclusive() {
    let r: Range<_> = (..=5).into();

    println!("Range: {r:?}");
    println!("Contains 1: {}", r.contains(1));
    println!("Contains 4: {}", r.contains(4));
    println!("Contains 5: {}", r.contains(5));
    println!("Contains 10: {}", r.contains(10));
}

fn stdlib_rangefull() {
    let r: Range<_> = (..).into();

    println!("Range: {r:?}");
    println!("Contains 1: {}", r.contains(1));
    println!("Contains 4: {}", r.contains(4));
    println!("Contains 5: {}", r.contains(5));
    println!("Contains 10: {}", r.contains(10));
}

fn composite() {
    let r: Range<_> = Range::continuous(1, 2).union(&(5..=5).into());

    println!("Range: {r:?}");
    println!("Contains 1: {}", r.contains(1));
    println!("Contains 4: {}", r.contains(4));
    println!("Contains 5: {}", r.contains(5));
    println!("Contains 10: {}", r.contains(10));
}

fn main() {
    stdlib_range();
    stdlib_rangeinclusive();
    stdlib_rangefull();
    stdlib_rangeto();
    stdlib_rangetoinclusive();
    composite();
}
