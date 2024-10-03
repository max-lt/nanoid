use clap::Parser;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

const ALPHABET: &[u8; 58] = b"123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";

const BUFFER_SIZE: usize = 256;

macro_rules! clz32 {
    ($val:expr) => {
        ($val as u32).leading_zeros()
    };
}

const MASK: u32 = (2 << (31 - clz32!(ALPHABET.len() as u32))) - 1;

struct InfiniteRand {
    rng: StdRng,
    buf: [u8; BUFFER_SIZE],
    index: usize,
}

impl InfiniteRand {
    fn new() -> Self {
        let mut rng = StdRng::from_entropy();
        let mut buf = [0; BUFFER_SIZE];
        rng.fill(&mut buf[..]);
        InfiniteRand { rng, buf, index: 0 }
    }
}

impl Iterator for InfiniteRand {
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

fn nanoid(inf: &mut InfiniteRand, size: usize) -> String {
    let mut id = String::new();

    loop {
        let index = (inf.next().unwrap() & MASK as u8) as usize;

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
}

fn main() {
    let args = Args::parse();

    let mut inf = InfiniteRand::new();

    if args.size <= 0 {
        eprintln!("Size must be greater than 0");
        std::process::exit(1);
    }

    if args.n <= 0 {
        eprintln!("Number of IDs must be greater than 0");
        std::process::exit(1);
    }

    for _ in 0..args.n {
        let id = nanoid(&mut inf, args.size);
        println!("{}", id);
    }
}

#[cfg(test)]
mod tests {
    use super::nanoid;
    use super::InfiniteRand;

    #[test]
    fn test_nanoid_length() {
        let mut inf = InfiniteRand::new();

        let id = nanoid(&mut inf, 170);

        assert_eq!(id.len(), 170);
    }

    #[test]
    fn test_nanoid_alphabet() {
        let mut inf = InfiniteRand::new();

        let id = nanoid(&mut inf, 200);

        // Every character in the ID must be in the ALPHABET
        for c in id.chars() {
            assert!(super::ALPHABET.contains(&(c as u8)));
        }
    }
}
