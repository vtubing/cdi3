#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct DisplayInfo3 {
  #[serde(default)]
  pub combined_parameters: Vec<(String, String)>,
  pub parameter_groups: Vec<ParameterGroup>,
  pub parameters: Vec<Parameter>,
  pub parts: Vec<Part>,
  pub version: u8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Parameter {
  pub group_id: String,
  pub id: String,
  pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct ParameterGroup {
  pub group_id: String,
  pub id: String,
  pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Part {
  pub id: String,
  pub name: String,
}
