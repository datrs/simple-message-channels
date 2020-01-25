use salsa20::stream_cipher::{NewStreamCipher, SyncStreamCipher};
use salsa20::XSalsa20;
use std::sync::{Arc, RwLock};

pub(crate) type SharedCipher = Arc<RwLock<Cipher>>;

type Key = [u8; 32];
pub struct Cipher {
    key: Key,
    cipher: Option<XSalsa20>,
}

impl Cipher {
    pub fn new(key: Key) -> Self {
        Self { key, cipher: None }
    }

    pub fn empty() -> Self {
        Self::new([0; 32])
    }

    pub fn initialize(&mut self, nonce: &[u8]) {
        self.cipher = XSalsa20::new_var(&self.key, &nonce).ok();
    }

    pub(crate) fn try_apply(&mut self, buffer: &mut [u8]) {
        if let Some(ref mut cipher) = &mut self.cipher {
            cipher
                .try_apply_keystream(buffer)
                .expect("inifite stream finished");
        }
    }
}
