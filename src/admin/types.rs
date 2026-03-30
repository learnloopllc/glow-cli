// admin/types.rs — Re-exports from auto-generated types + CLI aliases
//
// types_gen.rs is auto-generated from the LearnLoop API OpenAPI spec.
// Aliases below bridge API names to CLI names for backwards compatibility.

/// Pinned API version this CLI was built against (from api-versions.json via sync-types)
pub const PINNED_API_VERSION: &str = "2.0.9";

#[allow(unused_imports)]
pub use super::api::latest::*;

// ── Aliases (API name → CLI name) ───────────────────────────────

pub type WhoamiResponse = AuthMeResponse;
pub type OrgListResponse = OrganizationListResponse;
pub type Organization = OrganizationResponse;
pub type OrgDeleteResponse = DeletedResponse;
pub type OrgMembersResponse = MemberListResponse;
pub type OrgMemberAddResponse = AddMemberResponse;
pub type OrgMemberRemoveResponse = RemovedResponse;
pub type OrgDeploymentsResponse = DeployListResponse;
pub type Deployment = DeploymentResponse;
pub type DeployResponse = DeployCreateResponse;
pub type BillingPlansResponse = PlansResponse;
pub type BillingCheckoutResponse = CheckoutResponse;
pub type BillingInvoicesResponse = InvoicesResponse;
pub type BillingPricingResponse = PricingResponse;
#[cfg(test)]
pub type AiPricingResponse = GatewayPricingResponse;

// ── Tests ─────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_whoami() {
        let json = r#"{"user_id": "u-1", "email": "a@b.com", "name": "Alice", "is_superadmin": false, "org_id": "org-1", "org_role": "admin"}"#;
        let resp: WhoamiResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.email, "a@b.com");
        assert_eq!(resp.org_role, Some("admin".into()));
        assert!(!resp.is_superadmin);
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
    }

    #[test]
    fn test_deserialize_billing_plans() {
        let json = r#"{"plans": [{"id": "pro", "name": "Glow Pro", "pricing": "$49/mo", "included": 10, "overage_unit_price": "$0.10"}]}"#;
        let resp: BillingPlansResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.plans[0].name, "Glow Pro");
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
    fn test_deserialize_usage_summary_with_extra_fields() {
        // Extra fields (like meta) are silently ignored by serde
        let json = r#"{"total_simulations": 100, "plan": "payg", "estimated_cost": "$49.00", "meta": {"total_outcomes": 50}}"#;
        let resp: UsageSummaryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.total_simulations, 100);
        assert_eq!(resp.plan, Some("payg".into()));
        assert_eq!(resp.estimated_cost, Some("$49.00".into()));
    }

    #[test]
    fn test_deserialize_api_key_response() {
        let json = r#"{"id": "k-1", "key": "ll_live_abc123", "name": "Test Key", "key_prefix": "ll_live_abc1", "scopes": ["ai"], "spend_limit_cents": 5000}"#;
        let resp: ApiKeyResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.name, "Test Key");
        assert_eq!(resp.key, Some("ll_live_abc123".into()));
        assert_eq!(resp.spend_limit_cents, Some(5000));
    }

    #[test]
    fn test_deserialize_oauth_client_response() {
        let json = r#"{"id": "oc-1", "client_id": "ll_client_abc", "client_secret": "sec", "name": "My App", "redirect_uris": ["https://app.com/cb"], "scopes": ["openid", "profile"]}"#;
        let resp: OAuthClientResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.client_id, "ll_client_abc");
        assert_eq!(resp.name, "My App");
        assert_eq!(resp.scopes, Some(vec!["openid".into(), "profile".into()]));
    }

    #[test]
    fn test_deserialize_ai_pricing_response() {
        let json = r#"{"tiers": [{"id": "light", "name": "Light", "description": "Free local models", "cost": "Free", "models": [{"name": "qwen-3.5-9b", "type": "llm", "description": "Fast 9B model"}]}], "rate_limits": {"description": "Per-plan rate limits", "plans": {}}}"#;
        let resp: AiPricingResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.tiers[0].models[0].name, "qwen-3.5-9b");
    }

    #[test]
    fn test_deserialize_daily_usage_with_outcomes() {
        let json = r#"{"daily": [{"id": "d-1", "organization_id": "org-1", "usage_date": "2026-03-01", "simulation_count": 10}], "total_simulations": 10}"#;
        let resp: DailyUsageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.daily[0].simulation_count, 10);
        assert_eq!(resp.total_simulations, 10);
    }

    #[test]
    fn test_deserialize_invoices() {
        let json = r#"{"invoices": [{"id": "inv_1", "number": "INV-001", "status": "paid", "amount_due": 4900, "amount_paid": 4900, "currency": "usd", "period_start": "2026-03-01", "period_end": "2026-03-31"}]}"#;
        let resp: BillingInvoicesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.invoices[0].amount_due, 4900);
        assert_eq!(resp.invoices[0].status, Some("paid".into()));
    }

    #[test]
    fn test_deserialize_backup_info_with_template() {
        let json = r#"{"name": "fresh.sql.gz", "size_bytes": 512, "is_template": true}"#;
        let resp: BackupInfo = serde_json::from_str(json).unwrap();
        assert_eq!(resp.is_template, Some(true));
    }

    #[test]
    fn test_deserialize_invite_response() {
        let json = r#"{"id": "inv-1", "organization_id": "org-1", "email": "test@example.com", "role": "admin", "token": "tok_abc", "expires_at": "2026-04-28", "claimed": false}"#;
        let resp: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(resp["email"], "test@example.com");
        assert_eq!(resp["claimed"], false);
    }

    #[test]
    fn test_deserialize_member_role_update() {
        let json = r#"{"updated": true, "user_id": "u-1", "role": "admin"}"#;
        let resp: UpdateMemberRoleResponse = serde_json::from_str(json).unwrap();
        assert!(resp.updated);
        assert_eq!(resp.role, "admin");
    }
}
