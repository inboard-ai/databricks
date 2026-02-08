use serde::{Deserialize, Serialize};

// ============================================================================
// Dashboard (Lakeview) types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<LifecycleState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serialized_dashboard: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LifecycleState {
    Active,
    Trashed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DashboardView {
    #[serde(rename = "DASHBOARD_VIEW_BASIC")]
    Basic,
}

// ============================================================================
// Request / Response types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDashboardRequest {
    pub dashboard: Dashboard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDashboardRequest {
    pub dashboard: Dashboard,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListDashboardsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_trashed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<DashboardView>,
}

impl Default for ListDashboardsRequest {
    fn default() -> Self {
        Self {
            page_size: None,
            page_token: None,
            show_trashed: None,
            view: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListDashboardsResponse {
    #[serde(default)]
    pub dashboards: Vec<Dashboard>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PublishRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_credentials: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PublishedDashboard {
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub embed_credentials: Option<bool>,
    #[serde(default)]
    pub revision_create_time: Option<String>,
    #[serde(default)]
    pub warehouse_id: Option<String>,
}
