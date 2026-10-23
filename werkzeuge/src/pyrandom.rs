//! Nachbau von Pythons `random.Random` (Mersenne Twister), damit die Pixel-Texturen
//! genau so aussehen wie die, die früher das Python-Skript erzeugt hat.

const N: usize = 624;
const M: usize = 397;

pub struct PyRandom {
    mt: [u32; N],
    idx: usize,
}

impl PyRandom {
    /// Wie `random.Random(seed)` mit einer kleinen, nicht negativen Ganzzahl.
    pub fn new(seed: u32) -> PyRandom {
        let mut r = PyRandom { mt: [0; N], idx: N };
        r.init_by_array(&[seed]);
        r
    }

    fn init_genrand(&mut self, s: u32) {
        self.mt[0] = s;
        for i in 1..N {
            let prev = self.mt[i - 1];
            self.mt[i] = 1_812_433_253u32
                .wrapping_mul(prev ^ (prev >> 30))
                .wrapping_add(i as u32);
        }
        self.idx = N;
    }

    fn init_by_array(&mut self, key: &[u32]) {
        self.init_genrand(19_650_218);
        let (mut i, mut j) = (1usize, 0usize);
        for _ in 0..N.max(key.len()) {
            let prev = self.mt[i - 1];
            self.mt[i] = (self.mt[i] ^ (prev ^ (prev >> 30)).wrapping_mul(1_664_525))
                .wrapping_add(key[j])
                .wrapping_add(j as u32);
            i += 1;
            j += 1;
            if i >= N {
                self.mt[0] = self.mt[N - 1];
                i = 1;
            }
            if j >= key.len() {
                j = 0;
            }
        }
        for _ in 0..N - 1 {
            let prev = self.mt[i - 1];
            self.mt[i] = (self.mt[i] ^ (prev ^ (prev >> 30)).wrapping_mul(1_566_083_941))
                .wrapping_sub(i as u32);
            i += 1;
            if i >= N {
                self.mt[0] = self.mt[N - 1];
                i = 1;
            }
        }
        self.mt[0] = 0x8000_0000;
    }

    fn genrand(&mut self) -> u32 {
        if self.idx >= N {
            for k in 0..N {
                let y = (self.mt[k] & 0x8000_0000) | (self.mt[(k + 1) % N] & 0x7fff_ffff);
                let mut v = self.mt[(k + M) % N] ^ (y >> 1);
                if y & 1 == 1 {
                    v ^= 0x9908_b0df;
                }
                self.mt[k] = v;
            }
            self.idx = 0;
        }
        let mut y = self.mt[self.idx];
        self.idx += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c_5680;
        y ^= (y << 15) & 0xefc6_0000;
        y ^ (y >> 18)
    }

    /// `getrandbits(k)` für 1 ≤ k ≤ 32
    fn getrandbits(&mut self, k: u32) -> u32 {
        self.genrand() >> (32 - k)
    }

    /// Pythons `_randbelow(n)`: 0 ≤ Ergebnis < n
    fn randbelow(&mut self, n: u32) -> u32 {
        // Auch bei n = 1 verbraucht Python eine Zufallszahl – deshalb kein Abkürzen
        if n == 0 {
            return 0;
        }
        let k = 32 - n.leading_zeros();
        loop {
            let r = self.getrandbits(k);
            if r < n {
                return r;
            }
        }
    }

    /// `_randbelow(n)` für andere Module (z. B. `choice` auf festen Listen).
    pub fn randbelow_pub(&mut self, n: usize) -> usize {
        self.randbelow(n as u32) as usize
    }

    /// `randrange(start, stop)`
    pub fn randrange(&mut self, start: i64, stop: i64) -> i64 {
        start + self.randbelow((stop - start).max(0) as u32) as i64
    }

    /// `randint(a, b)` (beide Grenzen inklusive)
    pub fn randint(&mut self, a: i64, b: i64) -> i64 {
        self.randrange(a, b + 1)
    }

    /// `choice(seq)` – `None` bei leerer Liste
    pub fn choice<'a, T>(&mut self, seq: &'a [T]) -> Option<&'a T> {
        if seq.is_empty() {
            return None;
        }
        seq.get(self.randbelow(seq.len() as u32) as usize)
    }

    /// `random()`: 0 ≤ Ergebnis < 1 (53 Bit wie in Python)
    pub fn random(&mut self) -> f64 {
        let a = (self.genrand() >> 5) as f64;
        let b = (self.genrand() >> 6) as f64;
        (a * 67_108_864.0 + b) * (1.0 / 9_007_199_254_740_992.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Vergleichswerte aus Python 3: `r = random.Random(11)`
    #[test]
    fn wie_python() {
        let mut r = PyRandom::new(11);
        let got: Vec<i64> = (0..5).map(|_| r.randrange(0, 16)).collect();
        assert_eq!(got, PY_RANDRANGE_16);
        let mut r = PyRandom::new(5);
        assert!((r.random() - PY_RANDOM_5).abs() < 1e-15);
        // choice aus einer Liste mit einem Element verbraucht trotzdem eine Zahl
        let mut r = PyRandom::new(7);
        let _ = r.choice(&[1]);
        assert_eq!(r.randrange(0, 1000), PY_NACH_EINER_WAHL);
    }

    // Aus Python 3 übernommen
    const PY_RANDRANGE_16: [i64; 5] = [14, 14, 14, 6, 5];
    const PY_RANDOM_5: f64 = 0.6229016948897019;
    const PY_NACH_EINER_WAHL: i64 = 970;
}
