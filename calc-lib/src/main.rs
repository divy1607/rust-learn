use calc_lib::math::basic;
use calc_lib::add;
use calc_lib::multiply;

fn main() {
    let x = 6;
    let y = 4;
    let ans1 = add(x, y);
    let ans2 = multiply(x, y);
    let ans = basic::square(x);
    let anss = basic::cube(y);
    println!("{}", ans);
    println!("{}", ans1);
    println!("{}", ans2);
    println!("{}", anss);

}