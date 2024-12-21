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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_char_pool_default() {
        assert_eq!(
            build_char_pool(false, false, false),
            "abcdefghijklmnopqrstuvwxyz"
        );
    }

    #[test]
    fn test_build_char_pool_with_uppercase() {
        assert_eq!(
            build_char_pool(true, false, false),
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        );
    }

    #[test]
    fn test_build_char_pool_with_digits() {
        assert_eq!(
            build_char_pool(false, true, false),
            "abcdefghijklmnopqrstuvwxyz0123456789"
        );
    }

    #[test]
    fn test_build_char_pool_with_symbols() {
        assert_eq!(
            build_char_pool(false, false, true),
            "abcdefghijklmnopqrstuvwxyz!@#$%&&'()=~"
        );
    }

    #[test]
    fn test_build_char_pool_all_option() {
        assert_eq!(
            build_char_pool(true, true, true),
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%&&'()=~"
        );
    }

    #[test]
    fn test_generate_password_length() {
        let char_pool = build_char_pool(false, false, false);
        let password = generate_password(16, &char_pool);
        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_generate_password_char_pool() {
        let char_pool = build_char_pool(false, false, false);
        let password = generate_password(16, &char_pool);
        assert_eq!(password.chars().all(|c| char_pool.contains(c)), true);
    }
}