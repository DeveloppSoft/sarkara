use failure::Fail;


pub mod sparx256colm0;


pub trait AeadCipher {
    const KEY_LENGTH: usize;
    const NONCE_LENGTH: usize;
    const TAG_LENGTH: usize;

    type Error: Fail;

    /// TODO should be `Self::KEY_LENGTH`
    fn new(key: &[u8]) -> Self;
    /// TODO should be `Self::NONCE_LENGTH`
    fn seal(&self, nonce: &[u8], aad: &[u8], input: &[u8], output: &mut [u8]) -> Result<(), Self::Error>;
    fn open(&self, nonce: &[u8], aad: &[u8], input: &[u8], output: &mut [u8]) -> Result<bool, Self::Error>;
}


/// TODO https://github.com/rust-lang/rust/issues/44265
pub trait OnlineAE<'a>: AeadCipher {
    type Encryption: Encryption<'a, Self::Error>;
    type Decryption: Decryption<'a, Self::Error>;

    /// TODO should be `Self::NONCE_LENGTH`
    fn encrypt(&'a self, nonce: &[u8], aad: &[u8]) -> Self::Encryption;
    /// TODO should be `Self::NONCE_LENGTH`
    fn decrypt(&'a self, nonce: &[u8], aad: &[u8]) -> Self::Decryption;
}

pub trait Encryption<'a, Error> {
    fn process<'b>(&mut self, input: &'b [u8], output: &mut [u8]) -> Result<(), &'b [u8]>;
    fn finalize(self, input: &[u8], output: &mut [u8]) -> Result<(), Error>;
}

pub trait Decryption<'a, Error> {
    fn process<'b>(&mut self, input: &'b [u8], output: &mut [u8]) -> Result<(), &'b [u8]>;
    fn finalize(self, input: &[u8], output: &mut [u8]) -> Result<bool, Error>;
}
