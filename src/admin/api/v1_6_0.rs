// Auto-generated from learnloop-api OpenAPI spec v1.6.0
// Do not edit manually — regenerated on each API release.
// Schemas: 95

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
pub struct AdminStatsResponse {
    pub total_users: i64,
    pub total_organizations: i64,
    pub total_deployments: i64,
    pub active_deployments: i64,
    pub running_deployments: i64,
    pub total_simulations: i64,
    pub organizations_by_plan: std::collections::HashMap<String, i64>,
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
pub struct BackupDeleteResponse {
    pub deleted: bool,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub name: String,
    pub size_bytes: i64,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub is_template: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRestoreResponse {
    pub restoring: bool,
    pub backup: String,
    pub deployment_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupsListResponse {
    pub backups: Vec<BackupInfo>,
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
pub struct Body_upload_backup_deploy__deployment_id__backups_upload_post {
    pub file: String,
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
pub struct CompatCheckResponse {
    pub compatible: bool,
    pub current_version: String,
    pub status: String,
    pub latest_version: String,
    pub min_supported: String,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerHealth {
    pub service: String,
    pub status: String,
    #[serde(default)]
    pub health: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerMetrics {
    pub service: String,
    #[serde(default)]
    pub cpu_percent: Option<String>,
    #[serde(default)]
    pub memory_usage: Option<String>,
    #[serde(default)]
    pub memory_limit: Option<String>,
    #[serde(default)]
    pub memory_percent: Option<String>,
    #[serde(default)]
    pub net_io: Option<String>,
    #[serde(default)]
    pub block_io: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrganizationRequest {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyUsageEntry {
    pub id: String,
    pub organization_id: String,
    pub usage_date: serde_json::Value,
    pub simulation_count: i64,
    #[serde(default)]
    pub attempts_started: Option<i64>,
    #[serde(default)]
    pub attempts_completed: Option<i64>,
    #[serde(default)]
    pub outcomes: Option<i64>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyUsageResponse {
    pub daily: Vec<DailyUsageEntry>,
    pub total_simulations: i64,
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
pub struct DeployHealthResponse {
    pub deployment_id: String,
    pub containers: Vec<ContainerHealth>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployListResponse {
    pub deployments: Vec<DeploymentResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployLogsResponse {
    pub deployment_id: String,
    pub service: String,
    pub lines: i64,
    pub logs: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployMetricsResponse {
    pub deployment_id: String,
    pub containers: Vec<ContainerMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployRequest {
    pub organization_id: String,
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
pub struct DeployVersionUpdateResponse {
    pub deployment: DeploymentResponse,
    pub redeploying: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployVersionsResponse {
    pub versions: Vec<VersionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentEventResponse {
    pub id: String,
    pub deployment_id: String,
    pub event_type: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
    #[serde(default)]
    pub actor: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentEventsListResponse {
    pub events: Vec<DeploymentEventResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentResponse {
    pub id: String,
    #[serde(default)]
    pub organization_id: Option<String>,
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
pub struct InviteAcceptResponse {
    pub accepted: bool,
    pub organization_id: String,
    pub organization_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteListResponse {
    pub invites: Vec<InviteResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteMemberRequest {
    pub email: String,
    #[serde(default)]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteResponse {
    pub id: String,
    pub organization_id: String,
    pub email: String,
    pub role: String,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub claimed: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceInfo {
    pub id: String,
    #[serde(default)]
    pub number: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    pub amount_due: i64,
    pub amount_paid: i64,
    pub currency: String,
    #[serde(default)]
    pub period_start: Option<String>,
    #[serde(default)]
    pub period_end: Option<String>,
    #[serde(default)]
    pub hosted_invoice_url: Option<String>,
    #[serde(default)]
    pub pdf_url: Option<String>,
    #[serde(default)]
    pub created: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicesResponse {
    pub invoices: Vec<InvoiceInfo>,
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
pub struct OrgUsageResponse {
    pub total_simulations: i64,
    pub total_days_active: i64,
    #[serde(default)]
    pub plan: Option<String>,
    #[serde(default)]
    pub estimated_cost: Option<String>,
    #[serde(default)]
    pub meta: Option<OrgUsageMeta>,
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
    pub user_role: Option<String>,
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
pub struct PricingFeature {
    pub name: String,
    pub values: std::collections::HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingResponse {
    pub tiers: Vec<PricingTier>,
    pub features: Vec<PricingFeature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingTier {
    pub id: String,
    pub name: String,
    pub price: String,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub billing_period: Option<String>,
    pub description: String,
    pub cta: String,
    #[serde(default)]
    pub highlighted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionClaimResponse {
    pub token: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub exp: i64,
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
    #[serde(default)]
    pub api_key_id: Option<String>,
    #[serde(default)]
    pub oauth_client_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMemberRoleRequest {
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMemberRoleResponse {
    pub updated: bool,
    pub user_id: String,
    pub role: String,
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
pub struct UsageCheckResponse {
    pub authorized: bool,
    #[serde(default)]
    pub num_left: Option<i64>,
    pub num_to_next_check: i64,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageEntryResponse {
    pub id: String,
    pub organization_id: String,
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
    pub organization_id: String,
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
    #[serde(default)]
    pub meta: Option<OrgUsageMeta>,
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
    #[serde(default)]
    pub published_at: Option<String>,
    #[serde(default)]
    pub prerelease: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionUpdateRequest {
    pub version: String,
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

// ── API Keys ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    pub organization_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub scopes: Option<Vec<String>>,
    #[serde(default)]
    pub spend_limit_cents: Option<i64>,
    #[serde(default)]
    pub environment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyResponse {
    pub id: String,
    #[serde(default)]
    pub key: Option<String>,
    pub name: String,
    pub key_prefix: String,
    #[serde(default)]
    pub scopes: Option<Vec<String>>,
    #[serde(default)]
    pub spend_limit_cents: Option<i64>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyListResponse {
    pub keys: Vec<ApiKeyResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyRevokeResponse {
    pub revoked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyUsageResponse {
    pub total_requests: i64,
    pub total_tokens: i64,
    pub total_cost_cents: i64,
    #[serde(default)]
    pub by_model: Option<serde_json::Value>,
}

// ── OAuth Clients ────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOAuthClientRequest {
    pub organization_id: String,
    pub name: String,
    pub redirect_uris: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthClientResponse {
    pub id: String,
    pub client_id: String,
    #[serde(default)]
    pub client_secret: Option<String>,
    pub name: String,
    #[serde(default)]
    pub redirect_uris: Option<Vec<String>>,
    #[serde(default)]
    pub scopes: Option<Vec<String>>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthClientListResponse {
    pub clients: Vec<OAuthClientResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOAuthClientRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub redirect_uris: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthClientRevokeResponse {
    pub revoked: bool,
}

// ── AI Gateway ───────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiModelInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub model_type: String,
    #[serde(default)]
    pub params: Option<String>,
    pub description: String,
    #[serde(default)]
    pub provider: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiTier {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cost: String,
    pub models: Vec<AiModelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiPricingResponse {
    pub tiers: Vec<AiTier>,
    #[serde(default)]
    pub rate_limits: Option<serde_json::Value>,
}

// ── Org Usage (outcome-based) ────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgUsageMeta {
    #[serde(default)]
    pub total_outcomes: Option<i64>,
    #[serde(default)]
    pub attempts_completed: Option<i64>,
    #[serde(default)]
    pub discount_pct: Option<i64>,
    #[serde(default)]
    pub gross: Option<String>,
    #[serde(default)]
    pub discount: Option<String>,
    #[serde(default)]
    pub net: Option<String>,
}
