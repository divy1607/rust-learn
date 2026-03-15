mod utils;

use utils::math;
use utils::printer;

fn main() {
    let num1 = 5;
    let num2 = 3;

    let add = math::add(num1, num2);
    let subtract = math::subtract(num1, num2);
    let pr = math::multiply(num1, num2);

    printer::print_results(add);
    printer::print_results(subtract);
    printer::print_results(pr);
}