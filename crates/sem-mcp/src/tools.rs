use serde::Deserialize;

// ── Tool parameter structs ──

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ExtractEntitiesParams {
    #[schemars(description = "Path to the file (relative to repo root or absolute)")]
    pub file_path: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ImpactAnalysisParams {
    #[schemars(description = "Path to the file containing the entity")]
    pub file_path: String,
    #[schemars(description = "Name of the entity to analyze impact for")]
    pub entity_name: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DependenciesParams {
    #[schemars(description = "Path to the file containing the entity")]
    pub file_path: String,
    #[schemars(description = "Name of the entity to analyze")]
    pub entity_name: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DependentsParams {
    #[schemars(description = "Path to the file containing the entity")]
    pub file_path: String,
    #[schemars(description = "Name of the entity to analyze")]
    pub entity_name: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DiffParams {
    #[schemars(description = "Base ref to compare from (branch, tag, or commit hash, e.g. 'main')")]
    pub base_ref: String,
    #[schemars(description = "Target ref to compare to. Defaults to HEAD.")]
    pub target_ref: Option<String>,
    #[schemars(description = "Optional: diff only this file")]
    pub file_path: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VerifyParams {
    #[schemars(description = "Path to the file to verify contracts for (relative or absolute)")]
    pub file_path: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TestMapParams {
    #[schemars(description = "Path to the file containing the entity")]
    pub file_path: String,
    #[schemars(description = "Name of the entity to find related tests for")]
    pub entity_name: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ContextParams {
    #[schemars(description = "Path to the file containing the entity")]
    pub file_path: String,
    #[schemars(description = "Name of the target entity")]
    pub entity_name: String,
    #[schemars(description = "Maximum token budget. Defaults to 8000.")]
    pub token_budget: Option<usize>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct HotspotParams {
    #[schemars(description = "Optional: analyze hotspots for a specific file only")]
    pub file_path: Option<String>,
    #[schemars(description = "Maximum number of hotspots to return. Defaults to 20.")]
    pub limit: Option<usize>,
}
