// admin/mod.rs — HTTP client for the LearnLoop central API
//
// Handles: auth, licensing, billing, organizations, usage, deploy

pub mod api;
pub mod types;

use anyhow::Result;
use reqwest::blocking;
use serde_json::json;

use crate::api_common::{api_request, Auth};

pub struct AdminClient {
    base_url: String,
    http: blocking::Client,
    license_key: Option<String>,
    token: Option<String>,
}

impl AdminClient {
    pub fn new(base_url: &str, license_key: Option<&str>) -> Self {
        let token = crate::auth::get_token(base_url)
            .ok()
            .map(|t| t.access_token);

        AdminClient {
            base_url: base_url.trim_end_matches('/').to_string(),
            http: blocking::Client::new(),
            license_key: license_key.map(|s| s.to_string()),
            token,
        }
    }

    #[cfg(test)]
    fn new_with_token(base_url: &str, token: &str) -> Self {
        AdminClient {
            base_url: base_url.trim_end_matches('/').to_string(),
            http: blocking::Client::new(),
            license_key: None,
            token: Some(token.to_string()),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn bearer_auth(&self) -> Auth<'_> {
        match &self.token {
            Some(t) => Auth::Bearer(t),
            None => Auth::None,
        }
    }

    fn license_auth(&self) -> Auth<'_> {
        match &self.license_key {
            Some(k) => Auth::LicenseKey(k),
            None => Auth::None,
        }
    }

    // ── Auth ────────────────────────────────────────────────

    pub fn whoami(&self) -> Result<types::WhoamiResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url("/auth/me"),
            None,
            self.bearer_auth(),
        )
    }

    // ── Licenses ────────────────────────────────────────────

    pub fn validate_license(&self) -> Result<types::ValidateLicenseResponse> {
        let key = self.license_key.as_deref().ok_or_else(|| {
            anyhow::anyhow!("No license key set. Use --license-key or set LEARNLOOP_LICENSE_KEY.")
        })?;
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/licenses/validate"),
            Some(json!({ "key": key })),
            self.bearer_auth(),
        )
    }

    pub fn license_list(&self, active_only: bool) -> Result<types::LicenseListResponse> {
        let query = if active_only { "?active_only=true" } else { "" };
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/licenses{}", query)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn license_create(&self, key: &str, expiry: &str) -> Result<types::LicenseInfo> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/licenses"),
            Some(json!({ "key": key, "expiry": expiry })),
            self.bearer_auth(),
        )
    }

    pub fn license_update(
        &self,
        license_id: &str,
        key: Option<&str>,
        expiry: Option<&str>,
        active: Option<bool>,
    ) -> Result<types::LicenseInfo> {
        let mut body = json!({});
        if let Some(k) = key {
            body["key"] = json!(k);
        }
        if let Some(e) = expiry {
            body["expiry"] = json!(e);
        }
        if let Some(a) = active {
            body["active"] = json!(a);
        }
        api_request(
            &self.http,
            reqwest::Method::PUT,
            &self.url(&format!("/licenses/{}", license_id)),
            Some(body),
            self.bearer_auth(),
        )
    }

    pub fn license_delete(&self, license_id: &str) -> Result<types::LicenseDeleteResponse> {
        api_request(
            &self.http,
            reqwest::Method::DELETE,
            &self.url(&format!("/licenses/{}", license_id)),
            None,
            self.bearer_auth(),
        )
    }

    // ── Organizations ───────────────────────────────────────

    pub fn org_list(&self) -> Result<types::OrgListResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url("/organizations"),
            None,
            self.bearer_auth(),
        )
    }

    pub fn org_create(
        &self,
        name: &str,
        description: Option<&str>,
    ) -> Result<types::Organization> {
        let mut body = json!({ "name": name });
        if let Some(d) = description {
            body["description"] = json!(d);
        }
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/organizations"),
            Some(body),
            self.bearer_auth(),
        )
    }

    pub fn org_get(&self, org_id: &str) -> Result<types::Organization> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/organizations/{}", org_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn org_update(
        &self,
        org_id: &str,
        name: Option<&str>,
        description: Option<&str>,
        active: Option<bool>,
    ) -> Result<types::Organization> {
        let mut body = json!({});
        if let Some(n) = name {
            body["name"] = json!(n);
        }
        if let Some(d) = description {
            body["description"] = json!(d);
        }
        if let Some(a) = active {
            body["active"] = json!(a);
        }
        api_request(
            &self.http,
            reqwest::Method::PUT,
            &self.url(&format!("/organizations/{}", org_id)),
            Some(body),
            self.bearer_auth(),
        )
    }

    pub fn org_delete(&self, org_id: &str) -> Result<types::OrgDeleteResponse> {
        api_request(
            &self.http,
            reqwest::Method::DELETE,
            &self.url(&format!("/organizations/{}", org_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn org_members(&self, org_id: &str) -> Result<types::OrgMembersResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/organizations/{}/members", org_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn org_member_add(
        &self,
        org_id: &str,
        email: &str,
    ) -> Result<types::OrgMemberAddResponse> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url(&format!("/organizations/{}/members", org_id)),
            Some(json!({ "email": email })),
            self.bearer_auth(),
        )
    }

    pub fn org_member_remove(
        &self,
        org_id: &str,
        user_id: &str,
    ) -> Result<types::OrgMemberRemoveResponse> {
        api_request(
            &self.http,
            reqwest::Method::DELETE,
            &self.url(&format!("/organizations/{}/members/{}", org_id, user_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn org_license(&self, org_id: &str) -> Result<types::OrgLicenseResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/organizations/{}/license", org_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn org_license_set(
        &self,
        org_id: &str,
        license_id: &str,
    ) -> Result<types::OrgLicenseSetResponse> {
        api_request(
            &self.http,
            reqwest::Method::PUT,
            &self.url(&format!("/organizations/{}/license", org_id)),
            Some(json!({ "license_id": license_id })),
            self.bearer_auth(),
        )
    }

    pub fn org_license_remove(&self, org_id: &str) -> Result<types::OrgLicenseRemoveResponse> {
        api_request(
            &self.http,
            reqwest::Method::DELETE,
            &self.url(&format!("/organizations/{}/license", org_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn org_deployments(&self, org_id: &str) -> Result<types::OrgDeploymentsResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/organizations/{}/deployments", org_id)),
            None,
            self.bearer_auth(),
        )
    }

    // ── Deploy ───────────────────────────────────────────────

    pub fn deploy(
        &self,
        license_id: &str,
        deployment_name: &str,
        subdomain: &str,
        base_domain: Option<&str>,
        private: bool,
    ) -> Result<types::DeployResponse> {
        let mut body = json!({
            "license_id": license_id,
            "deployment_name": deployment_name,
            "subdomain": subdomain,
            "private": private,
        });
        if let Some(bd) = base_domain {
            body["base_domain"] = json!(bd);
        }
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/deploy"),
            Some(body),
            self.bearer_auth(),
        )
    }

    pub fn deploy_stop(&self, deployment_id: &str) -> Result<types::Deployment> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url(&format!("/deploy/{}/stop", deployment_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn deploy_destroy(&self, deployment_id: &str) -> Result<types::Deployment> {
        api_request(
            &self.http,
            reqwest::Method::DELETE,
            &self.url(&format!("/deploy/{}", deployment_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn deploy_list(&self, active_only: bool) -> Result<types::DeployListResponse> {
        let query = if !active_only {
            "?active_only=false"
        } else {
            ""
        };
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/deploy{}", query)),
            None,
            self.bearer_auth(),
        )
    }

    // ── Usage ───────────────────────────────────────────────

    pub fn usage_report(
        &self,
        license_id: &str,
        entry_hash: &str,
        simulation_count: i64,
    ) -> Result<types::UsageReportResponse> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/usage/report"),
            Some(json!({
                "license_id": license_id,
                "entry_hash": entry_hash,
                "simulation_count": simulation_count
            })),
            self.license_auth(),
        )
    }

    pub fn usage_summary(&self, license_id: &str) -> Result<types::UsageSummaryResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/usage/summary/{}", license_id)),
            None,
            self.bearer_auth(),
        )
    }

    // ── Billing ─────────────────────────────────────────────

    pub fn billing_plans(&self) -> Result<types::BillingPlansResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url("/billing/plans"),
            None,
            Auth::None,
        )
    }

    pub fn billing_status(&self, org_id: &str) -> Result<types::BillingStatusResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/billing/status/{}", org_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn billing_checkout(
        &self,
        org_id: &str,
        plan: &str,
    ) -> Result<types::BillingCheckoutResponse> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/billing/checkout"),
            Some(json!({ "organization_id": org_id, "plan": plan })),
            self.bearer_auth(),
        )
    }

    pub fn billing_portal(&self, org_id: &str) -> Result<types::BillingPortalResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/billing/portal/{}", org_id)),
            None,
            self.bearer_auth(),
        )
    }
}

// ── Tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whoami_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/auth/me")
            .match_header("Authorization", "Bearer test-token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"user_id": "u-1", "email": "a@b.com", "name": "Alice", "is_superadmin": false, "org_id": "org-1", "org_role": "admin", "idp": null, "github_username": null}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "test-token");
        let resp = client.whoami().unwrap();
        assert_eq!(resp.email, "a@b.com");
        assert_eq!(resp.org_role, Some("admin".into()));
        mock.assert();
    }

    #[test]
    fn test_whoami_unauthorized() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("GET", "/auth/me")
            .with_status(401)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "bad-token");
        let err = client.whoami().unwrap_err();
        assert!(err.to_string().contains("Authentication failed"));
    }

    #[test]
    fn test_license_list_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/licenses")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"licenses": [{"id": "lic-1", "key": "abc-123", "expiry": "2026-12-31", "active": true}]}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.license_list(false).unwrap();
        assert_eq!(resp.licenses.len(), 1);
        assert_eq!(resp.licenses[0].id, "lic-1");
        mock.assert();
    }

    #[test]
    fn test_license_create_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/licenses")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": "lic-new", "key": "new-key", "expiry": "2027-01-01", "active": true}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.license_create("new-key", "2027-01-01").unwrap();
        assert_eq!(resp.id, "lic-new");
        mock.assert();
    }

    #[test]
    fn test_license_delete_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("DELETE", "/licenses/lic-1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"deleted": true}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.license_delete("lic-1").unwrap();
        assert!(resp.deleted);
        mock.assert();
    }

    #[test]
    fn test_org_list_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/organizations")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"organizations": [{"id": "org-1", "name": "Acme", "description": null, "active": true}]}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.org_list().unwrap();
        assert_eq!(resp.organizations[0].name, "Acme");
        mock.assert();
    }

    #[test]
    fn test_org_create_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/organizations")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": "org-new", "name": "NewOrg", "description": "test", "active": true}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.org_create("NewOrg", Some("test")).unwrap();
        assert_eq!(resp.id, "org-new");
        mock.assert();
    }

    #[test]
    fn test_org_delete_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("DELETE", "/organizations/org-1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"deleted": true}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.org_delete("org-1").unwrap();
        assert!(resp.deleted);
        mock.assert();
    }

    #[test]
    fn test_org_members_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/organizations/org-1/members")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"members": [{"id": "u-1", "name": "Alice", "email": "a@b.com", "is_superadmin": false, "role": "admin"}]}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.org_members("org-1").unwrap();
        assert_eq!(resp.members.len(), 1);
        assert_eq!(resp.members[0].role, Some("admin".into()));
        mock.assert();
    }

    #[test]
    fn test_deploy_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/deploy")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"deployment": {"id": "d-1", "name": "my-glow", "subdomain": "my", "base_domain": "learn-loop.org", "domain": "my.learn-loop.org", "status": "pending", "health": "unknown", "hosting_type": "hosted", "active": true}, "repo": {"html_url": "https://github.com/learnloopllc/my-glow"}, "invite": null}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.deploy("lic-1", "my-glow", "my", None, true).unwrap();
        assert_eq!(resp.deployment.name, Some("my-glow".into()));
        assert_eq!(resp.deployment.status, Some("pending".into()));
        mock.assert();
    }

    #[test]
    fn test_deploy_stop_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/deploy/d-1/stop")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": "d-1", "name": "my-glow", "subdomain": "my", "domain": "my.learn-loop.org", "status": "stopped", "health": "unknown", "active": false}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.deploy_stop("d-1").unwrap();
        assert_eq!(resp.status, Some("stopped".into()));
        mock.assert();
    }

    #[test]
    fn test_deploy_destroy_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("DELETE", "/deploy/d-1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": "d-1", "name": "my-glow", "subdomain": "my", "domain": "my.learn-loop.org", "status": "destroyed", "health": "unknown", "active": false}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.deploy_destroy("d-1").unwrap();
        assert_eq!(resp.status, Some("destroyed".into()));
        assert_eq!(resp.active, Some(false));
        mock.assert();
    }

    #[test]
    fn test_deploy_destroy_not_stopped_returns_error() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("DELETE", "/deploy/d-1")
            .with_status(400)
            .with_body(r#"{"detail":"Deployment must be stopped before destroying"}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let err = client.deploy_destroy("d-1").unwrap_err();
        assert!(err.to_string().contains("400"));
    }

    #[test]
    fn test_deploy_list_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/deploy")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"deployments": [{"id": "d-1", "name": "my-glow", "subdomain": "my", "base_domain": "learn-loop.org", "domain": "my.learn-loop.org", "status": "running", "health": "healthy", "hosting_type": "hosted", "active": true}]}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.deploy_list(true).unwrap();
        assert_eq!(resp.deployments.len(), 1);
        assert_eq!(resp.deployments[0].name, Some("my-glow".into()));
        mock.assert();
    }

    #[test]
    fn test_deploy_list_all_includes_destroyed() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/deploy?active_only=false")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"deployments": [{"id": "d-1", "name": "a", "status": "running", "active": true}, {"id": "d-2", "name": "b", "status": "destroyed", "active": false}]}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.deploy_list(false).unwrap();
        assert_eq!(resp.deployments.len(), 2);
        mock.assert();
    }

    #[test]
    fn test_org_member_add_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/organizations/org-1/members")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"added": true, "email": "new@b.com"}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.org_member_add("org-1", "new@b.com").unwrap();
        assert!(resp.added);
        mock.assert();
    }

    #[test]
    fn test_usage_summary_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/usage/summary/lic-1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"total_simulations": 25, "free_tier": 10, "free_remaining": 0, "overage_simulations": 15, "estimated_cost": "$1.50"}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.usage_summary("lic-1").unwrap();
        assert_eq!(resp.total_simulations, 25);
        assert_eq!(resp.estimated_cost, Some("$1.50".into()));
        mock.assert();
    }

    #[test]
    fn test_billing_plans_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/billing/plans")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"plans": [{"id": "pro", "name": "LearnLoop Pro", "pricing": "$49/mo", "included": 10, "overage_unit_price": "$0.10"}]}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.billing_plans().unwrap();
        assert_eq!(resp.plans[0].id, "pro");
        mock.assert();
    }

    #[test]
    fn test_billing_status_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/billing/status/org-1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"subscribed": true, "status": "active", "stripe_subscription_id": "sub_123"}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.billing_status("org-1").unwrap();
        assert!(resp.subscribed);
        mock.assert();
    }

    #[test]
    fn test_billing_checkout_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/billing/checkout")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"url": "https://checkout.stripe.com/xxx", "session_id": "cs_123"}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.billing_checkout("org-1", "pro").unwrap();
        assert!(resp.url.contains("stripe"));
        mock.assert();
    }

    #[test]
    fn test_billing_portal_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/billing/portal/org-1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"url": "https://billing.stripe.com/yyy"}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.billing_portal("org-1").unwrap();
        assert!(resp.url.contains("stripe"));
        mock.assert();
    }
}
