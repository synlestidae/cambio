use base64::{encode, decode};
use crypto::digest::Digest;
use crypto;
use db::{TryFromRow, TryFromRowError};
use domain::{Id, DecryptError};
use openssl::aes;
use openssl::symm;
use postgres;
use rand::{OsRng, Rng};
use rand;
use std::iter;
use std;

#[Derive(Debug, Clone, TryFromRow)]
pub struct EthereumAccountDetails {
    address: String,
    encrypted_private_key_base64: String,
    private_key_sha256_hash: String,
    iv_base64: String
}

impl EthereumAccountDetails {
    pub fn new(address: &str, private_key: String, password: String) -> Self {
        let mut hasher = crypto::sha2::Sha256::new();
        hasher.input(&private_key.as_bytes());
        let mut iv: Vec<u8> = random_vec(256);
        let encrypted_private_key = encrypt_string(&password, &private_key, &mut iv);
        let encrypted_private_key_base64 = encode(&encrypted_private_key);

        drop(password);
        drop(private_key);

        Self {
            address: address.to_owned(),
            encrypted_private_key_base64: encrypted_private_key_base64,
            private_key_sha256_hash: hasher.result_str(),
            iv_base64: encode(&iv)
        }
    }

    pub fn decrypt_private_key(&self, password: String) -> Result<String, DecryptError> {
        let private_key_bytes = decode(&self.encrypted_private_key_base64).unwrap();
        let mut iv = decode(&self.iv_base64).unwrap();
        let output = decrypt_string(&password, &private_key_bytes, &mut iv).unwrap(); 
        let mut hasher = crypto::sha2::Sha256::new();
        hasher.input(&output);
        if self.private_key_sha256_hash != hasher.result_str() {
            return Err(DecryptError::DecryptedDataHashError);
        }
        String::from_utf8(output).map_err(|e| DecryptError::FromUtf8Error(e))
    }
}

const SALT_STR: &'static str = "FADC36BDDC51696E57FC1FE94A115";
const GEN_KEY_ITER_COUNT: u32 = 8;

fn get_key(password: &str) -> Vec<u8> {
    let mut hasher = crypto::sha2::Sha256::new();
    let mut crypto_key_bytes = vec![0; 32];
    hasher.input_str(password);
    hasher.result(&mut crypto_key_bytes);
    let mut crypto_key = crypto::poly1305::Poly1305::new(&crypto_key_bytes);
    let salt_vec = SALT_STR.as_bytes(); 
    let mut output = vec![0; 32];
    crypto::pbkdf2::pbkdf2(&mut crypto_key, &salt_vec, GEN_KEY_ITER_COUNT, &mut output);
    output
}

fn encrypt_string(password: &str, string: &str, iv: &[u8]) -> Vec<u8> {
    let mut random_vec: Vec<u8> = iv.iter().map(|&x| x).collect();
    let password_key = get_key(password);
    let key = aes::AesKey::new_encrypt(&password_key).unwrap();
    let string_bytes = string.as_bytes();
    let mut output = vec![0u8; string_bytes.len()];
    aes::aes_ige(&string_bytes, &mut output, &key, &mut random_vec, symm::Mode::Encrypt);
    output
}

fn decrypt_string(password: &str, string_bytes: &[u8], iv: &[u8]) -> Result<Vec<u8>, DecryptError> {
    let mut random_vec: Vec<u8> = iv.iter().map(|&x| x).collect();
    let password_key = get_key(password);
    let key = aes::AesKey::new_decrypt(&password_key).unwrap();
    let mut output = vec![0u8; string_bytes.len()];
    aes::aes_ige(&string_bytes, &mut output, &key, &mut random_vec, symm::Mode::Decrypt);
    Ok(output)
}

fn random_vec(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut vec: Vec<u8> = Vec::new();
    for i in 0..length {
        vec.push(rng.gen());
    }
    vec
}

fn random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut char_vec: Vec<char> = Vec::new();
    for i in 0..length {
        char_vec.push(rng.gen());
    }
    char_vec.iter().collect::<String>()
}
