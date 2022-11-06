use crate::ticktick_root_structs::Task;
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::collections::VecDeque;

#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MyTask {
    pub id: String,
    pub project_id: String,
    // pub sort_order: i64,
    pub title: String,
    pub content: Option<String>,
    // pub priority: i64,
    pub status: i64,
    // pub deleted: i64,
    // pub parent_id: Option<String>,
    // #[serde(default)]
    // pub tags: Vec<Value>,
    // #[serde(default)]
    // pub child_ids: Vec<String>,
    // pub due_date: Option<String>,
}

impl From<Task> for MyTask {
    fn from(x: Task) -> Self {
        MyTask {
            id: x.id,
            project_id: x.project_id,
            title: x.title,
            content: x.content,
            status: x.status,
        }
    }
}

impl std::fmt::Display for MyTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "* {}", self.title)?;

        if let Some(content) = &self.content {
            if !content.is_empty() {
                // TODO: maybe use bufreader read_line instead of lines() ?
                let mut lines: VecDeque<&str> = content.lines().collect();
                write!(f, "\n  * {}", lines.pop_front().unwrap())?;
                if !lines.is_empty() {
                    for x in lines {
                        write!(f, "\n    {}", x)?;
                    }
                }
            }
        }

        // TODO: subtasks
        // TODO: metadata: tags

        std::fmt::Result::Ok(())
    }
}
