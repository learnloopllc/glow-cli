// learnloop/types.rs — Response types from the LearnLoop central API
//
// We only define the fields we actually USE in the CLI.
// serde will silently ignore any extra fields in the JSON response.

use serde::{Deserialize, Serialize};

// ── Auth ─────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct WhoamiResponse {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub is_superadmin: bool,
    pub org_id: Option<String>,
    pub org_role: Option<String>,
    pub idp: Option<String>,
    pub github_username: Option<String>,
}

// ── License ──────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateLicenseResponse {
    pub valid: bool,
    pub license: Option<LicenseInfo>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub id: String,
    pub key: Option<String>,
    pub expiry: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseListResponse {
    pub licenses: Vec<LicenseInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseDeleteResponse {
    pub deleted: bool,
}

// ── Organizations ────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrgListResponse {
    pub organizations: Vec<Organization>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrgDeleteResponse {
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrgMember {
    pub id: String,
    pub name: String,
    pub email: String,
    pub is_superadmin: bool,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrgMembersResponse {
    pub members: Vec<OrgMember>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrgMemberAddResponse {
    pub added: bool,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrgMemberRemoveResponse {
    pub removed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrgLicenseResponse {
    pub plan: String,
    pub license: Option<LicenseInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrgLicenseSetResponse {
    pub assigned: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrgLicenseRemoveResponse {
    pub removed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deployment {
    pub id: String,
    pub name: Option<String>,
    pub subdomain: Option<String>,
    pub base_domain: Option<String>,
    pub domain: Option<String>,
    pub status: Option<String>,
    pub health: Option<String>,
    pub hosting_type: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrgDeploymentsResponse {
    pub deployments: Vec<Deployment>,
}

// ── Deploy ───────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployResponse {
    pub deployment: Deployment,
    pub repo: Option<serde_json::Value>,
    pub invite: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployListResponse {
    pub deployments: Vec<Deployment>,
}

// ── Usage ────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageReportResponse {
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageSummaryResponse {
    pub total_simulations: i64,
    pub free_tier: Option<i64>,
    pub free_remaining: Option<i64>,
    pub overage_simulations: Option<i64>,
    pub estimated_cost: Option<String>,
}

// ── Billing ──────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct BillingPlan {
    pub id: String,
    pub name: String,
    pub pricing: Option<String>,
    pub included: Option<i64>,
    pub overage_unit_price: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BillingPlansResponse {
    pub plans: Vec<BillingPlan>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BillingStatusResponse {
    pub subscribed: bool,
    pub status: Option<String>,
    pub stripe_subscription_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BillingCheckoutResponse {
    pub url: String,
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BillingPortalResponse {
    pub url: String,
}

// ── Tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_whoami() {
        let json = r#"{"user_id": "u-1", "email": "a@b.com", "name": "Alice", "is_superadmin": false, "org_id": "org-1", "org_role": "admin", "idp": "google", "github_username": null}"#;
        let resp: WhoamiResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.email, "a@b.com");
        assert_eq!(resp.org_role, Some("admin".into()));
        assert!(!resp.is_superadmin);
    }

    #[test]
    fn test_deserialize_license_list() {
        let json = r#"{"licenses": [{"id": "lic-1", "key": "abc", "expiry": "2026-12-31", "active": true}]}"#;
        let resp: LicenseListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.licenses.len(), 1);
        assert_eq!(resp.licenses[0].id, "lic-1");
    }

    #[test]
    fn test_deserialize_org_list() {
        let json = r#"{"organizations": [{"id": "org-1", "name": "Acme", "description": null, "active": true}]}"#;
        let resp: OrgListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.organizations[0].name, "Acme");
    }

    #[test]
    fn test_deserialize_org_members() {
        let json = r#"{"members": [{"id": "u-1", "name": "Bob", "email": "b@b.com", "is_superadmin": false, "role": "admin"}]}"#;
        let resp: OrgMembersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.members[0].role, "admin");
    }

    #[test]
    fn test_deserialize_deployment() {
        let json = r#"{"deployment": {"id": "d-1", "name": "test-glow", "subdomain": "test", "base_domain": "learn-loop.org", "domain": "test.learn-loop.org", "status": "running", "health": "healthy", "hosting_type": "hosted", "active": true}, "repo": null, "invite": null}"#;
        let resp: DeployResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.deployment.status, Some("running".into()));
    }

    #[test]
    fn test_deserialize_deploy_list() {
        let json = r#"{"deployments": [{"id": "d-1", "name": "test", "status": "running", "active": true}, {"id": "d-2", "name": "old", "status": "destroyed", "active": false}]}"#;
        let resp: DeployListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.deployments.len(), 2);
        assert_eq!(resp.deployments[1].status, Some("destroyed".into()));
    }

    #[test]
    fn test_deserialize_billing_plans() {
        let json = r#"{"plans": [{"id": "pro", "name": "LearnLoop Pro", "pricing": "$49/mo", "included": 10, "overage_unit_price": "$0.10"}]}"#;
        let resp: BillingPlansResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.plans[0].id, "pro");
        assert_eq!(resp.plans[0].included, Some(10));
    }

    #[test]
    fn test_deserialize_usage_summary() {
        let json = r#"{"total_simulations": 25, "free_tier": 10, "free_remaining": 0, "overage_simulations": 15, "estimated_cost": "$1.50"}"#;
        let resp: UsageSummaryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.total_simulations, 25);
        assert_eq!(resp.estimated_cost, Some("$1.50".into()));
    }

    #[test]
    fn test_deserialize_billing_checkout() {
        let json = r#"{"url": "https://checkout.stripe.com/xxx", "session_id": "cs_123"}"#;
        let resp: BillingCheckoutResponse = serde_json::from_str(json).unwrap();
        assert!(resp.url.contains("stripe"));
    }
}
