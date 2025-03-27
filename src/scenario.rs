use anyhow::{Context, anyhow};
use sqlx::{Executor, Row, any::AnyRow};
use std::io::BufReader;

/// implement to that we can explicitly state what we want
mod required {
    use serde::{
        Deserialize, Deserializer, Serializer,
        de::{Error, Visitor},
    };
    use std::fmt::Formatter;

    pub fn serialize<S>(value: &Option<String>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            None => s.serialize_none(),
            Some(value) => s.serialize_some(value),
        }
    }

    pub fn deserialize<'de, D>(d: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct De;

        impl<'de> Visitor<'de> for De {
            type Value = Option<String>;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("field must be present, but may be 'null' to deactivate")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Some(v.to_string()))
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                Ok(Some(String::deserialize(deserializer)?))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(None)
            }
        }

        d.deserialize_option(De)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Scenario {
    #[serde(with = "required")]
    pub get_sbom: Option<String>,

    #[serde(with = "required")]
    pub get_sbom_advisories: Option<String>,

    #[serde(with = "required")]
    pub get_sbom_packages: Option<String>,

    #[serde(with = "required")]
    pub get_sbom_related: Option<String>,

    #[serde(with = "required")]
    pub get_vulnerability: Option<String>,
}

impl Scenario {
    /// Load a scenario file, or evaluate one
    pub async fn load(scenario_file: Option<&str>) -> anyhow::Result<Self> {
        if let Some(scenario_file) = scenario_file {
            Ok(serde_json5::from_reader(BufReader::new(
                std::fs::File::open(scenario_file)
                    .with_context(|| format!("opening scenario file: {scenario_file}"))?,
            ))
            .context("reading scenario file")?)
        } else {
            Self::eval().await
        }
    }

    pub async fn eval() -> anyhow::Result<Self> {
        let db = std::env::var("DATABASE_URL")
            .map_err(|err| anyhow!("failed to get database URL from `DATABASE_URL`: {err}"))?;

        let loader = Loader::new(db);

        let large_sbom = loader.large_sbom().await?;
        let large_sbom_id = Some(large_sbom.0);
        let large_sbom_digest = Some(large_sbom.1);
        let max_vuln = Some(loader.max_vuln().await?);

        Ok(Self {
            get_sbom: large_sbom_digest.clone(),
            get_sbom_advisories: large_sbom_digest.clone(),
            get_sbom_related: large_sbom_id.clone(),
            get_sbom_packages: large_sbom_id.clone(),

            get_vulnerability: max_vuln,
        })
    }
}

struct Loader {
    db: String,
}

impl Loader {
    pub fn new(db: String) -> Self {
        Self { db }
    }

    async fn find(&self, sql: &str) -> anyhow::Result<String> {
        Ok(self.find_row(sql).await?.get("result"))
    }

    async fn find_row(&self, sql: &str) -> anyhow::Result<AnyRow> {
        let mut db = crate::db::connect(&self.db).await?;

        db.fetch_optional(sql)
            .await?
            .ok_or_else(|| anyhow!("nothing found"))
    }

    /// get the SHA256 of the largest SBOM (by number of packages)
    pub async fn large_sbom(&self) -> anyhow::Result<(String, String)> {
        // get the largest SBOM in the database
        let row = self
            .find_row(
                r#"
select
    b.sbom_id::text as id,
    concat('sha256:', c.sha256) as sha,
    count(b.node_id) as num
from sbom a
     join sbom_node b on a.sbom_id = b.sbom_id
     join source_document c on a.source_document_id = c.id
group by
    b.sbom_id,
    c.sha256
order by
    num desc
limit 1
"#,
            )
            .await?;

        Ok((row.get("id"), row.get("sha")))
    }

    /// A vulnerability, referenced by a lot of advisories
    pub async fn max_vuln(&self) -> anyhow::Result<String> {
        self.find(
            r#"
select
    a.id as result,
    count(b.vulnerability_id) as num
from vulnerability a
     join advisory_vulnerability b on a.id = b.vulnerability_id
group by
    a.id
order by num desc
"#,
        )
        .await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
    pub(crate) struct Scenario {
        #[serde(with = "required")]
        pub large_sbom: Option<String>,
    }

    #[test]
    fn missing() {
        let err = serde_json::from_str::<Scenario>(r#"{}"#).expect_err("Must be an error");
        assert_eq!(
            err.to_string(),
            "missing field `large_sbom` at line 1 column 2"
        );
    }

    #[test]
    fn skip() {
        let s = serde_json::from_str::<Scenario>(r#"{"large_sbom": null}"#).expect("Must be ok");
        assert_eq!(s.large_sbom, None);
    }

    #[test]
    fn present() {
        let s = serde_json::from_str::<Scenario>(r#"{"large_sbom": "foo"}"#).expect("Must be ok");
        assert_eq!(s.large_sbom.as_deref(), Some("foo"));
    }

    #[test]
    fn missing_json5() {
        let err = serde_json5::from_str::<Scenario>(r#"{}"#).expect_err("Must be an error");
        assert_eq!(err.to_string(), "missing field `large_sbom`");
    }

    #[test]
    fn skip_json5() {
        let s = serde_json5::from_str::<Scenario>(r#"{"large_sbom": null}"#).expect("Must be ok");
        assert_eq!(s.large_sbom, None);
    }

    #[test]
    fn present_json5() {
        let s = serde_json5::from_str::<Scenario>(r#"{"large_sbom": "foo"}"#).expect("Must be ok");
        assert_eq!(s.large_sbom.as_deref(), Some("foo"));
    }

    // Ensure the empty file parses
    #[test]
    fn empty() {
        let _ = serde_json5::from_str::<super::Scenario>(include_str!("../empty.json5"))
            .expect("Must be ok");
    }
}
