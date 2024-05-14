use std::{
    fmt,
    hash::{DefaultHasher, Hash, Hasher},
    str::FromStr,
};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
pub use error_info_derive::ToErrorInfo;

pub struct ErrInfo<T> {
    pub app_code: T,
    pub code: &'static str,
    pub hash: String,
    pub client_msg: &'static str,
    pub server_msg: String,
}
pub trait ToErrorInfo {
    type T: FromStr;
    fn to_error_info(&self) -> ErrInfo<Self::T>;
}
impl<T> ErrInfo<T>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
   pub fn new(
        app_code: &str,
        code: &'static str,
        client_msg: &'static str,
        server_msg: impl fmt::Display,
    ) -> Self {
        let server_msg = server_msg.to_string();
        let app_code = T::from_str(&app_code).expect("Can not parse app_code");
        let mut hasher = DefaultHasher::default();
        server_msg.hash(&mut hasher);
        let hash = hasher.finish();
        Self {
            app_code,
            code,
            hash: URL_SAFE_NO_PAD.encode(hash.to_be_bytes()),
            client_msg,
            server_msg: server_msg.to_string(),
        }
    }
}
impl<T> ErrInfo<T> {
    pub fn client_msg(&self) -> &str {
        if self.client_msg.is_empty() {
            &self.server_msg
        } else {
            self.client_msg
        }
    }
}
impl<T> fmt::Display for ErrInfo<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}-{}] {}", self.code, self.hash, self.client_msg())
    }
}

impl<T> fmt::Debug for ErrInfo<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}-{}] {}", self.code, self.hash, self.server_msg)
    }
}
