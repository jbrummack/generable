use std::{collections::HashMap, time::SystemTime};

use outlines_core::{index::Index, prelude::Vocabulary};

pub struct Sampler {
    idx: Index,
    lut: HashMap<u32, String>,
    state: u32,
}
fn llm_vocab() -> (Vocabulary, HashMap<u32, String>) {
    let vocab: Result<HashMap<String, u32>, _> = serde_json::from_str(include_str!("vocab.json"));
    let vocab = vocab.unwrap();

    let mut vocabulary = Vocabulary::new(151643);
    let mut lut = HashMap::new();
    for (k, v) in vocab {
        lut.insert(v, k.clone());
        vocabulary.try_insert(k, v).unwrap();
    }
    (vocabulary, lut)
}
fn ascii_vocab() -> (Vocabulary, HashMap<u32, String>) {
    let mut vocab = Vocabulary::new(128);
    let mut lut = HashMap::new();

    for i in 0u32..128 {
        let ch = char::from_u32(i).unwrap();
        let s = ch.to_string();
        vocab.try_insert(s.clone(), i).unwrap();
        lut.insert(i, s);
    }

    (vocab, lut)
}
impl Sampler {
    pub fn new(schema: serde_json::Value) -> Self {
        let (vocabulary, lut) = ascii_vocab();
        let regex = outlines_core::json_schema::regex_from_value(&schema, None, None).unwrap();
        let idx = Index::new(&regex, &vocabulary).unwrap();
        let state = idx.initial_state();
        Self { idx, state, lut }
    }
    pub fn next(&mut self) -> Option<String> {
        let pred = self.idx.allowed_tokens(&self.state)?;
        //println!("{pred:?}");
        let selection = select_random_ish(&pred);
        let next_token = self.idx.next_state(&self.state, &selection)?;
        self.state = next_token;
        self.lut.get(&selection).cloned()
    }
}

/*fn select_random_ish<T: Copy>(input: &Vec<T>) -> T {
    let now = SystemTime::now();
    let number = now.elapsed().unwrap().subsec_nanos() as usize;
    let access = number % input.len();
    input[access]
}
*/
use rand::{Rng, SeedableRng, rngs::StdRng};

fn select_random_ish<T: Copy>(input: &Vec<T>) -> T {
    let mut rng = StdRng::from_seed([128u8; 32]);
    //let mut rng = rand::SeedableRng::seed_from_u64(1337);
    let idx = rng.next_u64() as usize % input.len();
    input[idx]
}
