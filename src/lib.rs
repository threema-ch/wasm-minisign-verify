extern crate log;

mod utils;

use cfg_if::cfg_if;
use log::Level;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

/// Initialize the minisign-verify library.
///
/// If the `log_level` argument is supplied, the console logger is
/// initialized. Valid log levels: `trace`, `debug`, `info`, `warn` or
/// `error`.
#[wasm_bindgen(js_name = setupLogging)]
pub fn setup_logging(log_level: Option<String>) {
    utils::set_panic_hook();

    // Set log level
    if let Some(level) = log_level {
        match &*level {
            "trace" => utils::init_log(Level::Trace),
            "debug" => utils::init_log(Level::Debug),
            "info" => utils::init_log(Level::Info),
            "warn" => utils::init_log(Level::Warn),
            "error" => utils::init_log(Level::Error),
            other => {
                utils::init_log(Level::Warn);
                log::warn!("init: Invalid log level: {}", other);
            }
        }
    }
    log::trace!("minisign-verify: Initialized");
}

#[wasm_bindgen]
pub struct PublicKey(minisign_verify::PublicKey);

#[wasm_bindgen]
impl PublicKey {
    /// Create a Minisign public key from a string, as in the `minisign.pub` file
    pub fn decode(lines_str: &str) -> PublicKey {
        match minisign_verify::PublicKey::decode(lines_str) {
            Ok(pk) => PublicKey(pk),
            Err(e) => wasm_bindgen::throw_str(&format!("Decoding public key failed: {}", e)),
        }
    }

    /// Verify that `signature` is a valid signature for `bin` using this
    /// public key.
    ///
    /// If the verification succeeds, this function returns `true`. If the
    /// verification fails, an exception is thrown.
    pub fn verify(&self, bin: &[u8], signature: &Signature) -> bool {
        match self.0.verify(bin, &signature.0, true) {
            Ok(()) => true,
            Err(e) => wasm_bindgen::throw_str(&format!("Signature verification failed: {}", e))
        }
    }
}

#[wasm_bindgen]
pub struct Signature(minisign_verify::Signature);

#[wasm_bindgen]
impl Signature {
    /// Create a Minisign signature from a string
    pub fn decode(lines_str: &str) -> Signature {
        match minisign_verify::Signature::decode(lines_str) {
            Ok(pk) => Signature(pk),
            Err(e) => wasm_bindgen::throw_str(&format!("Decoding signature failed: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn verify_signature() {
        let sig: Signature = Signature::decode(concat!(
            "untrusted comment: signature from minisign secret key\n",
            "RWQzRRtiOy/fYEU/vGHUEfBg+lSmrdpViX3l9fX1Ps6FMBrBcsMw9uxsLPFr9pAMdKy1NVEX3MsHsuCKlSVNYc4C5/pCnU/Kugk=\n",
            "trusted comment: timestamp:1634045550	file:test.txt\n",
            "zEHzYWS0L/lFlN3hfMdAJA0MsVfazBXbwSw9XihxQ0msFQPlC30F6Ajvxi67KEFNd1GUhdi3DcslssTW8MUECQ==",
        ));

        let pk: PublicKey = PublicKey::decode(concat!(
            "untrusted comment: minisign public key 60DF2F3B621B4533\n",
            "RWQzRRtiOy/fYNCli5tW96CO6R+FnO92LceeIoWlCLj+BTVe+6q8T69M",
        ));

        assert!(pk.verify(b"test\n", &sig));
    }
}
