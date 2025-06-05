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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut purl = PackageUrl::new(&self.ty, &self.name).map_err(|_| fmt::Error)?;
        if let Some(ns) = &self.namespace {
            purl.with_namespace(ns);
        }
        if let Some(version) = &self.version {
            purl.with_version(version);
        }
        for (key, value) in &self.qualifiers {
            purl.add_qualifier(key, value).map_err(|_| fmt::Error)?;
        }
        Display::fmt(&purl, f)
    }
}
