use rocket::State;
use rocket_contrib::json::Json;

use crate::account::dto::CreateMember;
use crate::account::domain_value::ValidationPair;
use crate::account::material::Account;
use crate::account::tools::{Create, Token};

#[post("/create", data = "<params>")]
pub fn create(me: State<Account>, params: Json<CreateMember>) -> Result<Json<ValidationPair>, String>
{
  me.create(&params.mail, &params.nickname, &params.password)
    .and_then(|val_pair| Ok(Json(val_pair)))
}

#[get("/create/confirm/<id>")]
pub fn confirm(me: State<Account>, id: String) -> Json<bool>
{
  Json(me.confirm(&id))
}

#[post("/create/resend", data = "<params>")]
pub fn resend_confirm(me: State<Account>, params: Json<ValidationPair>) -> Json<bool>
{
  if !me.validate_token(&params) {
    return Json(false);
  }
  Json(me.send_confirmation(params.member_id))
}