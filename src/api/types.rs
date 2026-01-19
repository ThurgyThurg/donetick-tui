use serde::{Deserialize, Serialize};
use serde_json::Value;

// Response wrapper for list endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct ChoreListResponse {
    pub res: Vec<Chore>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chore {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub frequency_type: Option<String>,
    #[serde(default)]
    pub frequency: Option<i32>,
    #[serde(default)]
    pub next_due_date: Option<String>,
    #[serde(default)]
    pub assigned_to: Option<i64>,
    #[serde(default)]
    pub assignees: Option<Vec<Assignee>>,
    #[serde(default)]
    pub is_active: Option<bool>,
    #[serde(default)]
    pub labels_v2: Option<Vec<String>>,
    #[serde(default)]
    pub circle_id: Option<i64>,
    #[serde(default)]
    pub status: Option<i32>,  // Changed to i32 from String
    #[serde(default)]
    pub priority: Option<i32>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    // Additional fields from API that we'll ignore
    #[serde(default)]
    pub is_rolling: Option<bool>,
    #[serde(default)]
    pub assign_strategy: Option<String>,
    #[serde(default)]
    pub notification: Option<bool>,
    #[serde(default)]
    pub require_approval: Option<bool>,
    #[serde(default)]
    pub is_private: Option<bool>,
    #[serde(default)]
    pub created_by: Option<i64>,
    #[serde(default)]
    pub updated_by: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency_metadata: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_metadata: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thing_chore: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Assignee {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateChoreRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
}
