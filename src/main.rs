use {rayon::prelude::*, rug::Rational};

fn main() {
    let limit = 1_000_000;
    let result = (0i64..=limit)
        .into_par_iter()
        .map(|i| if i % 2 == 0 { 2 * i + 1 } else { -2 * i - 1 })
        .map(|denominator| Rational::from((1i64, denominator)))
        .sum::<Rational>()
        .to_f64();
    println!("limit:{} => {}", limit, 4.0 * result);
}
