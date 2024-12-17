use bytes::{Bytes, BytesMut};
use core::str;
use std::fmt::Display;

/// Utf8 payload.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Utf8Bytes(Bytes);

impl Utf8Bytes {
    /// Creates from a static str.
    #[inline]
    pub const fn from_static(str: &'static str) -> Self {
        Self(Bytes::from_static(str.as_bytes()))
    }

    /// Returns as a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        // safety: is valid uft8
        unsafe { str::from_utf8_unchecked(&self.0) }
    }
}

impl std::ops::Deref for Utf8Bytes {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Display for Utf8Bytes {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<Bytes> for Utf8Bytes {
    type Error = str::Utf8Error;

    #[inline]
    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        str::from_utf8(&bytes)?;
        Ok(Self(bytes))
    }
}

impl TryFrom<BytesMut> for Utf8Bytes {
    type Error = str::Utf8Error;

    #[inline]
    fn try_from(bytes: BytesMut) -> Result<Self, Self::Error> {
        bytes.freeze().try_into()
    }
}

impl TryFrom<Vec<u8>> for Utf8Bytes {
    type Error = str::Utf8Error;

    #[inline]
    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        Bytes::from(v).try_into()
    }
}

impl From<String> for Utf8Bytes {
    #[inline]
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

impl From<&str> for Utf8Bytes {
    #[inline]
    fn from(s: &str) -> Self {
        Self(Bytes::copy_from_slice(s.as_bytes()))
    }
}

impl From<&String> for Utf8Bytes {
    #[inline]
    fn from(s: &String) -> Self {
        s.as_str().into()
    }
}

impl From<Utf8Bytes> for Bytes {
    #[inline]
    fn from(Utf8Bytes(bytes): Utf8Bytes) -> Self {
        bytes
    }
}