mod printer;
mod inner;

fn main() {
    println!("Print from `a` to `Z` in ASCII chart:");
    printer::print_a2Z();

    println!("Print from `A` to `z` in ASCII chart:");
    inner::printer::print_A2z();
}
