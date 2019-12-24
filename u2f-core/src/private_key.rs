use openssl::ec::EcKey;
use openssl::pkey::Private;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Debug};
use std::mem;
use std::result::Result;

use serde_base64::{from_base64, to_base64};

pub struct PrivateKey(pub(crate) EcKey<Private>);

impl PrivateKey {
    pub fn from_pem(pem: &str) -> PrivateKey {
        PrivateKey(EcKey::private_key_from_pem(pem.as_bytes()).unwrap())
    }
}

impl Clone for PrivateKey {
    fn clone(&self) -> PrivateKey {
        PrivateKey(self.0.to_owned())
    }
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}

impl Debug for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // println!("{} bytes of T:", ::std::mem::size_of::<PrivateKey>());
        // let view = &self as *const _ as *const u8;
        // for i in 0..::std::mem::size_of::<PrivateKey>() as u8 {
        //     print!("{:02x} ", unsafe { *view.offset(i) });
        // }
        let bytes: &[u8] = unsafe { any_as_u8_slice(&self) };
        write!(f, "Private Key: {:?}", bytes)
    }
}

impl Serialize for PrivateKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        PrivateKeyAsPEM::from_key(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PrivateKey {
    fn deserialize<D>(deserializer: D) -> Result<PrivateKey, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(PrivateKeyAsPEM::deserialize(deserializer)?.as_key())
    }
}

struct PrivateKeyAsPEM(Vec<u8>);

impl PrivateKeyAsPEM {
    fn as_key(&self) -> PrivateKey {
        PrivateKey(EcKey::private_key_from_pem(&self.0).unwrap())
    }

    fn from_key(key: &PrivateKey) -> PrivateKeyAsPEM {
        PrivateKeyAsPEM(key.0.private_key_to_pem().unwrap())
    }
}

impl Serialize for PrivateKeyAsPEM {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        to_base64(&self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for PrivateKeyAsPEM {
    fn deserialize<D>(deserializer: D) -> Result<PrivateKeyAsPEM, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(PrivateKeyAsPEM(from_base64(deserializer)?))
    }
}
