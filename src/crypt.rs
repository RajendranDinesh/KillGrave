pub mod cryption {
    use base64::prelude::*;

    pub fn encrypt_string(string: String) -> String {
        BASE64_STANDARD.encode(string)
    }
}