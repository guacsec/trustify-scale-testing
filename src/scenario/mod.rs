mod purl;

use crate::{scenario::purl::CanonicalPurl, utils::DisplayVec};
use anyhow::{Context, anyhow};
use serde_json::Value;
use sqlx::{Executor, Row, postgres::PgRow};
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

    #[serde(with = "required")]
    pub sbom_by_package: Option<String>,

    #[serde(with = "required")]
    pub sbom_license_ids: Option<String>,

    #[serde(with = "required")]
    pub analyze_purl: Option<String>,

    #[serde(with = "required")]
    pub get_purl_details: Option<String>,

    pub get_recommendations: Option<DisplayVec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_sbom_pool: Option<Vec<String>>,

    #[serde(with = "required")]
    pub download_advisory: Option<String>,

    #[serde(with = "required")]
    pub get_advisory: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub put_advisory_lables: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch_advisory_lables: Option<Vec<String>>,
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
        let sbom_purl = Some(loader.sbom_purl().await?);
        let sbom_license_ids = large_sbom_id.clone().map(|id| format!("urn:uuid:{id}"));
        let analyze_purl = Some(loader.analysis_purl().await?);
        let get_purl_details = Some(loader.purl_details().await?);
        let recommendations_purl = Some(loader.purl_with_recommendations().await?);
        let delete_sbom_pool = Some(
            loader
                .deletable_sboms()
                .await?
                .iter()
                .map(|sbom_id| format!("urn:uuid:{sbom_id}"))
                .collect(),
        );
        let download_advisory = Some(loader.download_advisory().await?);
        let get_advisory = Some(loader.download_advisory().await?);

        let put_advisory_lables = Some(
            loader
                .put_advisory_lables()
                .await?
                .iter()
                .map(|advisory_id| format!("urn:uuid:{advisory_id}"))
                .collect(),
        );
        let patch_advisory_lables = Some(
            loader
                .patch_advisory_lables()
                .await?
                .iter()
                .map(|advisory_id| format!("urn:uuid:{advisory_id}"))
                .collect(),
        );

        Ok(Self {
            get_sbom: large_sbom_digest.clone(),
            get_sbom_advisories: large_sbom_digest.clone(),
            get_sbom_related: large_sbom_id.clone(),
            get_sbom_packages: large_sbom_id.clone(),

            get_vulnerability: max_vuln,

            sbom_by_package: sbom_purl,
            sbom_license_ids,
            analyze_purl,
            get_purl_details,
            get_recommendations: recommendations_purl,
            delete_sbom_pool,
            download_advisory,
            get_advisory,
            put_advisory_lables,
            patch_advisory_lables,
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

    async fn find_row(&self, sql: &str) -> anyhow::Result<PgRow> {
        let mut db = crate::db::connect(&self.db).await?;

        db.fetch_optional(sql)
            .await?
            .ok_or_else(|| anyhow!("nothing found"))
    }

    async fn find_rows(&self, sql: &str) -> anyhow::Result<Vec<PgRow>> {
        let mut db = crate::db::connect(&self.db).await?;

        Ok(db.fetch_all(sql).await?)
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

    /// A purl
    pub async fn sbom_purl(&self) -> anyhow::Result<String> {
        self.find_row(
            r#"
select
    b.purl as result
from
    sbom_package_purl_ref a
    left join qualified_purl b on a.qualified_purl_id = b.id
limit 1
"#,
        )
        .await
        .and_then(|row| {
            let value: Value = row.try_get("result")?;
            let purl: CanonicalPurl = serde_json::from_value(value)?;
            Ok::<String, anyhow::Error>(purl.to_string())
        })
    }

    /// A purl with vulnerabilities
    pub async fn analysis_purl(&self) -> anyhow::Result<String> {
        self.find_row(
            r#"
select distinct
    d.vulnerability_id,
    d.advisory_id,
    a.purl as result
from
    qualified_purl a
    left join versioned_purl b on a.versioned_purl_id = b.id
    left join base_purl c on b.base_purl_id = c.id
    inner join purl_status d on d.base_purl_id = c.id
    inner join status e on e.id = d.status_id
    inner join version_range f on d.version_range_id = f.id
where
    e.slug = 'affected'
and
    version_matches(b.version, f.*) = TRUE
order by
    vulnerability_id
limit 1
"#,
        )
        .await
        .and_then(|row| {
            let value: Value = row.try_get("result")?;
            let purl: CanonicalPurl = serde_json::from_value(value)?;
            Ok::<String, anyhow::Error>(purl.to_string())
        })
    }

    /// A purl ID for details lookup
    pub async fn purl_details(&self) -> anyhow::Result<String> {
        self.find(
            r#"
SELECT 
    spr.qualified_purl_id::text AS result,
    COUNT(DISTINCT spl.license_id) AS license_count
FROM 
    sbom_package_purl_ref spr
JOIN 
    sbom_package sp ON spr.sbom_id = sp.sbom_id AND spr.node_id = sp.node_id
JOIN 
    sbom_package_license spl ON sp.sbom_id = spl.sbom_id AND sp.node_id = spl.node_id
GROUP BY 
    spr.qualified_purl_id
ORDER BY 
    license_count DESC
LIMIT 1;
"#,
        )
        .await
    }

    // A purl whose version matches redhat-[0-9]+$ regex
    pub async fn purl_with_recommendations(&self) -> anyhow::Result<DisplayVec<String>> {
        self.find_rows(
            r#"
SELECT
    purl AS result
FROM
    qualified_purl
WHERE
    purl->>'version' ~ 'redhat-[0-9]+$'
LIMIT 10;
"#,
        )
        .await
        .and_then(|rows| {
            let mut result = vec![];
            for row in rows {
                let value: Value = row.try_get("result")?;
                let purl: CanonicalPurl = serde_json::from_value(value)?;
                result.push(purl.to_string());
            }
            Ok(DisplayVec(result))
        })
    }

    /// Get a pool of deletable SBOMs (up to 100)
    /// These SBOMs are selected based on having certain licenses that make them good candidates for deletion testing
    pub async fn deletable_sboms(&self) -> anyhow::Result<Vec<String>> {
        let mut db = crate::db::connect(&self.db).await?;

        let rows = sqlx::query(
            r#"
SELECT
    a.sbom_id::text as id
FROM
    sbom a
    JOIN sbom_node b ON a.sbom_id = b.sbom_id
    JOIN sbom_package c ON b.sbom_id = c.sbom_id AND b.node_id = c.node_id
    JOIN sbom_package_license d ON c.sbom_id = d.sbom_id AND c.node_id = d.node_id
    JOIN license e ON d.license_id = e.id
WHERE
    e.text IN ('GPLv3+ with exceptions', 'Apache-2.0', 'MIT', 'BSD-3-Clause')
GROUP BY
    a.sbom_id
ORDER BY
    a.sbom_id
LIMIT 100
"#,
        )
        .fetch_all(&mut db)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| row.get::<String, _>("id"))
            .collect())
    }

    /// A advisory ID for download and query the advisory details
    pub async fn download_advisory(&self) -> anyhow::Result<String> {
        self.find(
            r#"
SELECT id::text as result
FROM public.advisory order by modified desc limit 1;"#,
        )
        .await
    }

    /// Advisory IDs for put labels
    pub async fn put_advisory_lables(&self) -> anyhow::Result<Vec<String>> {
        let mut db = crate::db::connect(&self.db).await?;

        let rows = sqlx::query("SELECT id::text as id FROM public.advisory  where labels is not null order by modified desc limit 20;")
            .fetch_all(&mut db)
            .await?;
        Ok(rows
            .into_iter()
            .map(|row| row.get::<String, _>("id"))
            .collect())
    }

    /// Advisory IDs for patch labels
    pub async fn patch_advisory_lables(&self) -> anyhow::Result<Vec<String>> {
        let mut db = crate::db::connect(&self.db).await?;

        let rows = sqlx::query("SELECT id::text as id FROM public.advisory  where labels is not null order by modified desc OFFSET 20 limit 20;")
            .fetch_all(&mut db)
            .await?;
        Ok(rows
            .into_iter()
            .map(|row| row.get::<String, _>("id"))
            .collect())
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
        let _ = serde_json5::from_str::<super::Scenario>(include_str!("../../empty.json5"))
            .expect("Must be ok");
    }
}
