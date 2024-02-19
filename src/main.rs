mod prime_tree_num;
mod primes;

use std::{error::Error, path::PathBuf};

use clap::Parser;

use crate::{prime_tree_num::PrimeTree, primes::Primes};
#[derive(Parser)]
struct CliArgs {
    numbers: Vec<usize>,
    #[arg(long, short, default_value = "primes.cache")]
    primes_cache_path: PathBuf,
    #[arg(long, short, default_value_t = 1_000_000)]
    bound: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let CliArgs {
        numbers,
        bound,
        primes_cache_path: primes_file,
    } = CliArgs::parse();

    let primes = Primes::try_new(bound, &primes_file)?;
    println!(
        "Specified bound: {}. Extent of cache file ({}): {}.",
        bound,
        primes_file.to_string_lossy(),
        primes.extent()
    );

    for number in numbers {
        println!();
        println!(
            "Prime factors: {}",
            primes
                .factorize(number)
                .into_iter()
                .map(|(k, v)| (primes[k], v))
                .fold(String::new(), |s, (k, v)| {
                    let is_empty = s.is_empty();
                    s + &format!("{} {k} ^ {v} ", if is_empty { "" } else { "*" })[..]
                })
        );

        let mut tree = PrimeTree::new(&primes);
        tree.fill_with_num(number);

        println!("Tree form: {}", tree);
        println!();
    }

    Ok(())
}
