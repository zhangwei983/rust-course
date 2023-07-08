// Print from `a` to `Z` in the  ASCII chart.
# [allow (non_snake_case)]
pub fn print_a2Z() {
    // Reverse as `a` is behind `Z` in the ASCII chart.
    for ch in ('Z'..='a').rev() {
        println!("{}", ch);
    }
}
