use schemars::JsonSchema;

use crate::account::domain_value::ValidationPair;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct PostDeleteToken {
  pub token_id: u32,
  pub val_pair: ValidationPair,
}