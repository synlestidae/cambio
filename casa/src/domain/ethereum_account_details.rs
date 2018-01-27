use domain::{Id, DecryptError};
use db::{TryFromRow, TryFromRowError};
use postgres;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use crypto::aes::{self, KeySize};
use crypto::buffer::RefReadBuffer;
use crypto::symmetriccipher::SynchronousStreamCipher;
use crypto::{ symmetriccipher, buffer, blockmodes };
use crypto::symmetriccipher::Decryptor;
use crypto::buffer::{ ReadBuffer, WriteBuffer };
use crypto::blockmodes::PaddingProcessor;
use crypto::symmetriccipher::BlockDecryptor;
use crypto::blockmodes::EcbDecryptor;
use rand::{OsRng, Rng};
use std::iter;
use std;
use base64::{encode, decode};

#[Derive(Debug, Clone, TryFromRow)]
pub struct EthereumAccountDetails {
    address: String,
    encrypted_private_key_base64: String,
    private_key_sha256_hash: String
}

impl EthereumAccountDetails {
    pub fn new(address: &str, private_key: String, password: String) -> Self {
        let mut sha = Sha256::new();
        let mut gen = OsRng::new().expect("Failed to get OS random generator");
        sha.input_str(&private_key);
        let hash = sha.result_str();

        let encrypted_private_key = encrypt_string(&password, &private_key);
        println!("Encoding {:?}", encrypted_private_key);
        let encrypted_password_base64 = (&encrypted_private_key);


        drop(password);
        drop(private_key);

        Self {
            address: address.to_owned(),
            encrypted_private_key_base64: encode(encrypted_password_base64),
            private_key_sha256_hash: hash
        }
    }

    pub fn decrypt_private_key(&self, password: String) -> Result<String, DecryptError> {
        let mut sha = Sha256::new();
        let data = decode(&self.encrypted_private_key_base64).unwrap();
        println!("Decoding {:?}", data);
        let private_key_string: String = try!(decrypt_string(&password, data));
        sha.input_str(&private_key_string);
        let hash = sha.result_str();
        if self.private_key_sha256_hash != hash {
            return Err(DecryptError::DecryptedDataHashError);
        }
        Ok(private_key_string.to_string())
    }
}

fn encrypt_string(password: &str, string: &str) -> Vec<u8> {
        let mut cipher = aes::ecb_encryptor(KeySize::KeySize128, 
            password.as_bytes(), 
            blockmodes::PkcsPadding);
        let mut buffer_vec = Vec::new();
        buffer_vec.resize(512, 0);
        println!("Plain! {:?}", string.as_bytes());
        let mut read_buffer = buffer::RefReadBuffer::new(&string.as_bytes());
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer_vec);
        let mut final_result = Vec::new();

        loop {
            let result = cipher.encrypt(&mut read_buffer, 
                &mut write_buffer, false).expect("Bad read read");
            let mut result_reader = write_buffer.take_read_buffer();
            let result_bytes = result_reader.take_remaining();
            for &b in result_bytes.iter() {
                final_result.push(b);
            }
            match result {
                buffer::BufferResult::BufferUnderflow => {
                    break;
                },
                buffer::BufferResult::BufferOverflow => { 
                }
            }
        }

        final_result
}

fn decrypt_string(password: &str, mut data: Vec<u8>) -> Result<String, DecryptError> {
    let mut decryptor = aes::ecb_decryptor(
            aes::KeySize::KeySize256,
            password.as_bytes(),
            blockmodes::PkcsPadding);
    let mut buffer_vec = Vec::new();
    buffer_vec.resize(512, 0);
    let mut read_buffer = buffer::RefReadBuffer::new(&mut data);
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer_vec);
    let mut final_result = Vec::<u8>::new();

    loop {
        let result = decryptor.decrypt(&mut read_buffer, 
            &mut write_buffer, false).expect("Bad read read");
        let mut result_reader = write_buffer.take_read_buffer();
        let result_bytes = result_reader.take_remaining();
        for &b in result_bytes.iter() {
            final_result.push(b);
        }
        match result {
            buffer::BufferResult::BufferUnderflow => {
                break;
            },
            buffer::BufferResult::BufferOverflow => { 
            }
        }
    }

    println!("Plain! {:?}", final_result);

    String::from_utf8(final_result).map_err(|error| DecryptError::FromUtf8Error(error))
}
