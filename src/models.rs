use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldType {
    pub id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct BundleElement {
    pub id: Option<String>,
    pub name: Option<String>,
    pub archived: Option<bool>,
    pub ordinal: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Bundle {
    pub id: Option<String>,
    pub values: Option<Vec<BundleElement>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct CustomFieldDefaults {
    pub id: Option<String>,
    pub can_be_empty: Option<bool>,
    pub empty_field_text: Option<String>,
    pub is_public: Option<bool>,
    pub bundle: Option<Bundle>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomField {
    pub id: Option<String>,
    pub name: Option<String>,
    pub localized_name: Option<String>,
    pub field_type: Option<FieldType>,
    pub is_auto_attached: Option<bool>,
    pub is_updateable: Option<bool>,
    pub ordinal: Option<i32>,
    pub aliases: Option<String>,
    pub field_defaults: Option<CustomFieldDefaults>,
}

impl std::fmt::Display for CustomField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ID:             {}", self.id.as_deref().unwrap_or("-"))?;
        writeln!(f, "Name:           {}", self.name.as_deref().unwrap_or("-"))?;
        writeln!(
            f,
            "Localized Name: {}",
            self.localized_name.as_deref().unwrap_or("-")
        )?;
        writeln!(
            f,
            "Field Type:     {}",
            self.field_type
                .as_ref()
                .and_then(|ft| ft.id.as_deref())
                .unwrap_or("-")
        )?;
        writeln!(
            f,
            "Auto-Attached:  {}",
            self.is_auto_attached
                .map(|v| v.to_string())
                .as_deref()
                .unwrap_or("-")
        )?;
        writeln!(
            f,
            "Updateable:     {}",
            self.is_updateable
                .map(|v| v.to_string())
                .as_deref()
                .unwrap_or("-")
        )?;
        writeln!(
            f,
            "Ordinal:        {}",
            self.ordinal
                .map(|v| v.to_string())
                .as_deref()
                .unwrap_or("-")
        )?;
        writeln!(
            f,
            "Aliases:        {}",
            self.aliases.as_deref().unwrap_or("-")
        )?;

        // Print bundle values if present
        if let Some(defaults) = &self.field_defaults
            && let Some(bundle) = &defaults.bundle
            && let Some(values) = &bundle.values
        {
            writeln!(f, "Values:")?;
            for val in values {
                let name = val.name.as_deref().unwrap_or("-");
                let archived = val
                    .archived
                    .map(|a| if a { " (archived)" } else { "" })
                    .unwrap_or("");
                writeln!(f, "  - {}{}", name, archived)?;
            }
            return Ok(());
        }
        write!(f, "Values:         -")
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Leader {
    pub id: Option<String>,
    pub login: Option<String>,
    pub ring_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: Option<String>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub archived: Option<bool>,
    pub description: Option<String>,
    pub leader: Option<Leader>,
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ID:          {}", self.id.as_deref().unwrap_or("-"))?;
        writeln!(f, "Name:        {}", self.name.as_deref().unwrap_or("-"))?;
        writeln!(
            f,
            "Short Name:  {}",
            self.short_name.as_deref().unwrap_or("-")
        )?;
        writeln!(
            f,
            "Archived:    {}",
            self.archived
                .map(|v| v.to_string())
                .as_deref()
                .unwrap_or("-")
        )?;
        writeln!(
            f,
            "Description: {}",
            self.description.as_deref().unwrap_or("-")
        )?;
        write!(
            f,
            "Leader:      {}",
            self.leader
                .as_ref()
                .and_then(|l| l.login.as_deref())
                .unwrap_or("-")
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectCustomField {
    pub id: Option<String>,
    pub field: Option<CustomField>,
    pub can_be_empty: Option<bool>,
    pub empty_field_text: Option<String>,
    pub ordinal: Option<i32>,
    pub is_public: Option<bool>,
    pub bundle: Option<Bundle>,
}

impl std::fmt::Display for ProjectCustomField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ID:               {}", self.id.as_deref().unwrap_or("-"))?;
        writeln!(
            f,
            "Field Name:       {}",
            self.field
                .as_ref()
                .and_then(|fld| fld.name.as_deref())
                .unwrap_or("-")
        )?;
        writeln!(
            f,
            "Localized Name:   {}",
            self.field
                .as_ref()
                .and_then(|fld| fld.localized_name.as_deref())
                .unwrap_or("-")
        )?;
        writeln!(
            f,
            "Field Type:       {}",
            self.field
                .as_ref()
                .and_then(|fld| fld.field_type.as_ref())
                .and_then(|ft| ft.id.as_deref())
                .unwrap_or("-")
        )?;
        writeln!(
            f,
            "Can Be Empty:     {}",
            self.can_be_empty
                .map(|v| v.to_string())
                .as_deref()
                .unwrap_or("-")
        )?;
        writeln!(
            f,
            "Empty Field Text: {}",
            self.empty_field_text.as_deref().unwrap_or("-")
        )?;
        writeln!(
            f,
            "Ordinal:          {}",
            self.ordinal
                .map(|v| v.to_string())
                .as_deref()
                .unwrap_or("-")
        )?;
        writeln!(
            f,
            "Public:           {}",
            self.is_public
                .map(|v| v.to_string())
                .as_deref()
                .unwrap_or("-")
        )?;

        // Print bundle values if present
        if let Some(bundle) = &self.bundle
            && let Some(values) = &bundle.values
        {
            writeln!(f, "Values:")?;
            for val in values {
                let name = val.name.as_deref().unwrap_or("-");
                let archived = val
                    .archived
                    .map(|a| if a { " (archived)" } else { "" })
                    .unwrap_or("");
                writeln!(f, "  - {}{}", name, archived)?;
            }
            return Ok(());
        }
        write!(f, "Values:           -")
    }
}

// ── Request payloads ──────────────────────────────────────────────────

/// Body for POST /admin/customFieldSettings/customFields
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCustomFieldRequest {
    pub name: String,
    pub field_type: FieldType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auto_attached: Option<bool>,
}

/// Minimal reference to an existing global custom field (by id).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldRef {
    pub id: String,
}

/// Body for POST /admin/projects/{id}/customFields
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachProjectCustomFieldRequest {
    pub field: CustomFieldRef,
}

/// Body for POST /admin/customFieldSettings/bundles/{type}/{id}/values
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBundleValueRequest {
    pub name: String,
}
