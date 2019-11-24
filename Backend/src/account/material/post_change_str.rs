use schemars::JsonSchema;

use crate::account::domain_value::ValidationPair;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct PostChangeStr {
  pub content: String,
  pub validation: ValidationPair,
}