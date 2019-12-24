use app_id::AppId;
use key_handle::KeyHandle;
use private_key::PrivateKey;

// A private key is generated per application
// This stores the AppID for indexing, and a private key for signing
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ApplicationKey {
    pub application: AppId,
    pub handle: KeyHandle,
    key: PrivateKey,
}

impl ApplicationKey {
    pub fn new(application: AppId, handle: KeyHandle, key: PrivateKey) -> ApplicationKey {
        ApplicationKey {
            application,
            handle,
            key,
        }
    }
    pub(crate) fn key(&self) -> &PrivateKey {
        &self.key
    }
}
