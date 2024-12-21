use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn build_char_pool(uppercase: bool, digits:bool, symbols: bool) -> String {
    let mut char_pool = String::from("abcdefghijklmnopqrstuvwxyz");

    if uppercase {
        char_pool.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }

    if digits {
        char_pool.push_str("0123456789");
    }

    if symbols {
        char_pool.push_str("!@#$%&&'()=~");
    }
    char_pool
}

pub fn generate_password(length: usize, char_pool: &str) -> String {
    let mut rng = thread_rng();
    let pool_chars: Vec<char> = char_pool.chars().collect();

    (0..length)
        .map(|_| {
            *pool_chars.choose(&mut rng).expect("Failed to generate password")
        })
        .collect::<String>()
}
