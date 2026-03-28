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
    token: Option<String>,
}

impl AdminClient {
    pub fn new(base_url: &str) -> Self {
        let token = crate::auth::get_token(base_url)
            .ok()
            .map(|t| t.access_token);

        AdminClient {
            base_url: base_url.trim_end_matches('/').to_string(),
            http: blocking::Client::new(),
            token,
        }
    }

    #[cfg(test)]
    fn new_with_token(base_url: &str, token: &str) -> Self {
        AdminClient {
            base_url: base_url.trim_end_matches('/').to_string(),
            http: blocking::Client::new(),
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

    // ── Auth ────────────────────────────────────────────────

    pub fn whoami(&self) -> Result<types::WhoamiResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url("/me"),
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

    pub fn org_create(&self, name: &str, description: Option<&str>) -> Result<types::Organization> {
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

    pub fn org_member_add(&self, org_id: &str, email: &str) -> Result<types::OrgMemberAddResponse> {
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

    pub fn deploy_raw(&self, body: serde_json::Value) -> Result<types::DeployResponse> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/deploy"),
            Some(body),
            self.bearer_auth(),
        )
    }

    pub fn deploy_credentials(
        &self,
        deployment_id: &str,
    ) -> Result<types::DeployCredentialsResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/deploy/{}/credentials", deployment_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn deploy_status(&self, deployment_id: &str) -> Result<types::DeployStatusResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/deploy/{}/status", deployment_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn deploy_logs(
        &self,
        deployment_id: &str,
        service: &str,
        lines: u32,
    ) -> Result<types::DeployLogsResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!(
                "/deploy/{}/logs?service={}&lines={}",
                deployment_id, service, lines
            )),
            None,
            self.bearer_auth(),
        )
    }

    pub fn deploy_health(&self, deployment_id: &str) -> Result<types::DeployHealthResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/deploy/{}/health", deployment_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn deploy_metrics(&self, deployment_id: &str) -> Result<types::DeployMetricsResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/deploy/{}/metrics", deployment_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn deploy_events(
        &self,
        deployment_id: &str,
    ) -> Result<types::DeploymentEventsListResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/deploy/{}/events", deployment_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn deploy_redeploy(&self, deployment_id: &str) -> Result<types::Deployment> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url(&format!("/deploy/{}/redeploy", deployment_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn deploy_version(
        &self,
        deployment_id: &str,
        version: &str,
    ) -> Result<types::DeployVersionUpdateResponse> {
        api_request(
            &self.http,
            reqwest::Method::PUT,
            &self.url(&format!("/deploy/{}/version", deployment_id)),
            Some(json!({ "version": version })),
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

    pub fn usage_summary(&self, org_id: &str) -> Result<types::UsageSummaryResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/usage/summary/{}", org_id)),
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

    pub fn billing_invoices(
        &self,
        org_id: &str,
        limit: u32,
    ) -> Result<types::BillingInvoicesResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/billing/invoices/{}?limit={}", org_id, limit)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn billing_pricing(&self) -> Result<types::BillingPricingResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url("/billing/pricing"),
            None,
            Auth::None,
        )
    }

    // ── Usage (daily) ────────────────────────────────────────

    pub fn usage_daily(
        &self,
        org_id: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<types::DailyUsageResponse> {
        let mut query = String::new();
        if let Some(f) = from {
            query.push_str(&format!("?from={}", f));
        }
        if let Some(t) = to {
            let sep = if query.is_empty() { '?' } else { '&' };
            query.push_str(&format!("{}to={}", sep, t));
        }
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/usage/daily/{}{}", org_id, query)),
            None,
            self.bearer_auth(),
        )
    }

    // ── API Keys ─────────────────────────────────────────────

    pub fn api_keys_create(
        &self,
        org_id: &str,
        name: Option<&str>,
        scopes: Option<Vec<String>>,
        spend_limit_cents: Option<i64>,
    ) -> Result<types::ApiKeyResponse> {
        let mut body = json!({ "organization_id": org_id });
        if let Some(n) = name {
            body["name"] = json!(n);
        }
        if let Some(s) = scopes {
            body["scopes"] = json!(s);
        }
        if let Some(l) = spend_limit_cents {
            body["spend_limit_cents"] = json!(l);
        }
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/api-keys"),
            Some(body),
            self.bearer_auth(),
        )
    }

    pub fn api_keys_list(&self, org_id: &str) -> Result<types::ApiKeyListResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/api-keys/{}", org_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn api_keys_revoke(
        &self,
        org_id: &str,
        key_id: &str,
    ) -> Result<types::ApiKeyRevokeResponse> {
        api_request(
            &self.http,
            reqwest::Method::DELETE,
            &self.url(&format!("/api-keys/{}/{}", org_id, key_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn api_keys_usage(
        &self,
        org_id: &str,
        days: u32,
    ) -> Result<types::ApiKeyUsageResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/api-keys/{}/usage?days={}", org_id, days)),
            None,
            self.bearer_auth(),
        )
    }

    // ── OAuth Clients ────────────────────────────────────────

    pub fn oauth_clients_create(
        &self,
        org_id: &str,
        name: &str,
        redirect_uris: Vec<String>,
    ) -> Result<types::OAuthClientResponse> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url("/oauth-clients"),
            Some(json!({
                "organization_id": org_id,
                "name": name,
                "redirect_uris": redirect_uris,
            })),
            self.bearer_auth(),
        )
    }

    pub fn oauth_clients_list(&self, org_id: &str) -> Result<types::OAuthClientListResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/oauth-clients/{}", org_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn oauth_clients_update(
        &self,
        org_id: &str,
        client_id: &str,
        name: Option<&str>,
        redirect_uris: Option<Vec<String>>,
    ) -> Result<types::OAuthClientResponse> {
        let mut body = json!({});
        if let Some(n) = name {
            body["name"] = json!(n);
        }
        if let Some(uris) = redirect_uris {
            body["redirect_uris"] = json!(uris);
        }
        api_request(
            &self.http,
            reqwest::Method::PUT,
            &self.url(&format!("/oauth-clients/{}/{}", org_id, client_id)),
            Some(body),
            self.bearer_auth(),
        )
    }

    pub fn oauth_clients_revoke(
        &self,
        org_id: &str,
        client_id: &str,
    ) -> Result<types::OAuthClientRevokeResponse> {
        api_request(
            &self.http,
            reqwest::Method::DELETE,
            &self.url(&format!("/oauth-clients/{}/{}", org_id, client_id)),
            None,
            self.bearer_auth(),
        )
    }

    // ── AI Gateway ───────────────────────────────────────────

    pub fn ai_pricing(&self) -> Result<types::AiPricingResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url("/ai/pricing"),
            None,
            Auth::None,
        )
    }

    // ── Deploy: Backups ──────────────────────────────────────

    pub fn deploy_backup_create(
        &self,
        deployment_id: &str,
    ) -> Result<types::BackupInfo> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url(&format!("/deploy/{}/backups", deployment_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn deploy_backup_list(
        &self,
        deployment_id: &str,
    ) -> Result<types::BackupsListResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/deploy/{}/backups", deployment_id)),
            None,
            self.bearer_auth(),
        )
    }

    pub fn deploy_backup_restore(
        &self,
        deployment_id: &str,
        filename: &str,
    ) -> Result<types::BackupRestoreResponse> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url(&format!(
                "/deploy/{}/backups/{}/restore",
                deployment_id, filename
            )),
            None,
            self.bearer_auth(),
        )
    }

    pub fn deploy_backup_delete(
        &self,
        deployment_id: &str,
        filename: &str,
    ) -> Result<types::BackupDeleteResponse> {
        api_request(
            &self.http,
            reqwest::Method::DELETE,
            &self.url(&format!(
                "/deploy/{}/backups/{}",
                deployment_id, filename
            )),
            None,
            self.bearer_auth(),
        )
    }

    // ── Deploy: Versions ─────────────────────────────────────

    pub fn deploy_versions(
        &self,
        component_type: &str,
    ) -> Result<types::DeployVersionsResponse> {
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/deploy/versions/{}", component_type)),
            None,
            self.bearer_auth(),
        )
    }

    // ── Org Members: role update ─────────────────────────────

    pub fn org_member_update_role(
        &self,
        org_id: &str,
        user_id: &str,
        role: &str,
    ) -> Result<types::UpdateMemberRoleResponse> {
        api_request(
            &self.http,
            reqwest::Method::PUT,
            &self.url(&format!("/organizations/{}/members/{}", org_id, user_id)),
            Some(json!({ "role": role })),
            self.bearer_auth(),
        )
    }

    // ── Org Invitations ──────────────────────────────────────

    pub fn org_invite(
        &self,
        org_id: &str,
        email: &str,
        role: &str,
    ) -> Result<types::InviteResponse> {
        api_request(
            &self.http,
            reqwest::Method::POST,
            &self.url(&format!("/organizations/{}/invitations", org_id)),
            Some(json!({ "email": email, "role": role })),
            self.bearer_auth(),
        )
    }

    pub fn org_invites_list(
        &self,
        org_id: &str,
        pending_only: bool,
    ) -> Result<types::InviteListResponse> {
        let query = if !pending_only {
            "?pending_only=false"
        } else {
            ""
        };
        api_request(
            &self.http,
            reqwest::Method::GET,
            &self.url(&format!("/organizations/{}/invitations{}", org_id, query)),
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
            .mock("GET", "/me")
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
        let _mock = server.mock("GET", "/me").with_status(401).create();

        let client = AdminClient::new_with_token(&server.url(), "bad-token");
        let err = client.whoami().unwrap_err();
        assert!(err.to_string().contains("Authentication failed"));
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
            .with_body(
                r#"{"id": "org-new", "name": "NewOrg", "description": "test", "active": true}"#,
            )
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
        let resp = client
            .deploy_raw(serde_json::json!({
                "organization_id": "org-1",
                "deployment_name": "my-glow",
                "subdomain": "my",
                "version": "v1.0.0"
            }))
            .unwrap();
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
            .mock("GET", "/usage/summary/org-1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"total_simulations": 25, "free_tier": 10, "free_remaining": 0, "overage_simulations": 15, "estimated_cost": "$1.50"}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.usage_summary("org-1").unwrap();
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
            .with_body(
                r#"{"subscribed": true, "status": "active", "stripe_subscription_id": "sub_123"}"#,
            )
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

    #[test]
    fn test_billing_invoices_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/billing/invoices/org-1?limit=5")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"invoices": [{"id": "inv_1", "number": "INV-001", "status": "paid", "amount_due": 4900, "amount_paid": 4900, "currency": "usd"}]}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.billing_invoices("org-1", 5).unwrap();
        assert_eq!(resp.invoices.len(), 1);
        assert_eq!(resp.invoices[0].status, Some("paid".into()));
        assert_eq!(resp.invoices[0].amount_due, 4900);
        mock.assert();
    }

    #[test]
    fn test_billing_pricing_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/billing/pricing")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"tiers": [{"id": "free", "name": "Free", "price": "$0", "description": "Get started", "cta": "Start Free"}], "features": []}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.billing_pricing().unwrap();
        assert_eq!(resp.tiers[0].id, "free");
        mock.assert();
    }

    #[test]
    fn test_usage_daily_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/usage/daily/org-1?from=2026-03-01&to=2026-03-28")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"daily": [{"id": "d-1", "organization_id": "org-1", "usage_date": "2026-03-01", "simulation_count": 10, "attempts_started": 12, "attempts_completed": 10, "outcomes": 8}], "total_simulations": 10}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client
            .usage_daily("org-1", Some("2026-03-01"), Some("2026-03-28"))
            .unwrap();
        assert_eq!(resp.total_simulations, 10);
        assert_eq!(resp.daily[0].outcomes, Some(8));
        assert_eq!(resp.daily[0].attempts_started, Some(12));
        mock.assert();
    }

    #[test]
    fn test_usage_summary_with_meta() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/usage/summary/org-1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"total_simulations": 100, "plan": "payg", "estimated_cost": "$49.00", "meta": {"total_outcomes": 50, "attempts_completed": 80, "discount_pct": 20, "gross": "$100.00", "discount": "-$20.00", "net": "$80.00"}}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.usage_summary("org-1").unwrap();
        assert_eq!(resp.plan, Some("payg".into()));
        let meta = resp.meta.unwrap();
        assert_eq!(meta.total_outcomes, Some(50));
        assert_eq!(meta.discount_pct, Some(20));
        assert_eq!(meta.net, Some("$80.00".into()));
        mock.assert();
    }

    #[test]
    fn test_api_keys_create_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/api-keys")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": "key-1", "key": "ll_live_abc123xyz", "name": "My Key", "key_prefix": "ll_live_abc1", "scopes": ["ai"], "spend_limit_cents": 10000}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client
            .api_keys_create("org-1", Some("My Key"), Some(vec!["ai".into()]), Some(10000))
            .unwrap();
        assert_eq!(resp.name, "My Key");
        assert_eq!(resp.key, Some("ll_live_abc123xyz".into()));
        assert_eq!(resp.spend_limit_cents, Some(10000));
        mock.assert();
    }

    #[test]
    fn test_api_keys_list_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/api-keys/org-1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"keys": [{"id": "key-1", "name": "Default", "key_prefix": "ll_live_abc1", "scopes": ["ai"]}]}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.api_keys_list("org-1").unwrap();
        assert_eq!(resp.keys.len(), 1);
        assert_eq!(resp.keys[0].key_prefix, "ll_live_abc1");
        mock.assert();
    }

    #[test]
    fn test_api_keys_revoke_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("DELETE", "/api-keys/org-1/key-1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"revoked": true}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.api_keys_revoke("org-1", "key-1").unwrap();
        assert!(resp.revoked);
        mock.assert();
    }

    #[test]
    fn test_api_keys_usage_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/api-keys/org-1/usage?days=7")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"total_requests": 150, "total_tokens": 50000, "total_cost_cents": 250}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.api_keys_usage("org-1", 7).unwrap();
        assert_eq!(resp.total_requests, 150);
        assert_eq!(resp.total_tokens, 50000);
        assert_eq!(resp.total_cost_cents, 250);
        mock.assert();
    }

    #[test]
    fn test_oauth_clients_create_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/oauth-clients")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": "oc-1", "client_id": "ll_client_abc", "client_secret": "secret123", "name": "My App", "redirect_uris": ["https://app.com/callback"], "scopes": ["openid", "profile"]}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client
            .oauth_clients_create("org-1", "My App", vec!["https://app.com/callback".into()])
            .unwrap();
        assert_eq!(resp.client_id, "ll_client_abc");
        assert_eq!(resp.client_secret, Some("secret123".into()));
        assert_eq!(resp.name, "My App");
        mock.assert();
    }

    #[test]
    fn test_oauth_clients_list_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/oauth-clients/org-1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"clients": [{"id": "oc-1", "client_id": "ll_client_abc", "name": "My App", "redirect_uris": ["https://app.com/cb"], "scopes": ["openid"]}]}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.oauth_clients_list("org-1").unwrap();
        assert_eq!(resp.clients.len(), 1);
        assert_eq!(resp.clients[0].name, "My App");
        mock.assert();
    }

    #[test]
    fn test_oauth_clients_revoke_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("DELETE", "/oauth-clients/org-1/ll_client_abc")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"revoked": true}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.oauth_clients_revoke("org-1", "ll_client_abc").unwrap();
        assert!(resp.revoked);
        mock.assert();
    }

    #[test]
    fn test_ai_pricing_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/ai/pricing")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"tiers": [{"id": "light", "name": "Light", "description": "Local LLMs", "cost": "Free", "models": [{"name": "qwen-3.5-9b", "type": "llm", "description": "Fast local LLM"}]}]}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.ai_pricing().unwrap();
        assert_eq!(resp.tiers.len(), 1);
        assert_eq!(resp.tiers[0].id, "light");
        assert_eq!(resp.tiers[0].models[0].name, "qwen-3.5-9b");
        mock.assert();
    }

    #[test]
    fn test_deploy_backup_list_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/deploy/d-1/backups")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"backups": [{"name": "backup-2026-03-28.sql.gz", "size_bytes": 1024000, "created_at": "2026-03-28T10:00:00Z"}, {"name": "fresh.sql.gz", "size_bytes": 512, "is_template": true}]}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.deploy_backup_list("d-1").unwrap();
        assert_eq!(resp.backups.len(), 2);
        assert_eq!(resp.backups[1].is_template, Some(true));
        mock.assert();
    }

    #[test]
    fn test_deploy_backup_create_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/deploy/d-1/backups")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"name": "backup-2026-03-28.sql.gz", "size_bytes": 2048000}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.deploy_backup_create("d-1").unwrap();
        assert!(resp.name.contains("backup"));
        assert_eq!(resp.size_bytes, 2048000);
        mock.assert();
    }

    #[test]
    fn test_deploy_backup_restore_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/deploy/d-1/backups/backup-2026-03-28.sql.gz/restore")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"restoring": true, "backup": "backup-2026-03-28.sql.gz", "deployment_id": "d-1"}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client
            .deploy_backup_restore("d-1", "backup-2026-03-28.sql.gz")
            .unwrap();
        assert!(resp.restoring);
        mock.assert();
    }

    #[test]
    fn test_deploy_backup_delete_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("DELETE", "/deploy/d-1/backups/old.sql.gz")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"deleted": true, "name": "old.sql.gz"}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.deploy_backup_delete("d-1", "old.sql.gz").unwrap();
        assert!(resp.deleted);
        mock.assert();
    }

    #[test]
    fn test_deploy_versions_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/deploy/versions/api")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"versions": [{"name": "v2.4.0", "sha": "abc123def", "published_at": "2026-03-28", "prerelease": false}, {"name": "v2.5.0-beta", "sha": "def456abc", "prerelease": true}]}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.deploy_versions("api").unwrap();
        assert_eq!(resp.versions.len(), 2);
        assert_eq!(resp.versions[0].name, "v2.4.0");
        assert_eq!(resp.versions[1].prerelease, Some(true));
        mock.assert();
    }

    #[test]
    fn test_org_member_update_role_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("PUT", "/organizations/org-1/members/u-1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"updated": true, "user_id": "u-1", "role": "admin"}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.org_member_update_role("org-1", "u-1", "admin").unwrap();
        assert!(resp.updated);
        assert_eq!(resp.role, "admin");
        mock.assert();
    }

    #[test]
    fn test_org_invite_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/organizations/org-1/invitations")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": "inv-1", "organization_id": "org-1", "email": "new@example.com", "role": "member", "expires_at": "2026-04-28T00:00:00Z", "claimed": false}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.org_invite("org-1", "new@example.com", "member").unwrap();
        assert_eq!(resp.email, "new@example.com");
        assert_eq!(resp.role, "member");
        assert_eq!(resp.claimed, Some(false));
        mock.assert();
    }

    #[test]
    fn test_org_invites_list_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/organizations/org-1/invitations")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"invites": [{"id": "inv-1", "organization_id": "org-1", "email": "a@b.com", "role": "admin", "claimed": false}]}"#)
            .create();

        let client = AdminClient::new_with_token(&server.url(), "tok");
        let resp = client.org_invites_list("org-1", true).unwrap();
        assert_eq!(resp.invites.len(), 1);
        assert_eq!(resp.invites[0].email, "a@b.com");
        mock.assert();
    }
}
