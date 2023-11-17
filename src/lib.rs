mod parameter;
mod parameter_group;
mod part;

pub use parameter::Parameter;
pub use parameter_group::ParameterGroup;
pub use part::Part;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
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
