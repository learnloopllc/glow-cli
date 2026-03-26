// Auto-generated from learnloop-api OpenAPI spec v1.3.2
// Do not edit manually — regenerated on each API release.
// Schemas: 53

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddMemberRequest {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddMemberResponse {
    pub added: bool,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignedResponse {
    pub assigned: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthCallbackResponse {
    pub access_token: String,
    pub token_type: String,
    #[serde(default)]
    pub expires_in: Option<i64>,
    #[serde(default)]
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthMeResponse {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub is_superadmin: bool,
    #[serde(default)]
    pub org_id: Option<String>,
    #[serde(default)]
    pub org_role: Option<String>,
    #[serde(default)]
    pub idp: Option<String>,
    #[serde(default)]
    pub github_username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizeRequest {
    pub user_id: String,
    pub email: String,
    pub name: String,
    #[serde(default)]
    pub nonce: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub name: String,
    pub size_bytes: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingPortalResponse {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingStatusResponse {
    pub subscribed: bool,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub stripe_subscription_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_token_token_post {
    pub grant_type: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutRequest {
    pub organization_id: String,
    #[serde(default)]
    pub plan: Option<String>,
    #[serde(default)]
    pub success_url: Option<String>,
    #[serde(default)]
    pub cancel_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutResponse {
    pub url: String,
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimRequest {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigUpdateRequest {
    pub content: String,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLicenseRequest {
    pub key: String,
    pub expiry: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrganizationRequest {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedResponse {
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployCallbackResponse {
    pub status: String,
    pub deployment_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployConfigCommit {
    #[serde(default)]
    pub sha: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub html_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployConfigGetResponse {
    #[serde(default)]
    pub config: Option<String>,
    #[serde(default)]
    pub sha: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployConfigUpdateResponse {
    #[serde(default)]
    pub commit: Option<DeployConfigCommit>,
    pub redeploy_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployCreateResponse {
    pub deployment: DeploymentResponse,
    #[serde(default)]
    pub repo: Option<serde_json::Value>,
    #[serde(default)]
    pub invite: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployCredentialsResponse {
    pub client_id: String,
    pub client_secret: String,
    pub auth_secret: String,
    pub auth_url: String,
    pub realm: String,
    pub issuer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployListResponse {
    pub deployments: Vec<DeploymentResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployRequest {
    pub license_id: String,
    pub deployment_name: String,
    pub subdomain: String,
    #[serde(default)]
    pub base_domain: Option<String>,
    pub version: String,
    #[serde(default)]
    pub component_type: Option<String>,
    #[serde(default)]
    pub parent_deployment_id: Option<String>,
    #[serde(default)]
    pub origin: Option<String>,
    #[serde(default)]
    pub client_origins: Option<Vec<String>>,
    #[serde(default)]
    pub app_prefix: Option<String>,
    #[serde(default)]
    pub db_backup: Option<String>,
    #[serde(default)]
    pub hosting_type: Option<String>,
    #[serde(default)]
    pub airgapped: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployStatusResponse {
    pub deployment: DeploymentResponse,
    #[serde(default)]
    pub workflow_run: Option<WorkflowRunResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployVersionsResponse {
    pub versions: Vec<VersionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentResponse {
    pub id: String,
    #[serde(default)]
    pub license_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub subdomain: Option<String>,
    #[serde(default)]
    pub base_domain: Option<String>,
    #[serde(default)]
    pub port: Option<i64>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub health: Option<String>,
    #[serde(default)]
    pub hosting_type: Option<String>,
    #[serde(default)]
    pub deployed_sha: Option<String>,
    #[serde(default)]
    pub component_type: Option<String>,
    #[serde(default)]
    pub parent_deployment_id: Option<String>,
    #[serde(default)]
    pub github_repo_name: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub origin: Option<String>,
    #[serde(default)]
    pub client_origins: Option<Vec<String>>,
    #[serde(default)]
    pub app_prefix: Option<String>,
    #[serde(default)]
    pub secret_key: Option<String>,
    #[serde(default)]
    pub airgapped: Option<bool>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPValidationError {
    #[serde(default)]
    pub detail: Option<Vec<ValidationError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseListResponse {
    pub licenses: Vec<LicenseResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseResponse {
    pub id: String,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub expiry: Option<String>,
    #[serde(default)]
    pub plan: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseValidateValidResponse {
    pub valid: bool,
    #[serde(default)]
    pub license: Option<LicenseResponse>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberListResponse {
    pub members: Vec<MemberResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(default)]
    pub is_superadmin: Option<bool>,
    #[serde(default)]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgLicenseResponse {
    pub plan: String,
    #[serde(default)]
    pub license: Option<LicenseResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationListResponse {
    pub organizations: Vec<OrganizationResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationResponse {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanInfo {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub pricing: Option<String>,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub included: Option<i64>,
    #[serde(default)]
    pub overage_unit_price: Option<String>,
    #[serde(default)]
    pub daily_limit: Option<i64>,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub billing: Option<String>,
    #[serde(default)]
    pub stripe_checkout: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlansResponse {
    pub plans: Vec<PlanInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemovedResponse {
    pub removed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootResponse {
    pub service: String,
    pub version: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetLicenseRequest {
    pub license_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDeploymentRequest {
    #[serde(default)]
    pub subdomain: Option<String>,
    #[serde(default)]
    pub base_domain: Option<String>,
    #[serde(default)]
    pub origin: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub client_origins: Option<Vec<String>>,
    #[serde(default)]
    pub app_prefix: Option<String>,
    #[serde(default)]
    pub hosting_type: Option<String>,
    #[serde(default)]
    pub airgapped: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLicenseRequest {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub expiry: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrganizationRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageCheckRequest {
    pub current_sequence: i64,
    pub current_hash: String,
    #[serde(default)]
    pub attempts_since_last_check: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageEntryResponse {
    pub id: String,
    pub license_id: String,
    pub entry_hash: String,
    #[serde(default)]
    pub previous_hash: Option<String>,
    pub simulation_count: i64,
    #[serde(default)]
    pub reported_to_stripe: Option<bool>,
    #[serde(default)]
    pub seq: Option<i64>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageReportRequest {
    pub license_id: String,
    pub entry_hash: String,
    pub simulation_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageSummaryResponse {
    pub total_simulations: i64,
    #[serde(default)]
    pub free_tier: Option<i64>,
    #[serde(default)]
    pub free_remaining: Option<i64>,
    #[serde(default)]
    pub overage_simulations: Option<i64>,
    #[serde(default)]
    pub unreported_simulations: Option<i64>,
    #[serde(default)]
    pub plan: Option<String>,
    #[serde(default)]
    pub daily_limit: Option<i64>,
    #[serde(default)]
    pub estimated_cost: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateLicenseRequest {
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub loc: Vec<serde_json::Value>,
    pub msg: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub name: String,
    pub sha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRunResponse {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub conclusion: Option<String>,
    #[serde(default)]
    pub html_url: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}
