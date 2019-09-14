use std::collections::HashMap;
use std::io;
use std::io::ErrorKind;
use std::time::{SystemTime, UNIX_EPOCH};

use futures::Future;
use futures::future;
use secret_service::{Collection, EncryptionType, Item, SecretService};
use serde_json;
use u2f_core::{AppId, ApplicationKey, Counter, KeyHandle, SecretStore, try_reverse_app_id};

pub struct SecretServiceStore {
    service: SecretService,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Secret {
    application_key: ApplicationKey,
    counter: Counter,
}

impl SecretServiceStore {
    pub fn new() -> io::Result<SecretServiceStore> {
        let service = SecretService::new(EncryptionType::Dh).map_err(|error| io::Error::new(ErrorKind::Other, "get_default_collection"))?;
        Ok(SecretServiceStore {
            service,
        })
    }

    fn try_add_application_key(
        &self,
        key: &ApplicationKey,
    ) -> io::Result<()> {
        let collection = self.service.get_default_collection().map_err(|error| io::Error::new(ErrorKind::Other, "get_default_collection"))?;
        collection.ensure_unlocked().map_err(|error| io::Error::new(ErrorKind::Other, "to_vec"))?;
        let secret = serde_json::to_string(&Secret {
            application_key: key.clone(),
            counter: 0,
        }).map_err(|error| io::Error::new(ErrorKind::Other, error))?;
        let attributes = registration_attributes(&key.application, &key.handle);
        let attributes = attributes.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let label = match try_reverse_app_id(&key.application) {
            Some(app_id) => format!("Universal 2nd Factor token for {}", app_id),
            None => format!("Universal 2nd Factor token for {}", key.application.to_base64()),
        };
        let content_type = "application/json";
        let item = collection.create_item(&label, attributes, secret.as_bytes(), false, content_type).map_err(|error| io::Error::new(ErrorKind::Other, "create_item"))?;
        Ok(())
    }

    fn try_increment_counter(
        &self,
        app_id: &AppId,
        handle: &KeyHandle,
    ) -> io::Result<Counter> {
        let collection = self.service.get_default_collection().map_err(|error| io::Error::new(ErrorKind::Other, "get_default_collection"))?;
        let option = find_item(&collection, app_id, handle).map_err(|error| io::Error::new(ErrorKind::Other, "find_item"))?;
        if option.is_none() {
            return Err(io::Error::new(ErrorKind::Other, "not found"));
        }
        let item = option.unwrap();
        let secret_bytes = item.get_secret().map_err(|error| io::Error::new(ErrorKind::Other, "get_secret"))?;
        let mut secret: Secret = serde_json::from_slice(&secret_bytes).map_err(|error| io::Error::new(ErrorKind::Other, "from_slice"))?;

        secret.counter += 1;

        let secret_string = serde_json::to_string(&secret).map_err(|error| io::Error::new(ErrorKind::Other, error))?;
        item.set_secret(secret_string.as_bytes(), "application/json").map_err(|error| io::Error::new(ErrorKind::Other, "get_attributes"))?;

        let attributes = item.get_attributes().map_err(|error| io::Error::new(ErrorKind::Other, error.to_string()))?;
        let mut attributes: HashMap<_, _> = attributes.into_iter().collect();
        attributes.entry("times_used".to_string()).and_modify(|value| {
            let count = value.parse::<u32>().unwrap_or(0);
            *value = (count + 1).to_string();
        }).or_insert(0.to_string());
        let mut attributes: Vec<(&str, &str)> = attributes.iter().map(|(key, value)| (key.as_str(), value.as_str())).collect();
        attributes.sort_by_cached_key(|(key, _)| key.to_owned());
        item.set_attributes(attributes).map_err(|error| io::Error::new(ErrorKind::Other, "get_attributes"))?;

        let label = match try_reverse_app_id(app_id) {
            Some(app_id) => format!("Universal 2nd Factor token for {}", app_id),
            None => format!("Universal 2nd Factor token for {}", app_id.to_base64()),
        };
        item.set_label(&label).map_err(|error| io::Error::new(ErrorKind::Other, error.to_string()))?;

        Ok(secret.counter)
    }


    fn try_retrieve_application_key(
        &self,
        app_id: &AppId,
        handle: &KeyHandle,
    ) -> io::Result<Option<ApplicationKey>> {
        let collection = self.service.get_default_collection().map_err(|error| io::Error::new(ErrorKind::Other, error.to_string()))?;
        let option = find_item(&collection, app_id, handle).map_err(|error| io::Error::new(ErrorKind::Other, error.to_string()))?;
        if option.is_none() {
            return Ok(None);
        }
        let item = option.unwrap();
        let secret_bytes = item.get_secret().map_err(|error| io::Error::new(ErrorKind::Other, error.to_string()))?;
        let secret: Secret = serde_json::from_slice(&secret_bytes).map_err(|error| io::Error::new(ErrorKind::Other, error))?;
        Ok(Some(secret.application_key))
    }
}


impl SecretStore for SecretServiceStore {
    fn add_application_key(
        &self,
        key: &ApplicationKey,
    ) -> Box<dyn Future<Item=(), Error=io::Error>> {
        match self.try_add_application_key(key) {
            Ok(option) => Box::new(future::ok(option)),
            Err(err) => Box::new(future::err(err)),
        }
    }

    fn get_and_increment_counter(
        &self,
        application: &AppId,
        handle: &KeyHandle,
    ) -> Box<dyn Future<Item=Counter, Error=io::Error>> {
        match self.try_increment_counter(application, handle) {
            Ok(option) => Box::new(future::ok(option)),
            Err(err) => Box::new(future::err(err)),
        }
    }

    fn retrieve_application_key(
        &self,
        application: &AppId,
        handle: &KeyHandle,
    ) -> Box<dyn Future<Item=Option<ApplicationKey>, Error=io::Error>> {
        match self.try_retrieve_application_key(application, handle) {
            Ok(option) => Box::new(future::ok(option)),
            Err(err) => Box::new(future::err(err)),
        }
    }
}


fn search_attributes(app_id: &AppId, handle: &KeyHandle) -> Vec<(&'static str, String)> {
    vec![
        ("application", "com.github.danstiner.rust-u2f".to_string()),
        ("u2f_app_id_hash", app_id.to_base64()),
        ("u2f_key_handle", handle.to_base64()),
        ("xdg:schem", "com.github.danstiner.rust-u2f".to_string())
    ]
}

fn registration_attributes(app_id: &AppId, handle: &KeyHandle) -> Vec<(&'static str, String)> {
    let mut attributes = search_attributes(app_id, handle);
    attributes.push(("times_used", 0.to_string()));

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("time moved backwards");
    attributes.push(("date_registered", since_the_epoch.as_secs().to_string()));

    match try_reverse_app_id(app_id) {
        Some(id) => attributes.push(("u2f_app_id", id)),
        None => {}
    };

    attributes
}

fn find_item<'a>(
    collection: &'a Collection<'a>,
    app_id: &AppId,
    handle: &KeyHandle,
) -> io::Result<Option<Item<'a>>> {
    collection.ensure_unlocked().map_err(|error| io::Error::new(ErrorKind::Other, "ensure_unlocked"))?;
    let attributes = search_attributes(app_id, handle);
    let attributes = attributes.iter().map(|(k, v)| (*k, v.as_str())).collect();
    let mut result = collection.search_items(attributes).map_err(|error| io::Error::new(ErrorKind::Other, "search_items"))?;
    Ok(result.pop())
}

#[cfg(test)]
mod tests {
    use u2f_core::PrivateKey;

    use super::*;

    #[test]
    fn todo() {}
}