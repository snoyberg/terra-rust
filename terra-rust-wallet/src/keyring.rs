//! Silly insecure stub to replace the keyring crate
use std::{cell::RefCell, collections::HashMap, path::PathBuf};

use anyhow::*;

pub type KeyringError = anyhow::Error;

type Wallet = HashMap<Service, HashMap<Username, Password>>;
type Service = String;
type Username = String;
type Password = String;

pub struct Keyring {
    path: PathBuf,
    wallet: RefCell<Wallet>,
    service: String,
    username: String,
}

impl Keyring {
    pub fn new(service: &str, username: &str) -> Result<Self> {
        let path: PathBuf = std::env::var("TERRA_RUST_WALLET_FILE")
            .context("Need to set env var TERRA_RUST_WALLET_FILE")?
            .into();
        let wallet = if path.exists() {
            let mut file = std::fs::File::open(&path)
                .with_context(|| format!("Could not open {}", path.display()))?;
            serde_json::from_reader(&mut file)
                .with_context(|| format!("Could not read from {}", path.display()))?
        } else {
            Wallet::new()
        };
        Ok(Keyring {
            path,
            wallet: RefCell::new(wallet),
            service: service.to_owned(),
            username: username.to_owned(),
        })
    }

    fn set(&self, password: Option<String>) -> Result<()> {
        let mut w = self.wallet.borrow_mut();
        let h = w.entry(self.service.clone()).or_default();
        match password {
            None => {
                h.remove(&self.username);
            }
            Some(p) => {
                h.insert(self.username.clone(), p);
            }
        }

        if let Some(dir) = self.path.parent() {
            std::fs::create_dir_all(&dir)
                .with_context(|| format!("Could not create dir {}", dir.display()))?;
        }

        let mut file = std::fs::File::create(&self.path)
            .with_context(|| format!("Could not create file {}", self.path.display()))?;
        serde_json::to_writer(&mut file, &*w)
            .with_context(|| format!("Error writing JSON to {}", self.path.display()))
    }

    pub fn set_password(&self, password: &str) -> Result<()> {
        self.set(Some(password.to_owned()))
    }

    pub fn get_password(&self) -> Result<String> {
        self.wallet
            .borrow()
            .get(&self.service)
            .context("Wallet not created (service)")?
            .get(&self.username)
            .context("Wallet not created (username)")
            .map(|s| s.clone())
    }

    pub fn delete_password(&self) -> Result<()> {
        self.set(None)
    }
}
