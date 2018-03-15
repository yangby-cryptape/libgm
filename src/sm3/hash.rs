//
// Created by Niujx on 2018-03-12.
//
// Sample 1
// Input:"abc"
// Output:66c7f0f4 62eeedd9 d1f2d46b dc10e4e2 4167c487 5cf2f7a2 297da02b 8f4ba8e0

// Sample 2
// Input:"abcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcd"
// Outpuf:debe9ff9 2275b8a1 38604889 c18e5a4d 6fdb70e5 387e5765 293dcba3 9c0c5732

fn ff0(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

fn ff1(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (x & z) | (y & z)
}

fn gg0(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

fn gg1(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

fn p0(x: u32) -> u32 {
    x ^ x.rotate_left(9) ^ x.rotate_left(17)
}

fn p1(x: u32) -> u32 {
    x ^ x.rotate_left(15) ^ x.rotate_left(23)
}

fn get_u32_be(b: &[u8; 64], i: usize) -> u32 {
    let n: u32 = (b[i] as u32) << 24 | (b[i + 1] as u32) << 16 | (b[i + 2] as u32) << 8 | (b[i + 3] as u32) << 0;
    n
}

pub struct Sm3Hash {
    digest: [u32; 8],
    length: u64,
    unhandle_msg: Vec<u8>,
}

impl Sm3Hash {
    pub fn new(data: &str) -> Sm3Hash {
        let mut hash = Sm3Hash {
            digest: [0x7380166f, 0x4914b2b9, 0x172442d7, 0xda8a0600, 0xa96f30bc, 0x163138aa, 0xe38dee4d, 0xb0fb0e4e],
            length: (data.len() << 3) as u64,
            unhandle_msg: Vec::new(),
        };
        for i in data.bytes() {
            hash.unhandle_msg.push(i);
        }
        hash
    }

    pub fn get_hash(&mut self) -> [u8; 32] {
        let mut output: [u8; 32] = [0; 32];
        self.pad();
        let len = self.unhandle_msg.len();
        let mut count: usize = 0;
        let mut buffer: [u8; 64] = [0; 64];


        while count * 64 != len {
            for i in (count * 64)..(count * 64 + 64) {
                buffer[i - count * 64] = self.unhandle_msg[i];
            }
            self.update(&buffer);
            count += 1;
        }
        for i in 0..8 {
            output[i * 4] = (self.digest[i] >> 24) as u8;
            output[i * 4 + 1] = (self.digest[i] >> 16) as u8;
            output[i * 4 + 2] = (self.digest[i] >> 8) as u8;
            output[i * 4 + 3] = (self.digest[i] >> 0) as u8;
        }
        output
    }

    fn pad(&mut self) {
        self.unhandle_msg.push(0x80);
        let blocksize = 64;
        while self.unhandle_msg.len() % blocksize != 56 {
            self.unhandle_msg.push(0x00);
        }

        self.unhandle_msg.push((self.length >> 56 & 0xff) as u8);
        self.unhandle_msg.push((self.length >> 48 & 0xff) as u8);
        self.unhandle_msg.push((self.length >> 40 & 0xff) as u8);
        self.unhandle_msg.push((self.length >> 32 & 0xff) as u8);
        self.unhandle_msg.push((self.length >> 24 & 0xff) as u8);
        self.unhandle_msg.push((self.length >> 16 & 0xff) as u8);
        self.unhandle_msg.push((self.length >> 8 & 0xff) as u8);
        self.unhandle_msg.push((self.length >> 0 & 0xff) as u8);

        if self.unhandle_msg.len() % 64 != 0 {
            panic!("-------SM3 Pad: error msgLen ------");
        }
    }

    fn update(&mut self, buffer: &[u8; 64]) {
        //get expend
        let mut w: [u32; 68] = [0; 68];
        let mut w1: [u32; 64] = [0; 64];

        for i in 0..16 {
            w[i] = get_u32_be(&buffer, i * 4);
        }

        for i in 16..68 {
            w[i] = p1(w[i - 16] ^ w[i - 9] ^ w[i - 3].rotate_left(15)) ^ w[i - 13].rotate_left(7) ^ w[i - 6];
        }

        for i in 0..64 {
            w1[i] = w[i] ^ w[i + 4];
        }


        let mut a = self.digest[0] as u32;
        let mut b = self.digest[1] as u32;
        let mut c = self.digest[2] as u32;
        let mut d = self.digest[3] as u32;
        let mut e = self.digest[4] as u32;
        let mut f = self.digest[5] as u32;
        let mut g = self.digest[6] as u32;
        let mut h = self.digest[7] as u32;
        let mut ss1: u32;
        let mut ss2: u32;
        let mut tt1: u32;
        let mut tt2: u32;

        for i in 0..16 {
            ss1 = a.rotate_left(12).wrapping_add(e).wrapping_add(0x79cc4519u32.rotate_left(i as u32)).rotate_left(7);
            ss2 = ss1 ^ a.rotate_left(12);
            tt1 = ff0(a, b, c).wrapping_add(d).wrapping_add(ss2).wrapping_add(w1[i]);
            tt2 = gg0(e, f, g).wrapping_add(h).wrapping_add(ss1).wrapping_add(w[i]);
            d = c;
            c = b.rotate_left(9);
            b = a;
            a = tt1;
            h = g;
            g = f.rotate_left(19);
            f = e;
            e = p0(tt2);

//            println!("{} {:8x} {:8x} {:8x} {:8x} {:8x} {:8x} {:8x} {:8x} ", i, a, b, c, d, e, f, g, h);
        }

        for i in 16..64 {
            ss1 = a.rotate_left(12).wrapping_add(e).wrapping_add(0x7a879d8au32.rotate_left(i as u32)).rotate_left(7);
            ss2 = ss1 ^ a.rotate_left(12);
            tt1 = ff1(a, b, c).wrapping_add(d).wrapping_add(ss2).wrapping_add(w1[i]);
            tt2 = gg1(e, f, g).wrapping_add(h).wrapping_add(ss1).wrapping_add(w[i]);
            d = c;
            c = b.rotate_left(9);
            b = a;
            a = tt1;
            h = g;
            g = f.rotate_left(19);
            f = e;
            e = p0(tt2);
        }

        self.digest[0] = a ^ self.digest[0];
        self.digest[1] = b ^ self.digest[1];
        self.digest[2] = c ^ self.digest[2];
        self.digest[3] = d ^ self.digest[3];
        self.digest[4] = e ^ self.digest[4];
        self.digest[5] = f ^ self.digest[5];
        self.digest[6] = g ^ self.digest[6];
        self.digest[7] = h ^ self.digest[7];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lets_hash() {
        let s = "abc";
//        let s = "abcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcd";
        let mut sm3 = Sm3Hash::new(s);

        let hash = sm3.get_hash();

//        for i in hash.iter() {
//            print!("{:x} ", i);
//        }
//        println!();

        let standrad_hash: [u8; 32] = [
            0x66, 0xc7, 0xf0, 0xf4, 0x62, 0xee, 0xed, 0xd9,
            0xd1, 0xf2, 0xd4, 0x6b, 0xdc, 0x10, 0xe4, 0xe2,
            0x41, 0x67, 0xc4, 0x87, 0x5c, 0xf2, 0xf7, 0xa2,
            0x29, 0x7d, 0xa0, 0x2b, 0x8f, 0x4b, 0xa8, 0xe0
        ];

        for i in 0..32 {
            assert_eq!(standrad_hash[i], hash[i]);
        }
    }
}