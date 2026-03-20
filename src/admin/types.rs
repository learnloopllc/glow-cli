// admin/types.rs — Re-exports from auto-generated types + CLI aliases
//
// types_gen.rs is auto-generated from the LearnLoop API OpenAPI spec.
// Aliases below bridge API names to CLI names for backwards compatibility.

#[allow(unused_imports)]
pub use super::types_gen::*;

// ── Aliases (API name → CLI name) ───────────────────────────────
//
// The generated types use the API's response model names. The CLI
// historically used different names. These aliases keep existing
// client code working without changes.

pub type WhoamiResponse = AuthMeResponse;
pub type ValidateLicenseResponse = LicenseValidateValidResponse;
pub type LicenseInfo = LicenseResponse;
pub type LicenseDeleteResponse = DeletedResponse;
pub type OrgListResponse = OrganizationListResponse;
pub type Organization = OrganizationResponse;
pub type OrgDeleteResponse = DeletedResponse;
pub type OrgMember = MemberResponse;
pub type OrgMembersResponse = MemberListResponse;
pub type OrgMemberAddResponse = AddMemberResponse;
pub type OrgMemberRemoveResponse = RemovedResponse;
pub type OrgLicenseSetResponse = AssignedResponse;
pub type OrgLicenseRemoveResponse = RemovedResponse;
pub type OrgDeploymentsResponse = DeployListResponse;
pub type Deployment = DeploymentResponse;
pub type DeployResponse = DeployCreateResponse;
// UsageReportResponse is now generated directly from the API
pub type BillingPlan = PlanInfo;
pub type BillingPlansResponse = PlansResponse;
pub type BillingCheckoutResponse = CheckoutResponse;

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
    fn test_deserialize_license_list() {
        let json = r#"{"licenses": [{"id": "lic-1", "key": "abc", "expiry": "2026-12-31", "active": true}]}"#;
        let resp: LicenseListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.licenses.len(), 1);
        assert_eq!(resp.licenses[0].id, "lic-1");
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
}
