#![feature(test)]

use std::convert::TryInto;

struct State {
    lo: u32,
    hi: u32,
}

impl State {
    pub fn update(&mut self, v: u32) {
        self.lo = self.lo.wrapping_add(v);
        self.hi ^= self.lo;
        self.lo = self.lo.rotate_left(20).wrapping_add(self.hi);
        self.hi = self.hi.rotate_left(9) ^ self.lo;
        self.lo = self.lo.rotate_left(27).wrapping_add(self.hi);
        self.hi = self.hi.rotate_left(19);
    }
}

pub fn hash(seed: u64, data: &[u8]) -> u32 {
    let mut bytes = data;

    let mut s = State {
        lo: seed as u32,
        hi: (seed >> 32) as u32,
    };

    while bytes.len() >= 4 {
        let k1 = u32::from_le_bytes(bytes[..4].try_into().unwrap());
        s.update(k1);
        bytes = &bytes[4..];
    }

    let fin = match bytes.len() {
        0 => 0x80,
        1 => (0x80 as u32) << 8 | bytes[0] as u32,
        2 => (0x80 as u32) << 16 | u16::from_le_bytes(bytes[..2].try_into().unwrap()) as u32,
        3 => {
            (0x80 as u32) << 24
                | u16::from_le_bytes(bytes[..2].try_into().unwrap()) as u32
                | (bytes[2] as u32) << 16
        }
        _ => panic!("len > 3"),
    };

    s.update(fin);
    s.update(0);

    s.lo ^ s.hi
}

extern crate test;

#[cfg(test)]
mod tests {

    use super::*;

    #[deny(soft_unstable)]
    use test::Bencher;

    macro_rules! test_one {
        ($h:expr, $data:expr) => {{
            let got = hash(0x5D70D359C498B3F8, $data);
            assert_eq!($h, got, "data={:?}", $data);
        }};
    }

    #[test]
    fn smoke() {
        test_one!(0xf7f2c954, b"");
        test_one!(0xd46e71f7, b"a");
        test_one!(0xb40c651c, b"ab");
        test_one!(0x5b3bc23d, b"abc");
        test_one!(0x6b15e57b, b"abcd");
        test_one!(0x601e6ea8, b"abcde");
        test_one!(0xfc18bd2c, b"abcdef");
        test_one!(0x79b01bfb, b"abcdefg");
        test_one!(0x54793238, b"abcdefgh");
        test_one!(0xebf98191, b"abcdefghi");
        test_one!(0x68a8001d, b"abcdefghij");
        test_one!(0x659105c1, b"Discard medicine more than two years old.");
        test_one!(
            0xb98b31d,
            b"He who has a shady past knows that nice guys finish last."
        );
        test_one!(0xbae17c9a, b"I wouldn't marry him with a ten foot pole.");
        test_one!(
            0x9a299f69,
            b"Free! Free!/A trip/to Mars/for 900/empty jars/Burma Shave"
        );
        test_one!(
            0xb463d704,
            b"The days of the digital watch are numbered.  -Tom Stoppard"
        );
        test_one!(0xe6059c5f, b"Nepal premier won't resign.");
        test_one!(
            0xbdd4f772,
            b"For every action there is an equal and opposite government program."
        );
        test_one!(
            0x12af7ede,
            b"His money is twice tainted: 'taint yours and 'taint mine."
        );
        test_one!(0x1e9cae8, b"There is no reason for any individual to have a computer in their home. -Ken Olsen, 1977");
        test_one!(
            0xcb683e33,
            b"It's a tiny change to the code and not completely disgusting. - Bob Manchek"
        );
        test_one!(0x2074fbfa, b"size:  a.out:  bad magic");
        test_one!(
            0x52abb615,
            b"The major problem is with sendmail.  -Mark Horton"
        );
        test_one!(
            0x5a509711,
            b"Give me a rock, paper and scissors and I will move the world.  CCFestoon"
        );
        test_one!(
            0xf97f5273,
            b"If the enemy is within range, then so are you."
        );
        test_one!(
            0x494c0cb,
            b"It's well we cannot hear the screams/That we create in others' dreams."
        );
        test_one!(
            0x7150a3c0,
            b"You remind me of a TV show, but that's all right: I watch it anyway."
        );
        test_one!(0xc5f56430, b"C is as portable as Stonehedge!!");
        test_one!(0x712bcf01, b"Even if I could be Shakespeare, I think I should still choose to be Faraday. - A. Huxley");
        test_one!(0xedd44de6, b"The fugacity of a constituent in a mixture of gases at a given temperature is proportional to its mole fraction.  Lewis-Randall Rule");
        test_one!(
            0xd9440105,
            b"How can you write a big system without C++?  -Paul Glick"
        );
    }

    macro_rules! bench_hash {
        ($name:ident, $size:expr) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                let mut val: u32 = 0;

                let mut v = Vec::<u8>::new();
                v.resize($size, 0);

                b.iter(|| {
                    val += hash(0, &v);
                })
            }
        };
    }

    bench_hash!(benchmark_0008, 8);
    bench_hash!(benchmark_0016, 16);
    bench_hash!(benchmark_0032, 32);
    bench_hash!(benchmark_0040, 40);
    bench_hash!(benchmark_0060, 60);
    bench_hash!(benchmark_0064, 64);
    bench_hash!(benchmark_0072, 72);
    bench_hash!(benchmark_0080, 80);
    bench_hash!(benchmark_0100, 100);
    bench_hash!(benchmark_0150, 150);
    bench_hash!(benchmark_0200, 200);
    bench_hash!(benchmark_0250, 250);
    bench_hash!(benchmark_0512, 512);
    bench_hash!(benchmark_1024, 1024);
    bench_hash!(benchmark_8192, 8192);
}
