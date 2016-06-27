use argon2rs::{ Argon2, Variant, ParamErr };
use ::utils::Bytes;
use super::{ KeyDerive, Key };


/// Interactive Opslimit.
pub const OPSLIMIT_INTERACTIVE: u32 = 4;
/// Interactive Memlimit.
pub const MEMLIMIT_INTERACTIVE: u32 = 33554432;
/// Moderate Opslimit.
pub const OPSLIMIT_MODERATE: u32 = 6;
/// Moderate Memlimit.
pub const MEMLIMIT_MODERATE: u32 = 134217728;
/// Sensitive Opslimit.
pub const OPSLIMIT_SENSITIVE: u32 = 8;
/// Sensitive Memlimit.
pub const MEMLIMIT_SENSITIVE: u32 = 536870912;


/// Argon2i.
///
/// # Example(keyderive)
/// ```
/// use sarkara::pwhash::{ Argon2i, KeyDerive };
///
/// let (pass, salt) = ([0; 8], [0; 8]);
/// let key = Argon2i::new()
///     .derive(&pass, &salt)
///     .ok().unwrap();
/// assert!(key != pass[..]);
/// ```
///
/// # Example(pwhash)
/// ```
/// use sarkara::pwhash::{ Argon2i, KeyDerive };
///
/// let pass = [0; 8];
/// let key = Argon2i::new()
///     .with_size(16)
///     .pwhash(&pass)
///     .ok().unwrap();
/// assert_eq!(key.len(), 16);
/// ```
///
/// # Example(keyverify)
/// ```
/// use sarkara::pwhash::{ Argon2i, KeyDerive, KeyVerify };
///
/// let (pass, salt) = ([0; 8], [0; 8]);
/// let key = Argon2i::new()
///     .derive(&pass, &salt)
///     .ok().unwrap();
///
/// assert!(Argon2i::new().verify(&pass, &salt, &key).ok().unwrap());
/// ```
#[derive(Clone, Debug)]
pub struct Argon2i {
    /// key derive key. default empty.
    pub key: Bytes,
    /// associated data. default empty.
    pub aad: Bytes,
    /// output length. default `16`.
    pub outlen: usize,
    /// passes parameter. default `OPSLIMIT_INTERACTIVE`.
    pub passes: u32,
    /// lanes parameter. default `1`.
    pub lanes: u32,
    /// kib parameter. default `MEMLIMIT_INTERACTIVE / 1024`.
    pub kib: u32
}

impl Default for Argon2i {
    fn default() -> Argon2i {
        Argon2i {
            key: Bytes(Vec::new()),
            aad: Bytes(Vec::new()),
            outlen: 16,
            passes: OPSLIMIT_INTERACTIVE,
            lanes: 1,
            kib: MEMLIMIT_INTERACTIVE / 1024
        }
    }
}

impl Argon2i {
    /// Create a new Argon2i.
    pub fn new() -> Argon2i {
        Argon2i::default()
    }
}

impl KeyDerive for Argon2i {
    fn with_size(&mut self, len: usize) -> &mut Self {
        self.outlen = len;
        self
    }
    fn with_key(&mut self, key: &[u8]) -> &mut Self {
        self.key = Bytes::new(key);
        self
    }
    fn with_aad(&mut self, aad: &[u8]) -> &mut Self {
        self.aad = Bytes::new(aad);
        self
    }
    fn with_opslimit(&mut self, opslimit: u32) -> &mut Self {
        self.passes = opslimit;
        self
    }
    fn with_memlimit(&mut self, memlimit: u32) -> &mut Self {
        self.kib = memlimit / 1024;
        self
    }

    fn derive(&self, password: &[u8], salt: &[u8]) -> Result<Key, ParamErr> {
        let mut output = Bytes(vec![0; self.outlen]);
        Argon2::new(self.passes, self.lanes, self.kib, Variant::Argon2i)?
            .hash(&mut output, password, salt, &self.key, &self.aad);
        Ok(output)
    }
}
