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

    use hex;

    macro_rules! test_one {
        ($seed:expr, $data:expr, $h:expr) => {{
            let input = if $data.len() > 0 {
                hex::decode($data).unwrap()
            } else {
                Vec::<u8>::new()
            };
            let want = ($h >> 32) ^ $h;
            let got = hash($seed, &input);
            assert_eq!(want as u32, got, "data={:?}", $data);
        }};
    }

    #[test]
    fn test_dotnet_runtime() {
        // Tests from https://github.com/dotnet/runtime/blob/master/src/libraries/Common/tests/Tests/System/MarvinTests.cs

        let seed_1: u64 = 0x4FB61A001BDBCC;
        let seed_2: u64 = 0x804FB61A001BDBCC;
        let seed_3: u64 = 0x804FB61A801BDBCC;
        let test_data_string_0_byte = b"";
        let test_data_string_1_byte = b"af";
        let test_data_string_2_byte = b"e70f";
        let test_data_string_3_byte = b"37f495";
        let test_data_string_4_byte = b"8642dc59";
        let test_data_string_5_byte = b"153fb79826";
        let test_data_string_6_byte = b"0932e6246c47";
        let test_data_string_7_byte = b"ab427ea8d10fc7";

        test_one!(seed_1, test_data_string_0_byte, 0x30ED35C100CD3C7Du64);
        test_one!(seed_1, test_data_string_1_byte, 0x48E73FC77D75DDC1u64);
        test_one!(seed_1, test_data_string_2_byte, 0xB5F6E1FC485DBFF8u64);
        test_one!(seed_1, test_data_string_3_byte, 0xF0B07C789B8CF7E8u64);
        test_one!(seed_1, test_data_string_4_byte, 0x7008F2E87E9CF556u64);
        test_one!(seed_1, test_data_string_5_byte, 0xE6C08C6DA2AFA997u64);
        test_one!(seed_1, test_data_string_6_byte, 0x6F04BF1A5EA24060u64);
        test_one!(seed_1, test_data_string_7_byte, 0xE11847E4F0678C41u64);

        test_one!(seed_2, test_data_string_0_byte, 0x10A9D5D3996FD65Du64);
        test_one!(seed_2, test_data_string_1_byte, 0x68201F91960EBF91u64);
        test_one!(seed_2, test_data_string_2_byte, 0x64B581631F6AB378u64);
        test_one!(seed_2, test_data_string_3_byte, 0xE1F2DFA6E5131408u64);
        test_one!(seed_2, test_data_string_4_byte, 0x36289D9654FB49F6u64);
        test_one!(seed_2, test_data_string_5_byte, 0xA06114B13464DBDu64);
        test_one!(seed_2, test_data_string_6_byte, 0xD6DD5E40AD1BC2EDu64);
        test_one!(seed_2, test_data_string_7_byte, 0xE203987DBA252FB3u64);

        test_one!(seed_3, "00", 0xA37FB0DA2ECAE06Cu64);
        test_one!(seed_3, "FF", 0xFECEF370701AE054u64);
        test_one!(seed_3, "00FF", 0xA638E75700048880u64);
        test_one!(seed_3, "FF00", 0xBDFB46D969730E2Au64);
        test_one!(seed_3, "FF00FF", 0x9D8577C0FE0D30BFu64);
        test_one!(seed_3, "00FF00", 0x4F9FBDDE15099497u64);
        test_one!(seed_3, "00FF00FF", 0x24EAA279D9A529CAu64);
        test_one!(seed_3, "FF00FF00", 0xD3BEC7726B057943u64);
        test_one!(seed_3, "FF00FF00FF", 0x920B62BBCA3E0B72u64);
        test_one!(seed_3, "00FF00FF00", 0x1D7DDF9DFDF3C1BFu64);
        test_one!(seed_3, "00FF00FF00FF", 0xEC21276A17E821A5u64);
        test_one!(seed_3, "FF00FF00FF00", 0x6911A53CA8C12254u64);
        test_one!(seed_3, "FF00FF00FF00FF", 0xFDFD187B1D3CE784u64);
        test_one!(seed_3, "00FF00FF00FF00", 0x71876F2EFB1B0EE8u64);
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
