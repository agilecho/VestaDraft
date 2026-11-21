#[derive(Clone, Debug)]
pub struct Ids {
    seed: [u8; 16],
    counter: u64,
}
impl Ids {
    pub fn new(seed: [u8; 16]) -> Self {
        Self { seed, counter: 0 }
    }
    pub fn fork(&mut self) -> Self {
        let s = self.next_id();
        let mut seed = [0; 16];
        for (i, b) in seed.iter_mut().enumerate() {
            *b = u8::from_str_radix(&s[2 * i..2 * i + 2], 16).expect("generated hex");
        }
        Self::new(seed)
    }
    pub fn next_id(&mut self) -> String {
        self.counter += 1;
        crate::digest(&[self.seed.as_slice(), &self.counter.to_be_bytes()].concat())[..32].into()
    }
}
