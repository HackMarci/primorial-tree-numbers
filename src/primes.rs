use std::{
    collections::BTreeMap,
    error::Error,
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    ops::Index,
    path::PathBuf,
};

use bit_array::BitArray;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Primes {
    primes: Vec<usize>,
}

impl Primes {
    pub fn extent(&self) -> usize {
        self.primes.len()
    }

    pub fn try_new(bound: usize, primes_cache_path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let primes = match OpenOptions::new()
            .read(true)
            .write(true)
            .open(primes_cache_path)
        {
            Ok(mut file) => {
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes)?;
                let mut out: Primes = bincode::deserialize(&bytes[..])?;

                if out.primes.len() < bound {
                    println!(
                        "New bound ({bound}) is larger than the extant of {}. Regenerating",
                        primes_cache_path.to_string_lossy()
                    );
                    out = Primes {
                        primes: Self::aproximatly_to_nth(bound),
                    };
                    file.seek(SeekFrom::Start(0))?;
                    file.write_all(&bincode::serialize(&out)?[..])?;
                }
                out
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                println!(
                    "Cache not found on {0} Generating new cache file.",
                    primes_cache_path.to_string_lossy()
                );
                let out = Primes {
                    primes: Self::aproximatly_to_nth(bound),
                };
                let file = &mut File::create(primes_cache_path)?;
                file.write_all(&bincode::serialize(&out)?[..])?;
                out
            }
            Err(err) => Err(Box::new(err))?,
        };
        Ok(primes)
    }

    fn aproximatly_to_nth(n: usize) -> Vec<usize> {
        let nf = n as f32;
        let limit = (nf * ((nf).ln() + (nf).ln().ln())) as usize;
        Self::all_less_than(limit)
    }

    fn all_less_than(n: usize) -> Vec<usize> {
        let mut mask = BitArray::new_ones(n);
        mask.reset_unchecked(0);
        mask.reset_unchecked(1);

        for i in 2..((n as f32).sqrt().ceil() as usize) {
            if mask[i] {
                for j in ((i * i)..n).step_by(i) {
                    mask.reset_unchecked(j)
                }
            }
        }

        let mut iter = mask.iter();
        (0..n).filter(|_| *iter.next().unwrap()).collect()
    }

    pub fn factorize(&self, num: usize) -> BTreeMap<usize, usize> {
        let mut factors = BTreeMap::new();

        let mut i = 0;
        let mut p = self.primes[i];
        let mut freq;
        let mut num = num;

        while p * p <= num {
            freq = 0;
            while num % p == 0 {
                num /= p;
                freq += 1;
            }
            if freq != 0 {
                factors.insert(i, freq);
            }
            i += 1;
            p = self.primes[i]
        }

        if num > 1 {
            factors.insert(
                self.primes.binary_search(&num).expect(
                    &format!("{num} is not in the primes list. Try again with a larger bound")[..],
                ),
                1,
            );
        }

        factors
    }
}

impl Index<usize> for Primes {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.primes[index]
    }
}

#[cfg(test)]
mod test {
    use crate::primes::Primes;

    #[test]
    fn nth() {
        assert_eq!(
            *Primes::aproximatly_to_nth(1000 - 1)
                .iter()
                .nth(1000 - 1)
                .unwrap(),
            7919
        );
    }

    #[test]
    fn less_than() {
        assert_eq!(Primes::all_less_than(1000).len(), 168);
    }

    #[test]
    fn factorize() {
        let primes = Primes {
            primes: Primes::aproximatly_to_nth(1000),
        };

        let num = 100;
        let factors = primes.factorize(num);
        let num_back: usize = factors
            .iter()
            .map(|(p, e)| primes[*p].pow(usize::from(*e) as u32))
            .product();
        assert_eq!(num_back, num);
    }
}
