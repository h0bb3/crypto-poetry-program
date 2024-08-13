use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;
use anchor_lang::solana_program::keccak::hash;

// List of categorized words for refrigerator poetry
const NOUNS: &[&str] = &[
    "sun", "moon", "earth", "star", "shadow", "dream", "love",
    "time", "life", "heart", "magic", "mystery", "whisper",
    "breeze", "embrace", "spark", "kiss", "laugh", "smile",
    "song", "dance", "light", "darkness", "ocean", "wind", "fire",
    "silence", "voice", "sky", "night", "day", "flower", "river",
    "storm", "forest", "block", "chain", "satellite", "meme", 
    "crypto", "defi", "depin", "lambo", "hodler", "miner", 
    "whale", "doge", "numbers", "shill", "bitcoin", "ethereum", "solana",
    "dao", "nft", "metaverse", "web3", "contract", "dapp", "wallet"
];

const VERBS: &[&str] = &[
    "shine", "glow", "dance", "sing", "whisper", "laugh",
    "embrace", "dream", "kiss", "smile", "spark", "breeze",
    "radiate", "soften", "tender", "burn", "shiver",
    "cradle", "echo", "blossom", "flutter", "soar", "rush", "hodl",
    "mine", "trade", "pump", "dump", "moon", "shill", "launch"
];

const ADJECTIVES: &[&str] = &[
    "radiant", "soft", "gentle", "eternal", "sweet", "golden",
    "beautiful", "mystical", "serene", "bright", "dark", "tender",
    "silent", "luminous", "wild", "calm", "enchanted", "majestic",
    "glorious", "peaceful", "vivid", "melancholic", "brilliant", 
    "vibrant", "decentralized", "hyper", "blockchained", "moonlit", 
    "crypto", "stellar"
];

const PREPOSITIONS: &[&str] = &[
    "under", "through", "with", "in", "at", "where"
];

const TEMPLATES: &[&str] = &[
    "The {noun} {verb} {adjective}",
    "{verb} like a {adjective} {noun}",
    "{preposition} the {noun}, we {verb}",
    "A {adjective} {noun} to {verb}",
    "{noun}, {noun}, and {adjective} {noun}",
    "{preposition} the {adjective} {noun}, {verb}",
    "{preposition} the {noun}, {verb} {adjective} {noun}",
    "{adjective} {noun} with {noun}",
    "{adjective} {noun} {verb}, {verb}",
    "{verb} at the {noun}, {adjective} {noun}"
];

pub struct RandomnessManager {
    seed: [u8; 32],
}

impl RandomnessManager {
    pub fn new(account_pubkey: &Pubkey) -> Self {
        let clock = Clock::get().unwrap();
        let mut seed = [0u8; 32];
        
        let slot_bytes = clock.slot.to_le_bytes();
        let timestamp_bytes = clock.unix_timestamp.to_le_bytes();
        
        for (i, (s, t)) in slot_bytes.iter().zip(timestamp_bytes.iter()).enumerate() {
            seed[i] = s ^ t; // Mix slot and timestamp
        }
        
        for (i, &b) in account_pubkey.to_bytes().iter().enumerate().take(32).skip(timestamp_bytes.len()) {
            seed[i] ^= b; // Incorporate the account public key
        }
        
        Self { seed: hash(&seed).to_bytes() }
    }

    pub fn next_index(&mut self, upper_bound: usize) -> usize {
        let hashed_seed = hash(&self.seed).to_bytes();
        self.seed.copy_from_slice(&hashed_seed); // Update the seed with the new hash value
        let index = (u32::from_le_bytes([hashed_seed[0], hashed_seed[1], hashed_seed[2], hashed_seed[3]]) as usize) % upper_bound;
        index
    }
}

pub fn generate_poem(account_pubkey: &Pubkey) -> String {
    let mut rng = RandomnessManager::new(account_pubkey);
    let num_lines = rng.next_index(4) + 2;

    let mut poem = String::new();
    for _ in 0..num_lines {
        let template_idx = rng.next_index(TEMPLATES.len());
        let template = TEMPLATES[template_idx];
        let line = fill_template(template, &mut rng);
        poem.push_str(&line);
        poem.push('\n');
    }
    poem
}

fn fill_template(template: &str, rng: &mut RandomnessManager) -> String {
    let mut line = template.to_string();

    for (placeholder, words) in [
        ("noun", NOUNS),
        ("verb", VERBS),
        ("adjective", ADJECTIVES),
        ("preposition", PREPOSITIONS),
    ] {
        line = replace_placeholder(&line, placeholder, words, rng);
    }

    line = adjust_a_an(&line);
    capitalize(&line)
}

fn replace_placeholder(template: &str, placeholder: &str, words: &[&str], rng: &mut RandomnessManager) -> String {
    let mut result = template.to_string();
    while let Some(start) = result.find(format!("{{{}}}", placeholder).as_str()) {
        let index = rng.next_index(words.len());
        result.replace_range(start..start + placeholder.len() + 2, words[index]);
    }
    result
}

fn adjust_a_an(line: &str) -> String {
    let mut result = String::new();
    let mut words = line.split_whitespace().peekable();

    while let Some(word) = words.next() {
        if word.eq_ignore_ascii_case("a") {
            if let Some(next_word) = words.peek() {
                if starts_with_vowel(next_word) {
                    result.push_str("an ");
                    continue;
                }
            }
        }
        result.push_str(word);
        result.push(' ');
    }
    result.trim_end().to_string()
}

fn starts_with_vowel(word: &str) -> bool {
    matches!(word.chars().next(), Some(c) if "aeiouAEIOU".contains(c))
}

fn capitalize(word: &str) -> String {
    let mut c = word.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
