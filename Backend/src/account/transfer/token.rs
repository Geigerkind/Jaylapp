use rocket::State;
use rocket_contrib::json::Json;

use crate::account::dto::{CreateToken, RestrictedContent};
use crate::account::domain_value::ValidationPair;
use crate::account::material::{Account, APIToken};
use crate::account::tools::Token;
use language::domain_value::Language;
use language::tools::Get;

#[post("/token/create", data = "<params>")]
pub fn create_token(me: State<Account>, params: Json<CreateToken>) -> Result<Json<APIToken>, String>
{
  if !me.validate_token(&params.val_pair) {
    return Err(me.dictionary.get("general.error.validate", Language::English));
  }

  me.create_token(&params.purpose, params.val_pair.member_id, params.exp_date)
    .and_then(|api_token| Ok(Json(api_token)))
}

#[post("/token/get", data = "<params>")]
pub fn get_tokens(me: State<Account>, params: Json<ValidationPair>) -> Result<Json<Vec<APIToken>>, String> {
  if !me.validate_token(&params) {
    return Err(me.dictionary.get("general.error.validate", Language::English));
  }

  Ok(Json(me.get_all_token(params.member_id)))
}

#[post("/token/delete", data = "<params>")]
pub fn delete_token(me: State<Account>, params: Json<RestrictedContent<u32, ValidationPair>>) -> Result<(), String>
{
  if !me.validate_token(&params.validation) {
    return Err(me.dictionary.get("general.error.validate", Language::English));
  }
  me.delete_token(params.content, params.validation.member_id)
}