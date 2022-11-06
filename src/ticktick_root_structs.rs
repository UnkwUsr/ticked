// by https://transform.tools/json-to-rust-serde

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub check_point: i64,
    pub sync_task_bean: SyncTaskBean,
    pub project_profiles: Vec<ProjectProfile>,
    pub project_groups: Vec<ProjectGroup>,
    pub filters: Value,
    pub tags: Vec<Tag>,
    pub sync_task_order_bean: Value,
    pub sync_order_bean: Value,
    pub inbox_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncTaskBean {
    pub update: Vec<Task>,
    pub delete: Vec<Task>,
    pub add: Vec<Task>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub sort_order: i64,
    pub title: String,
    pub content: Option<String>,
    pub time_zone: String,
    pub is_floating: bool,
    pub is_all_day: Option<bool>,
    pub reminder: Option<String>,
    pub reminders: Vec<Value>,
    #[serde(default)]
    pub ex_date: Vec<Value>,
    pub repeat_task_id: Option<String>,
    pub priority: i64,
    pub status: i64,
    pub items: Vec<Item>,
    pub progress: Option<i64>,
    pub modified_time: String,
    pub etag: String,
    pub deleted: i64,
    pub created_time: String,
    pub creator: i64,
    pub repeat_from: Option<String>,
    #[serde(default)]
    pub pomodoro_summaries: Vec<Value>,
    pub column_id: Option<String>,
    pub kind: String,
    pub desc: Option<String>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    pub comment_count: Option<i64>,
    #[serde(default)]
    pub focus_summaries: Vec<Value>,
    pub repeat_flag: Option<String>,
    pub repeat_first_date: Option<String>,
    pub parent_id: Option<String>,
    #[serde(default)]
    pub tags: Vec<Value>,
    #[serde(default)]
    pub child_ids: Vec<String>,
    pub img_mode: Option<i64>,
    pub completed_time: Option<String>,
    pub completed_user_id: Option<i64>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub status: i64,
    pub title: String,
    pub sort_order: i64,
    pub start_date: Value,
    pub is_all_day: bool,
    pub time_zone: String,
    pub snooze_reminder_time: Value,
    pub completed_time: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub id: String,
    pub ref_id: String,
    // pub path: String,
    pub size: i64,
    pub file_name: String,
    pub file_type: String,
    pub created_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectProfile {
    pub id: String,
    pub name: String,
    pub is_owner: bool,
    pub color: Option<String>,
    pub in_all: bool,
    pub sort_order: i64,
    pub sort_type: String,
    pub user_count: i64,
    pub etag: String,
    pub modified_time: String,
    pub closed: Option<bool>,
    pub muted: bool,
    pub transferred: Value,
    pub group_id: Option<String>,
    pub view_mode: String,
    pub notification_options: Vec<Value>,
    pub team_id: Value,
    pub permission: Option<String>,
    pub kind: String,
    pub timeline: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectGroup {
    pub id: String,
    pub etag: String,
    pub name: String,
    pub show_all: bool,
    pub sort_order: i64,
    pub view_mode: Value,
    pub deleted: i64,
    pub user_id: i64,
    pub sort_type: String,
    pub team_id: Value,
    pub timeline: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    pub raw_name: String,
    pub label: String,
    pub sort_order: i64,
    pub sort_type: String,
    pub color: String,
    pub etag: String,
}
