use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Job types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    #[serde(default)]
    pub job_id: Option<i64>,
    #[serde(default)]
    pub creator_user_name: Option<String>,
    #[serde(default)]
    pub settings: Option<JobSettings>,
    #[serde(default)]
    pub created_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobSettings {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub tasks: Option<Vec<Task>>,
    #[serde(default)]
    pub schedule: Option<CronSchedule>,
    #[serde(default)]
    pub max_concurrent_runs: Option<i32>,
    #[serde(default)]
    pub timeout_seconds: Option<i32>,
    #[serde(default)]
    pub email_notifications: Option<EmailNotifications>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub task_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_task: Option<NotebookTask>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_jar_task: Option<SparkJarTask>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_python_task: Option<SparkPythonTask>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_task: Option<SqlTask>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<TaskDependency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_cluster_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDependency {
    pub task_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookTask {
    pub notebook_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_parameters: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparkJarTask {
    pub main_class_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparkPythonTask {
    pub python_file: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlTask {
    pub warehouse_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<SqlQueryRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlQueryRef {
    pub query_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronSchedule {
    pub quartz_cron_expression: String,
    pub timezone_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_status: Option<PauseStatus>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PauseStatus {
    Paused,
    Unpaused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailNotifications {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_start: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_success: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateJob {
    #[serde(flatten)]
    pub settings: JobSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateJobResponse {
    pub job_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateJob {
    pub job_id: i64,
    pub new_settings: JobSettings,
}

// ============================================================================
// Run types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    #[serde(default)]
    pub run_id: Option<i64>,
    #[serde(default)]
    pub job_id: Option<i64>,
    #[serde(default)]
    pub run_name: Option<String>,
    #[serde(default)]
    pub state: Option<RunState>,
    #[serde(default)]
    pub start_time: Option<i64>,
    #[serde(default)]
    pub end_time: Option<i64>,
    #[serde(default)]
    pub setup_duration: Option<i64>,
    #[serde(default)]
    pub execution_duration: Option<i64>,
    #[serde(default)]
    pub cleanup_duration: Option<i64>,
    #[serde(default)]
    pub tasks: Option<Vec<RunTask>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunTask {
    pub task_key: String,
    #[serde(default)]
    pub run_id: Option<i64>,
    #[serde(default)]
    pub state: Option<RunState>,
    #[serde(default)]
    pub start_time: Option<i64>,
    #[serde(default)]
    pub end_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunState {
    #[serde(default)]
    pub life_cycle_state: Option<RunLifeCycleState>,
    #[serde(default)]
    pub result_state: Option<RunResultState>,
    #[serde(default)]
    pub state_message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RunLifeCycleState {
    Pending,
    Running,
    Terminating,
    Terminated,
    Skipped,
    InternalError,
    Blocked,
    WaitingForRetry,
    Queued,
}

impl RunLifeCycleState {
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            RunLifeCycleState::Terminated
                | RunLifeCycleState::Skipped
                | RunLifeCycleState::InternalError
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RunResultState {
    Success,
    Failed,
    Timedout,
    Canceled,
    MaxConcurrentRunsReached,
    ExcludedByCondition,
    SuccessWithFailures,
}

impl RunResultState {
    pub fn is_success(&self) -> bool {
        matches!(self, RunResultState::Success)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RunNow {
    pub job_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_params: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jar_params: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_params: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RunNowResponse {
    pub run_id: i64,
    #[serde(default)]
    pub number_in_job: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitRun {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_name: Option<String>,
    pub tasks: Vec<Task>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitRunResponse {
    pub run_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RepairRun {
    pub run_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rerun_tasks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rerun_all_failed_tasks: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RepairRunResponse {
    pub repair_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RunOutput {
    #[serde(default)]
    pub notebook_output: Option<NotebookOutput>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub metadata: Option<Run>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NotebookOutput {
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListJobsResponse {
    #[serde(default)]
    pub jobs: Vec<Job>,
    #[serde(default)]
    pub has_more: bool,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListRunsResponse {
    #[serde(default)]
    pub runs: Vec<Run>,
    #[serde(default)]
    pub has_more: bool,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Permission types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobPermissionLevel {
    CanManage,
    CanManageRun,
    CanView,
    IsOwner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobPermissions {
    #[serde(default)]
    pub access_control_list: Option<Vec<JobAccessControlResponse>>,
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub object_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobAccessControlResponse {
    #[serde(default)]
    pub all_permissions: Option<Vec<JobPermission>>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub service_principal_name: Option<String>,
    #[serde(default)]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobPermission {
    #[serde(default)]
    pub inherited: Option<bool>,
    #[serde(default)]
    pub inherited_from_object: Option<Vec<String>>,
    #[serde(default)]
    pub permission_level: Option<JobPermissionLevel>,
}

#[derive(Debug, Clone, Serialize)]
pub struct JobPermissionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<Vec<JobAccessControlRequest>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct JobAccessControlRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_level: Option<JobPermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JobPermissionsDescription {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub permission_level: Option<JobPermissionLevel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetJobPermissionLevelsResponse {
    #[serde(default)]
    pub permission_levels: Vec<JobPermissionsDescription>,
}

// ============================================================================
// Internal helpers
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub(crate) struct JobId {
    pub job_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct RunId {
    pub run_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ResetJob {
    pub job_id: i64,
    pub new_settings: JobSettings,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct CancelAllRuns {
    pub job_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct EmptyResponse {}
