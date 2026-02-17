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
