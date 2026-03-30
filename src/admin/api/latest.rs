// Auto-generated from LearnLoop API OpenAPI spec v2.0.8
// Do not edit manually — regenerated on each API release.
// Schemas: 95

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModelEntry {
    pub id: String,
    #[serde(default)]
    pub object: Option<String>,
    #[serde(default)]
    pub created: Option<i64>,
    #[serde(default)]
    pub owned_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModelsResponse {
    pub data: Vec<AIModelEntry>,
    #[serde(default)]
    pub object: Option<String>,
}

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
pub struct ApiKeyCreateResponse {
    pub id: String,
    pub key: String,
    pub name: String,
    pub key_prefix: String,
    pub scopes: Vec<String>,
    #[serde(default)]
    pub spend_limit_cents: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyListResponse {
    pub keys: Vec<ApiKeyResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyResponse {
    pub id: String,
    pub name: String,
    pub key_prefix: String,
    pub scopes: Vec<String>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub spend_limit_cents: Option<i64>,
    #[serde(default)]
    pub spent_cents: Option<i64>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub last_used_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyRevokeResponse {
    pub revoked: bool,
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
pub struct AuthorizeCodeResponse {
    pub code: String,
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
    pub created_at: Option<serde_json::Value>,
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
    #[serde(default)]
    pub client_id: Option<String>,
    #[serde(default)]
    pub client_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_upload_backup_deploy__deployment_id__backups_upload_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChoice {
    pub index: i64,
    pub message: ChatCompletionMessage,
    #[serde(default)]
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionMessage {
    pub role: String,
    #[serde(default)]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChatCompletionChoice>,
    #[serde(default)]
    pub object: Option<String>,
    #[serde(default)]
    pub usage: Option<ChatCompletionUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionUsage {
    #[serde(default)]
    pub prompt_tokens: Option<i64>,
    #[serde(default)]
    pub completion_tokens: Option<i64>,
    #[serde(default)]
    pub total_tokens: Option<i64>,
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
pub struct CreateOAuthClientRequest {
    pub organization_id: String,
    pub name: String,
    pub redirect_uris: Vec<String>,
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
    pub version: String,
    #[serde(default)]
    pub base_domain: Option<String>,
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
    #[serde(default)]
    pub api_key_id: Option<String>,
    #[serde(default)]
    pub oauth_client_id: Option<String>,
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
    pub api_key_id: Option<String>,
    #[serde(default)]
    pub oauth_client_id: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayModelInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(default)]
    pub params: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub provider: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayPricingResponse {
    pub tiers: Vec<GatewayTier>,
    pub rate_limits: GatewayRateLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayRateLimitEntry {
    pub rpm: i64,
    pub rpd: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayRateLimits {
    pub description: String,
    pub plans: std::collections::HashMap<String, std::collections::HashMap<String, GatewayRateLimitEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayTier {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cost: String,
    pub models: Vec<GatewayModelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPValidationError {
    #[serde(default)]
    pub detail: Option<Vec<ValidationError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceInfo {
    pub id: String,
    pub amount_due: i64,
    pub amount_paid: i64,
    pub currency: String,
    #[serde(default)]
    pub number: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
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
pub struct JWKEntry {
    pub kty: String,
    #[serde(rename = "use")]
    pub r#use: String,
    pub alg: String,
    pub kid: String,
    pub n: String,
    pub e: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JWKSResponse {
    pub keys: Vec<JWKEntry>,
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
pub struct OAuthClientCreateResponse {
    pub id: String,
    pub client_id: String,
    pub client_secret: String,
    pub name: String,
    pub redirect_uris: Vec<String>,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthClientListResponse {
    pub clients: Vec<OAuthClientResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthClientResponse {
    pub id: String,
    pub client_id: String,
    pub name: String,
    pub redirect_uris: Vec<String>,
    #[serde(default)]
    pub client_secret_prefix: Option<String>,
    #[serde(default)]
    pub scopes: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub organization_id: Option<String>,
    #[serde(default)]
    pub created_by: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthClientRevokeResponse {
    pub revoked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OIDCDiscoveryResponse {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
    pub jwks_uri: String,
    pub response_types_supported: Vec<String>,
    pub grant_types_supported: Vec<String>,
    pub subject_types_supported: Vec<String>,
    pub id_token_signing_alg_values_supported: Vec<String>,
    pub scopes_supported: Vec<String>,
    pub token_endpoint_auth_methods_supported: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgUsageResponse {
    pub total_simulations: i64,
    pub total_days_active: i64,
    #[serde(default)]
    pub plan: Option<String>,
    #[serde(default)]
    pub estimated_cost: Option<String>,
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
    pub description: String,
    pub cta: String,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub billing_period: Option<String>,
    #[serde(default)]
    pub highlighted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionClaimResponse {
    pub deployment_token: String,
    pub org_id: String,
    #[serde(default)]
    pub deployment_id: Option<String>,
    #[serde(default)]
    pub plan: Option<String>,
    #[serde(default)]
    pub secret_key: Option<String>,
    #[serde(default)]
    pub auth_secret: Option<String>,
    #[serde(default)]
    pub auth_config: Option<serde_json::Value>,
    #[serde(default)]
    pub ai_config: Option<serde_json::Value>,
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
pub struct TokenResponse {
    pub access_token: String,
    pub id_token: String,
    pub token_type: String,
    pub expires_in: i64,
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
pub struct UpdateOAuthClientRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub redirect_uris: Option<Vec<String>>,
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
    #[serde(default)]
    pub started: Option<i64>,
    #[serde(default)]
    pub completed: Option<i64>,
    #[serde(default)]
    pub passed: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageCheckResponse {
    pub authorized: bool,
    pub num_to_next_check: i64,
    #[serde(default)]
    pub num_left: Option<i64>,
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
    pub simulation_count: i64,
    #[serde(default)]
    pub previous_hash: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserinfoResponse {
    pub sub: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
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
    #[serde(default)]
    pub compat: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionUpdateRequest {
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookResponse {
    #[serde(default)]
    pub status: Option<String>,
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
