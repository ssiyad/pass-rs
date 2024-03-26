mod fs;

use clap::ValueEnum;

#[derive(Clone, ValueEnum)]
pub enum Backend {
    Fs,
}

impl Backend {
    /// Additional initialization steps for the crypto backend
    pub fn init(&self) {
        match self {
            Backend::Fs => fs::init(),
        }
    }

    /// Read the content
    ///
    /// * `name`:
    pub fn read(&self, name: String) -> Vec<u8> {
        match self {
            Backend::Fs => fs::read(name),
        }
    }

    /// Write the content
    ///
    /// * `name`:
    /// * `content`:
    pub fn write(&self, name: String, content: Vec<u8>) {
        match self {
            Backend::Fs => fs::write(name, content),
        }
    }

    /// Remove secret
    ///
    /// * `name`:
    pub fn remove(&self, name: String) {
        match self {
            Backend::Fs => fs::remove(name),
        }
    }
}

/// Get the crypto backend
pub fn get() -> Backend {
    let root = crate::args::root();
    let storage_config = root.join(".pass").join("storage");
    let storage_type = std::fs::read_to_string(storage_config)
        .unwrap()
        .trim()
        .to_owned();

    match storage_type.as_str() {
        "fs" => Backend::Fs,
        _ => panic!("No storage setup"),
    }
}
