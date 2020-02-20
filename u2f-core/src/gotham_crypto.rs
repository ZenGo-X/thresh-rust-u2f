use app_id::AppId;
use application_key::ApplicationKey;
use attestation::{Attestation, AttestationCertificate};
use key_handle::KeyHandle;
use openssl::hash::{hash, MessageDigest};
use openssl::pkey::PKey;
use openssl::sign::Signer;

use private_key::PrivateKey;
use secp256k1::Signature as secpSignature;
use std::io;

use super::CryptoOperations;
use super::SignError;
use super::Signature;

use client_lib::*;

pub struct GothamCryptoOperations {
    client_shim: ClientShim,
    attestation: Attestation,
}

impl GothamCryptoOperations {
    pub fn new(attestation: Attestation) -> GothamCryptoOperations {
        GothamCryptoOperations {
            client_shim: ClientShim::new("http://localhost:8000".to_string(), None),
            attestation: attestation,
        }
    }

    fn generate_key(&self) -> ecdsa::PrivateShare {
        ecdsa::get_master_key(&self.client_shim)
    }

    fn generate_key_handle() -> io::Result<KeyHandle> {
        Ok(rand::random())
    }

    fn one_party_sign(
        &self,
        key: &PrivateKey,
        data: &[u8],
    ) -> Result<Box<dyn Signature>, SignError> {
        let ec_key = key.0.to_owned();
        let pkey = PKey::from_ec_key(ec_key).unwrap();
        let mut signer = Signer::new(MessageDigest::sha256(), &pkey).unwrap();
        signer.update(data).unwrap();
        let signature = signer.sign_to_vec().unwrap();
        Ok(Box::new(RawSignature(signature)))
    }
}

impl CryptoOperations for GothamCryptoOperations {
    fn attest(&self, data: &[u8]) -> Result<Box<dyn Signature>, SignError> {
        self.one_party_sign(&self.attestation.key, data)
    }

    fn generate_application_key(&self, application: &AppId) -> io::Result<ApplicationKey> {
        let key = self.generate_key();
        let handle = Self::generate_key_handle()?;
        Ok(ApplicationKey::new(*application, handle, key))
    }

    fn get_attestation_certificate(&self) -> AttestationCertificate {
        self.attestation.certificate.clone()
    }

    fn sign(&self, ps: &ecdsa::PrivateShare, data: &[u8]) -> Result<Box<dyn Signature>, SignError> {
        let x_pos = BigInt::from(0);
        let y_pos = BigInt::from(0);

        let child_master_key = ps.master_key.get_child(vec![x_pos.clone(), y_pos.clone()]);

        // TODO check result
        let result = hash(MessageDigest::sha256(), data).unwrap();

        let signature = ecdsa::sign(
            &self.client_shim,
            BigInt::from(&result[..]),
            &child_master_key,
            x_pos,
            y_pos,
            &ps.id,
        )
        .expect("No signature generated. Is server running?");

        let mut v = BigInt::to_vec(&signature.r);
        v.extend(BigInt::to_vec(&signature.s));

        let der_sig = secpSignature::from_compact(&v[..])
            .expect("compact signatures are 64 bytes; DER signatures are 68-72 bytes")
            .serialize_der()
            .to_vec();
        Ok(Box::new(RawSignature(der_sig)))
    }
}

#[derive(Debug)]
struct RawSignature(Vec<u8>);

impl Signature for RawSignature {}

impl AsRef<[u8]> for RawSignature {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
