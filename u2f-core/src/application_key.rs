use client_lib::ecdsa::PrivateShare;

use app_id::AppId;
use key_handle::KeyHandle;

// A private key is generated per application
// This stores the AppID for indexing, and a private key for signing
#[derive(Serialize, Deserialize)]
pub struct ApplicationKey {
    pub application: AppId,
    pub handle: KeyHandle,
    key: PrivateShare,
}

impl ApplicationKey {
    pub fn new(application: AppId, handle: KeyHandle, key: PrivateShare) -> ApplicationKey {
        ApplicationKey {
            application,
            handle,
            key,
        }
    }
    pub(crate) fn key(&self) -> &PrivateShare {
        &self.key
    }
}

impl std::fmt::Debug for ApplicationKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl Clone for ApplicationKey {
    fn clone(&self) -> ApplicationKey {
        let ser = serde_json::to_string(&self).unwrap();
        let app_key = serde_json::from_str(&ser).unwrap();
        app_key
    }
}
