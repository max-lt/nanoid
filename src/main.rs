use clap::Parser;
use rand::rngs::StdRng;
use rand::rngs::SysRng;
use rand::Rng;
use rand::RngExt;
use rand::SeedableRng;

const ALPHABET: &[u8; 58] = b"123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";

const BUFFER_SIZE: usize = 256;

macro_rules! clz32 {
    ($val:expr) => {
        ($val as u32).leading_zeros()
    };
}

const MASK: u8 = (2 << (31 - clz32!(ALPHABET.len() as u32))) - 1;

struct InfiniteRand<R: Rng> {
    rng: R,
    buf: [u8; BUFFER_SIZE],
    index: usize,
}

impl<R: Rng> InfiniteRand<R> {
    fn new(mut rng: R) -> Self {
        let mut buf = [0; BUFFER_SIZE];
        rng.fill(&mut buf[..]);
        InfiniteRand { rng, buf, index: 0 }
    }
}

impl<R: Rng> Iterator for InfiniteRand<R> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.buf[self.index];

        if (self.index + 1) == BUFFER_SIZE {
            self.rng.fill(&mut self.buf[..]);
            self.index = 0;
        } else {
            self.index += 1;
        }

        Some(result)
    }
}

fn nanoid<R: Rng>(inf: &mut InfiniteRand<R>, size: usize) -> String {
    let mut id = String::new();

    loop {
        let index = (inf.next().unwrap() & MASK) as usize;

        if index < ALPHABET.len() {
            id.push(ALPHABET[index] as char);
        }

        if id.len() == size {
            break;
        }
    }

    id
}

#[derive(Parser, Debug)]
struct Args {
    /// Number of IDs to generate
    #[arg(short = 'n', long = "number", default_value_t = 1, value_name = "INT")]
    n: usize,

    /// Size of each ID
    #[arg(short = 's', long = "size", default_value_t = 12, value_name = "INT")]
    size: usize,

    /// Draw randomness from an Infinite Noise TRNG instead of the system RNG
    #[cfg(feature = "infnoise")]
    #[arg(long)]
    infnoise: bool,
}

fn run<R: Rng>(rng: R, args: &Args) {
    let mut inf = InfiniteRand::new(rng);
    for _ in 0..args.n {
        let id = nanoid(&mut inf, args.size);
        println!("{}", id);
    }
}

fn main() {
    let args = Args::parse();

    if args.size == 0 {
        eprintln!("Size must be greater than 0");
        std::process::exit(1);
    }

    if args.n == 0 {
        eprintln!("Number of IDs must be greater than 0");
        std::process::exit(1);
    }

    #[cfg(feature = "infnoise")]
    if args.infnoise {
        let device = infnoise_rs::device::Infnoise::open(None).unwrap_or_else(|err| {
            eprintln!("infnoise: {err}");
            std::process::exit(1);
        });
        return run(rand::rand_core::UnwrapErr(device), &args);
    }

    let rng = StdRng::try_from_rng(&mut SysRng).expect("failed to seed RNG from system entropy");
    run(rng, &args);
}

#[cfg(test)]
mod tests {
    use super::nanoid;
    use super::InfiniteRand;

    fn test_rng() -> impl super::Rng {
        use super::SeedableRng;
        super::StdRng::seed_from_u64(42)
    }

    #[test]
    fn test_nanoid_length() {
        let mut inf = InfiniteRand::new(test_rng());

        let id = nanoid(&mut inf, 170);

        assert_eq!(id.len(), 170);
    }

    #[test]
    fn test_nanoid_alphabet() {
        let mut inf = InfiniteRand::new(test_rng());

        let id = nanoid(&mut inf, 200);

        // Every character in the ID must be in the ALPHABET
        for c in id.chars() {
            assert!(super::ALPHABET.contains(&(c as u8)));
        }
    }
}
