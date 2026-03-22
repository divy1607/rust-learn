macro_rules! max {
    ($x: expr, $y: expr) => {
        if $x > $y { $x } else { $y }
    };
}

fn main() {
    println!("{}", max!(10, 7));
    println!("{}", max!(3.14, 4.13));
}