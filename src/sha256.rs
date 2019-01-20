const H: [u32; 8] = [
    0x6a09e667,
    0xbb67ae85,
    0x3c6ef372,
    0xa54ff53a,
    0x510e527f,
    0x9b05688c,
    0x1f83d9ab,
    0x5be0cd19
];


const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
];

const MAX_U32: u64 = 0x100000000;

// 循环左移：(val >> (N - n) | (val << n))
// 循环右移：(val << (32 - n) | (val >> n))

pub fn pre_process() {
    let a = "abc".chars();
    println!("ascii: {:?}", "abc".as_bytes());
    // a.chars().iter().fold()
}

#[inline] // rotate_right
pub fn rot_r(x: u32, n: u32) -> u32 {
    let mut res: u32 = x >> n;
    res |= x << (32 - n);
    res
}

#[inline] // rotate_left
pub fn rot_l(x: u32, n: u32) -> u32 {
    let mut res: u32 = x << n;
    res |= x >> (32 - n);
    res
}

#[inline] // choose
pub fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
    // (x & y) ^ (~x & z)
}

#[inline] // majority
pub fn ma(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

#[inline] // sigma 0
pub fn sum_0(x: u32) -> u32 {
    rot_r(x, 2) ^ rot_r(x, 13) ^ rot_r(x, 22)
}

#[inline] // sigma 1
pub fn sum_1(x: u32) -> u32 {
    rot_r(x, 6) ^ rot_r(x, 11) ^ rot_r(x, 25)
}

#[inline] // ep0, lots of rotaing
pub fn delta_0(x: u32) -> u32 {
    rot_r(x, 7) ^ rot_r(x, 18) ^ (x >> 3)
}

#[inline] // ep1, lots of rotaing
pub fn delta_1(x: u32) -> u32 {
    rot_r(x, 17) ^ rot_r(x, 19) ^ (x >> 10)
}


pub fn digest() {
    let target = "abc";
    let t: [u32; 16] = [
        0x61626380, 0x00000000, 0x00000000, 0x00000000, 
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 
        0x00000000, 0x00000000, 0x00000000, 0x00000018
    ];
    
    let mut prev: [u32; 64] = [0; 64];
    prev[..16].clone_from_slice(&t[..16]);
    
    for i in 16..64 {
        println!("index: {}", i);
        prev[i] = delta_1(prev[i - 2]) + prev[i - 7] + delta_0(prev[i - 15]) + prev[i - 16];
    }
    
    // initialzed value
    let mut a = H[0];
    let mut b = H[1];
    let mut c = H[2];
    let mut d = H[3];
    let mut e = H[4];
    let mut f = H[5];
    let mut g = H[6];
    let mut h = H[7];
    
    
    for i in 0..64 {
        let z_d = h + ch(e, f, g) + delta_1(e) + prev[i] + K[i];
        let z_a = ma(a, b, c) + delta_0(a);
        let z2 = 0;
        
        h = g;
        g = f;
        f = e;
        e = d + z_d;
        d = c;
        c = b;
        b = a;
        a = z_a + z_d;
    }
    
    let mut h1: [u32; 8] = [0; 8];
    
    h1[0] = H[0] + a;
    h1[1] = H[1] + b;
    h1[2] = H[2] + c;
    h1[3] = H[3] + d;
    h1[4] = H[4] + e;
    h1[5] = H[5] + f;
    h1[6] = H[6] + g;
    h1[7] = H[7] + h;
    
    println!("hash: {:?}", h1);
}