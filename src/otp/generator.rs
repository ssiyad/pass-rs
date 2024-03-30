use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::time::SystemTime;

pub struct Generator {
    secret: String,
    size: u8,
    step_size: u64,
}

impl Generator {
    /// Create a new Generator. Default size is 6 and step_size is 30.
    ///
    /// * `secret`:
    pub fn new(secret: String) -> Self {
        Self {
            secret,
            size: 6,
            step_size: 30,
        }
    }

    /// Set the size of the code.
    ///
    /// * `size`:
    fn with_size(mut self, size: u8) -> Self {
        self.size = size;
        self
    }

    /// Decode `secret`
    fn secret_base32(&self) -> Vec<u8> {
        base32::decode(base32::Alphabet::RFC4648 { padding: false }, &self.secret)
            .expect("Unable to decode secret")
    }

    /// Sign `secret` and `data` using HMAC-SHA1.
    ///
    /// * `data`:
    fn sign(&self, data: &[u8]) -> Vec<u8> {
        let secret = self.secret_base32();
        let mut mac = Hmac::<Sha1>::new_from_slice(&secret).unwrap();
        mac.update(data);
        mac.finalize().into_bytes().to_vec()
    }

    /// Get the time until the next refresh.
    ///
    /// * `time`:
    fn refresh_in(&self, time: u64) -> u64 {
        self.step_size - (time % self.step_size)
    }

    /// Get the time until the next refresh using the current time.
    pub fn refresh_current_in(&self) -> u64 {
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.refresh_in(time)
    }

    /// Generate a code using `time`.
    ///
    /// * `time`:
    fn generate(&self, time: u64) -> usize {
        let step = time / self.step_size;
        let signed = self.sign(&step.to_be_bytes());
        let offset = (signed[19] & 0xf) as usize;
        let truncated = signed[offset..=(offset + 3)].to_vec();
        let code_num = u32::from_be_bytes(truncated.try_into().unwrap()) & 0x7fffffff;
        let size_adjused = (code_num as u64) % 10_u64.pow(self.size as u32);
        size_adjused as usize
    }

    /// Generate a code using the current time.
    pub fn generate_current(&self) -> usize {
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.generate(time)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn known_value_1() {
        let otp_gen = Generator::new("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ".to_string()).with_size(8);
        let code = otp_gen.generate(59);
        assert_eq!(code, 94287082);
    }

    #[test]
    fn known_value_2() {
        let otp_gen = Generator::new("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ".to_string()).with_size(8);
        let code = otp_gen.generate(1111111109);
        assert_eq!(code, 7081804);
    }

    #[test]
    fn refresh_in_seconds_1() {
        let otp_gen = Generator::new("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ".to_string()).with_size(8);
        let refresh = otp_gen.refresh_in(59);
        assert_eq!(refresh, 1);
    }

    #[test]
    fn refresh_in_seconds_2() {
        let otp_gen = Generator::new("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ".to_string()).with_size(8);
        let refresh = otp_gen.refresh_in(12);
        assert_eq!(refresh, 18);
    }

    #[test]
    fn current_value() {
        let otp_gen = Generator::new("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ".to_string()).with_size(8);
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let code = otp_gen.generate(time);
        let code_current = otp_gen.generate_current();
        assert_eq!(code, code_current);
    }
}
