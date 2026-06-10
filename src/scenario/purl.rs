// copied from trustify

use packageurl::PackageUrl;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CanonicalPurl {
    pub ty: String,
    pub namespace: Option<String>,
    pub name: String,
    pub version: Option<String>,
    pub qualifiers: BTreeMap<String, String>,
}

impl Display for CanonicalPurl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut purl = PackageUrl::new(&self.ty, &self.name).map_err(|_| fmt::Error)?;
        if let Some(ns) = &self.namespace {
            let _ = purl.with_namespace(ns);
        }
        if let Some(version) = &self.version {
            let _ = purl.with_version(version);
        }
        for (key, value) in &self.qualifiers {
            let _ = purl.add_qualifier(key, value).map_err(|_| fmt::Error)?;
        }
        Display::fmt(&purl, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_full() {
        let purl = CanonicalPurl {
            ty: "rpm".into(),
            namespace: Some("redhat".into()),
            name: "openssl".into(),
            version: Some("3.0.0".into()),
            qualifiers: BTreeMap::new(),
        };
        assert_eq!(purl.to_string(), "pkg:rpm/redhat/openssl@3.0.0");
    }

    #[test]
    fn display_no_namespace() {
        let purl = CanonicalPurl {
            ty: "rpm".into(),
            namespace: None,
            name: "openssl".into(),
            version: Some("3.0.0".into()),
            qualifiers: BTreeMap::new(),
        };
        assert_eq!(purl.to_string(), "pkg:rpm/openssl@3.0.0");
    }

    #[test]
    fn display_no_version() {
        let purl = CanonicalPurl {
            ty: "rpm".into(),
            namespace: Some("redhat".into()),
            name: "openssl".into(),
            version: None,
            qualifiers: BTreeMap::new(),
        };
        assert_eq!(purl.to_string(), "pkg:rpm/redhat/openssl");
    }

    #[test]
    fn display_minimal() {
        let purl = CanonicalPurl {
            ty: "generic".into(),
            namespace: None,
            name: "foo".into(),
            version: None,
            qualifiers: BTreeMap::new(),
        };
        assert_eq!(purl.to_string(), "pkg:generic/foo");
    }

    #[test]
    fn display_with_qualifiers() {
        let mut qualifiers = BTreeMap::new();
        qualifiers.insert("arch".into(), "x86_64".into());
        qualifiers.insert("distro".into(), "rhel".into());
        let purl = CanonicalPurl {
            ty: "rpm".into(),
            namespace: Some("redhat".into()),
            name: "openssl".into(),
            version: Some("3.0.0".into()),
            qualifiers,
        };
        let s = purl.to_string();
        assert!(s.starts_with("pkg:rpm/redhat/openssl@3.0.0"));
        assert!(s.contains("arch=x86_64"));
        assert!(s.contains("distro=rhel"));
    }

    #[test]
    fn display_empty_qualifiers() {
        let purl = CanonicalPurl {
            ty: "maven".into(),
            namespace: Some("com.fasterxml.jackson.core".into()),
            name: "jackson-databind".into(),
            version: Some("2.13.0".into()),
            qualifiers: BTreeMap::new(),
        };
        assert_eq!(
            purl.to_string(),
            "pkg:maven/com.fasterxml.jackson.core/jackson-databind@2.13.0"
        );
    }

    #[test]
    fn serde_roundtrip() {
        let purl = CanonicalPurl {
            ty: "rpm".into(),
            namespace: Some("redhat".into()),
            name: "openssl".into(),
            version: Some("3.0.0".into()),
            qualifiers: BTreeMap::new(),
        };
        let json = serde_json::to_string(&purl).unwrap();
        let back: CanonicalPurl = serde_json::from_str(&json).unwrap();
        assert_eq!(purl, back);
    }
}
