#![cfg(feature = "rustls-tls")]

use rustls::{Certificate, PrivateKey};
use rustls_pemfile::{certs, rsa_private_keys};

use std::{borrow::Cow, path::Path};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClientIdentity {
    cert_chain_path: Cow<'static, Path>,
    priv_key_path: Cow<'static, Path>,
}

impl ClientIdentity {
    /// Creates new identity.
    ///
    /// `cert_chain_path` - path to a certificate chain (in PEM or DER)
    /// `priv_key_path` - path to a private key (in DER or PEM) (it'll take the first one)
    pub fn new<T, U>(cert_chain_path: T, priv_key_path: U) -> Self
    where
        T: Into<Cow<'static, Path>>,
        U: Into<Cow<'static, Path>>,
    {
        Self {
            cert_chain_path: cert_chain_path.into(),
            priv_key_path: priv_key_path.into(),
        }
    }

    /// Sets the certificate chain path (in DER or PEM).
    pub fn with_cert_chain_path<T>(mut self, cert_chain_path: T) -> Self
    where
        T: Into<Cow<'static, Path>>,
    {
        self.cert_chain_path = cert_chain_path.into();
        self
    }

    /// Sets the private key path (in DER or PEM) (it'll take the first one).
    pub fn with_priv_key_path<T>(mut self, priv_key_path: T) -> Self
    where
        T: Into<Cow<'static, Path>>,
    {
        self.priv_key_path = priv_key_path.into();
        self
    }

    /// Returns the certificate chain path.
    pub fn cert_chain_path(&self) -> &Path {
        self.cert_chain_path.as_ref()
    }

    /// Returns the private key path.
    pub fn priv_key_path(&self) -> &Path {
        self.priv_key_path.as_ref()
    }

    pub(crate) fn load(&self) -> crate::Result<(Vec<Certificate>, PrivateKey)> {
        let cert_data = std::fs::read(self.cert_chain_path.as_ref())?;
        let key_data = std::fs::read(self.priv_key_path.as_ref())?;

        let mut cert_chain = Vec::new();
        if std::str::from_utf8(&cert_data).is_err() {
            cert_chain.push(Certificate(cert_data));
        } else {
            for cert in certs(&mut &*cert_data)? {
                cert_chain.push(Certificate(cert));
            }
        }

        let priv_key;
        if std::str::from_utf8(&key_data).is_err() {
            priv_key = Some(PrivateKey(key_data));
        } else {
            priv_key = rsa_private_keys(&mut &*key_data)?
                .into_iter()
                .take(1)
                .map(PrivateKey)
                .next();
        }

        Ok((
            cert_chain,
            priv_key.ok_or_else(|| crate::Error::from(crate::DriverError::NoKeyFound))?,
        ))
    }
}
