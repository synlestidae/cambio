use std::string::FromUtf8Error;
use crypto::symmetriccipher::SymmetricCipherError;

pub enum DecryptError {
    FromUtf8Error(FromUtf8Error),
    SymmetricCipherError(SymmetricCipherError),
    DecryptedDataHashError
}
