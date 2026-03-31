use anyhow::{Context, Result};
use reqwest::blocking::Client;

use crate::models::{
    AttachProjectCustomFieldRequest, BundleElement, CreateBundleValueRequest,
    CreateCustomFieldRequest, CustomField, CustomFieldRef, FieldType, Project, ProjectCustomField,
};

const DEFAULT_CUSTOM_FIELD_FIELDS: &str =
    "$type,fieldType($type,id),id,isAutoAttached,isUpdateable,localizedName,name,ordinal,aliases";

const DESCRIBE_CUSTOM_FIELD_FIELDS: &str = "$type,aliases,fieldDefaults($type,bundle($type,id,values($type,archived,id,name,ordinal)),canBeEmpty,emptyFieldText,id,isPublic),fieldType($type,id),id,isAutoAttached,isUpdateable,localizedName,name,ordinal";

const DEFAULT_PROJECT_FIELDS: &str =
    "$type,archived,description,id,leader($type,id,login,ringId),name,shortName";

const DEFAULT_PROJECT_CUSTOM_FIELD_FIELDS: &str = "$type,canBeEmpty,emptyFieldText,field($type,fieldType($type,id),id,localizedName,name),id,isPublic,ordinal";

const DESCRIBE_PROJECT_CUSTOM_FIELD_FIELDS: &str = "$type,bundle($type,id,values($type,archived,id,name,ordinal)),canBeEmpty,emptyFieldText,field($type,fieldType($type,id),id,localizedName,name),id,isPublic,ordinal";

pub struct YouTrackClient {
    base_url: String,
    client: Client,
}

impl YouTrackClient {
    /// Build a new client.
    ///
    /// * `base_url` – YouTrack API root, e.g. `https://example.youtrack.cloud/api`
    /// * `api_key`  – permanent token used as Bearer credential
    pub fn new(base_url: &str, api_key: &str) -> Result<Self> {
        use reqwest::header::{self, HeaderMap, HeaderValue};

        let mut headers = HeaderMap::new();
        let auth = format!("Bearer {api_key}");
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&auth).context("invalid API key characters")?,
        );
        headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .context("failed to build HTTP client")?;

        Ok(Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client,
        })
    }

    /// List custom fields with optional pagination.
    pub fn list_custom_fields(
        &self,
        skip: Option<u32>,
        top: Option<u32>,
    ) -> Result<Vec<CustomField>> {
        let mut request = self
            .client
            .get(format!(
                "{}/admin/customFieldSettings/customFields",
                self.base_url
            ))
            .query(&[("fields", DEFAULT_CUSTOM_FIELD_FIELDS)]);

        if let Some(s) = skip {
            request = request.query(&[("$skip", s)]);
        }
        if let Some(t) = top {
            request = request.query(&[("$top", t)]);
        }

        let response = request.send().context("request failed")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            anyhow::bail!("YouTrack API returned {status}: {body}");
        }

        let fields: Vec<CustomField> = response.json().context("failed to parse response")?;
        Ok(fields)
    }

    /// Find a single custom field by exact name (case-insensitive), including bundle values.
    pub fn get_custom_field_by_name(&self, name: &str) -> Result<Option<CustomField>> {
        let mut request = self
            .client
            .get(format!(
                "{}/admin/customFieldSettings/customFields",
                self.base_url
            ))
            .query(&[("fields", DESCRIBE_CUSTOM_FIELD_FIELDS)]);

        // No pagination — fetch all to search by name
        let _ = &mut request;

        let response = request.send().context("request failed")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            anyhow::bail!("YouTrack API returned {status}: {body}");
        }

        let all: Vec<CustomField> = response.json().context("failed to parse response")?;
        let needle = name.to_lowercase();
        Ok(all
            .into_iter()
            .find(|cf| cf.name.as_deref().map(|n| n.to_lowercase()) == Some(needle.clone())))
    }

    /// List projects with optional pagination.
    pub fn list_projects(&self, skip: Option<u32>, top: Option<u32>) -> Result<Vec<Project>> {
        let mut request = self
            .client
            .get(format!("{}/admin/projects", self.base_url))
            .query(&[("fields", DEFAULT_PROJECT_FIELDS)]);

        if let Some(s) = skip {
            request = request.query(&[("$skip", s)]);
        }
        if let Some(t) = top {
            request = request.query(&[("$top", t)]);
        }

        let response = request.send().context("request failed")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            anyhow::bail!("YouTrack API returned {status}: {body}");
        }

        let projects: Vec<Project> = response.json().context("failed to parse response")?;
        Ok(projects)
    }

    /// Find a single project by short name (case-insensitive).
    pub fn get_project_by_name(&self, name: &str) -> Result<Option<Project>> {
        let all = self.list_projects(None, None)?;
        let needle = name.to_lowercase();
        Ok(all
            .into_iter()
            .find(|p| p.short_name.as_deref().map(|n| n.to_lowercase()) == Some(needle.clone())))
    }

    /// Resolve a project short name to its ID.
    fn resolve_project_id(&self, short_name: &str) -> Result<String> {
        let project = self
            .get_project_by_name(short_name)?
            .ok_or_else(|| anyhow::anyhow!("project with short name '{}' not found", short_name))?;
        project
            .id
            .ok_or_else(|| anyhow::anyhow!("project '{}' has no ID", short_name))
    }

    /// List custom fields scoped to a project (by short name) with optional pagination.
    pub fn list_project_custom_fields(
        &self,
        project_short_name: &str,
        skip: Option<u32>,
        top: Option<u32>,
    ) -> Result<Vec<ProjectCustomField>> {
        let project_id = self.resolve_project_id(project_short_name)?;

        let mut request = self
            .client
            .get(format!(
                "{}/admin/projects/{}/customFields",
                self.base_url, project_id
            ))
            .query(&[("fields", DEFAULT_PROJECT_CUSTOM_FIELD_FIELDS)]);

        if let Some(s) = skip {
            request = request.query(&[("$skip", s)]);
        }
        if let Some(t) = top {
            request = request.query(&[("$top", t)]);
        }

        let response = request.send().context("request failed")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            anyhow::bail!("YouTrack API returned {status}: {body}");
        }

        let fields: Vec<ProjectCustomField> =
            response.json().context("failed to parse response")?;
        Ok(fields)
    }

    /// Find a single project-scoped custom field by field name (case-insensitive), including bundle values.
    pub fn get_project_custom_field_by_name(
        &self,
        project_short_name: &str,
        name: &str,
    ) -> Result<Option<ProjectCustomField>> {
        let project_id = self.resolve_project_id(project_short_name)?;

        let request = self
            .client
            .get(format!(
                "{}/admin/projects/{}/customFields",
                self.base_url, project_id
            ))
            .query(&[("fields", DESCRIBE_PROJECT_CUSTOM_FIELD_FIELDS)]);

        let response = request.send().context("request failed")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            anyhow::bail!("YouTrack API returned {status}: {body}");
        }

        let all: Vec<ProjectCustomField> = response.json().context("failed to parse response")?;
        let needle = name.to_lowercase();
        Ok(all.into_iter().find(|pcf| {
            pcf.field
                .as_ref()
                .and_then(|f| f.name.as_deref())
                .map(|n| n.to_lowercase())
                == Some(needle.clone())
        }))
    }

    // ── Create operations ─────────────────────────────────────────────

    /// Create a new global custom field.
    pub fn create_custom_field(
        &self,
        name: &str,
        field_type_id: &str,
        auto_attach: Option<bool>,
    ) -> Result<CustomField> {
        let body = CreateCustomFieldRequest {
            name: name.to_string(),
            field_type: FieldType {
                id: Some(field_type_id.to_string()),
            },
            is_auto_attached: auto_attach,
        };

        let response = self
            .client
            .post(format!(
                "{}/admin/customFieldSettings/customFields",
                self.base_url
            ))
            .query(&[("fields", DESCRIBE_CUSTOM_FIELD_FIELDS)])
            .json(&body)
            .send()
            .context("request failed")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            anyhow::bail!("YouTrack API returned {status}: {body}");
        }

        response.json().context("failed to parse response")
    }

    /// Attach an existing global custom field to a project.
    pub fn attach_custom_field_to_project(
        &self,
        project_short_name: &str,
        custom_field_id: &str,
    ) -> Result<ProjectCustomField> {
        let project_id = self.resolve_project_id(project_short_name)?;

        let body = AttachProjectCustomFieldRequest {
            field: CustomFieldRef {
                id: custom_field_id.to_string(),
            },
        };

        let response = self
            .client
            .post(format!(
                "{}/admin/projects/{}/customFields",
                self.base_url, project_id
            ))
            .query(&[("fields", DESCRIBE_PROJECT_CUSTOM_FIELD_FIELDS)])
            .json(&body)
            .send()
            .context("request failed")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            anyhow::bail!("YouTrack API returned {status}: {body}");
        }

        response.json().context("failed to parse response")
    }

    /// Map a field type id (e.g. "enum[1]", "state[1]") to the bundle URL segment.
    fn bundle_type_from_field_type(field_type_id: &str) -> Result<&'static str> {
        let ft = field_type_id.to_lowercase();
        if ft.starts_with("enum") {
            Ok("enum")
        } else if ft.starts_with("state") {
            Ok("state")
        } else if ft.starts_with("ownedfield") || ft.starts_with("owned") {
            Ok("ownedField")
        } else if ft.starts_with("version") {
            Ok("version")
        } else if ft.starts_with("build") {
            Ok("build")
        } else {
            anyhow::bail!(
                "field type '{}' does not support bundle values",
                field_type_id
            )
        }
    }

    /// Add a value to a global custom field's bundle.
    /// Resolves field → fieldDefaults → bundle, then POSTs to the correct bundle type endpoint.
    pub fn add_value_to_custom_field(
        &self,
        field_name: &str,
        value_name: &str,
    ) -> Result<BundleElement> {
        // Fetch the field with bundle info
        let cf = self
            .get_custom_field_by_name(field_name)?
            .ok_or_else(|| anyhow::anyhow!("custom field '{}' not found", field_name))?;

        let field_type_id = cf
            .field_type
            .as_ref()
            .and_then(|ft| ft.id.as_deref())
            .ok_or_else(|| anyhow::anyhow!("custom field '{}' has no field type", field_name))?;

        let bundle_type = Self::bundle_type_from_field_type(field_type_id)?;

        let bundle_id = cf
            .field_defaults
            .as_ref()
            .and_then(|d| d.bundle.as_ref())
            .and_then(|b| b.id.as_deref())
            .ok_or_else(|| {
                anyhow::anyhow!("custom field '{}' has no associated bundle", field_name)
            })?;

        self.post_bundle_value(bundle_type, bundle_id, value_name)
    }

    /// Add a value to a project-scoped custom field's bundle.
    pub fn add_value_to_project_custom_field(
        &self,
        project_short_name: &str,
        field_name: &str,
        value_name: &str,
    ) -> Result<BundleElement> {
        let pcf = self
            .get_project_custom_field_by_name(project_short_name, field_name)?
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "custom field '{}' not found in project '{}'",
                    field_name,
                    project_short_name
                )
            })?;

        let field_type_id = pcf
            .field
            .as_ref()
            .and_then(|f| f.field_type.as_ref())
            .and_then(|ft| ft.id.as_deref())
            .ok_or_else(|| anyhow::anyhow!("custom field '{}' has no field type", field_name))?;

        let bundle_type = Self::bundle_type_from_field_type(field_type_id)?;

        let bundle_id = pcf
            .bundle
            .as_ref()
            .and_then(|b| b.id.as_deref())
            .ok_or_else(|| {
                anyhow::anyhow!("custom field '{}' has no associated bundle", field_name)
            })?;

        self.post_bundle_value(bundle_type, bundle_id, value_name)
    }

    /// POST a new value to /admin/customFieldSettings/bundles/{type}/{bundleId}/values.
    fn post_bundle_value(
        &self,
        bundle_type: &str,
        bundle_id: &str,
        value_name: &str,
    ) -> Result<BundleElement> {
        let body = CreateBundleValueRequest {
            name: value_name.to_string(),
        };

        let response = self
            .client
            .post(format!(
                "{}/admin/customFieldSettings/bundles/{}/{}/values",
                self.base_url, bundle_type, bundle_id
            ))
            .query(&[("fields", "$type,archived,id,name,ordinal")])
            .json(&body)
            .send()
            .context("request failed")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            anyhow::bail!("YouTrack API returned {status}: {body}");
        }

        response.json().context("failed to parse response")
    }
}
