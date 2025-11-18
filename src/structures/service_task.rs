use serde::{Deserialize, Serialize};

/// An Operaton Service Task with its description elements
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServiceTask {
    /// The external task id (Camunda/Operaton external task id)
    id: String,
    /// The id of the Service Task (called `activityId` in Operaton)
    activity_id: String,
    process_instance_id: String,
    suspended: bool,
    topic_name: String,
    priority: usize,
    business_key: Option<String>,
    worker_id: Option<String>,
}

impl ServiceTask {
    pub fn id(&self) -> &str { &self.id }

    pub fn activity_id(&self) -> &str {
        &self.activity_id
    }

    pub fn process_instance_id(&self) -> &str {
        &self.process_instance_id
    }

    pub fn suspended(&self) -> bool {
        self.suspended
    }

    pub fn topic_name(&self) -> &str {
        &self.topic_name
    }

    pub fn priority(&self) -> usize {
        self.priority
    }

    pub fn business_key(&self) -> Option<String> {
        self.business_key.clone()
    }
}