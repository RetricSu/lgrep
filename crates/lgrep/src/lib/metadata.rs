use std::env;
use std::string::ToString;

pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u16,
    pub pre: String,
}

pub struct Metadata {
    pub name: String,
    pub author: String,
    pub version: Version,
    pub description: String,
}

impl ToString for Version {
    fn to_string(&self) -> String {
        let mut version = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if !self.pre.is_empty() {
            version.push('-');
            version.push_str(&self.pre);
        }
        version.trim().to_string()
    }
}

impl Version {
    pub fn current() -> Self {
        let major = env!("CARGO_PKG_VERSION_MAJOR")
            .parse::<u8>()
            .expect("CARGO_PKG_VERSION_MAJOR parse success");
        let minor = env!("CARGO_PKG_VERSION_MINOR")
            .parse::<u8>()
            .expect("CARGO_PKG_VERSION_MINOR parse success");
        let patch = env!("CARGO_PKG_VERSION_PATCH")
            .parse::<u16>()
            .expect("CARGO_PKG_VERSION_PATCH parse success");

        let pre = env!("CARGO_PKG_VERSION_PRE")
            .parse::<String>()
            .expect("CARGO_PKG_VERSION_PRE parse success");
        Self {
            major,
            minor,
            patch,
            pre,
        }
    }
}

impl Metadata {
    pub fn current() -> Self {
        let name = env!("CARGO_PKG_NAME")
            .parse::<String>()
            .expect("CARGO_PKG_AUTHORS parse success");

        let author = env!("CARGO_PKG_AUTHORS")
            .parse::<String>()
            .expect("CARGO_PKG_AUTHORS parse success");

        let description = env!("CARGO_PKG_DESCRIPTION")
            .parse::<String>()
            .expect("CARGO_PKG_DESCRIPTION parse success");

        let version = Version::current();

        Self {
            name,
            author,
            version,
            description,
        }
    }
}
