use crate::util::sha3;
use crate::util::mail;
use crate::util::random;
use crate::util::strformat;
use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::account::tools::validator::Validator;
use crate::account::material::account::Account;
use crate::database::tools::mysql::execute::Execute;
use crate::util::language::tools::get::Get;
use crate::util::language::domainvalue::language::Language;

pub trait Forgot {
  fn send_forgot_password(&self, params: &ValidationPair) -> Result<(), String>;
  fn recv_forgot_password(&self, id: &str) -> Result<(), String>;
}

impl Forgot for Account {
  fn send_forgot_password(&self, params: &ValidationPair) -> Result<(), String>
  {
    if !self.validate(params) {
      return Err(self.dictionary.get("general.error.validate", Language::English));
    }

    let forgot_id: String;
    {
      {
        let member = self.member.read().unwrap();
        let entry = member.get(&params.id).unwrap();
        forgot_id = sha3::hash(&[&params.id.to_string(), "forgot", &entry.salt]);
        if !mail::send(&entry.mail, &entry.nickname, self.dictionary.get("forgot.confirmation.subject", Language::English),
          strformat::fmt(self.dictionary.get("forgot.confirmation.text", Language::English), &vec![&forgot_id])){
            return Err(self.dictionary.get("general.error.mail_send", Language::English));
        }
      }
      if self.db_main.execute_wparams("UPDATE member SET forgot_password=1 WHERE id=:id", params!("id" => params.id)) {
        let mut member = self.member.write().unwrap();
        let entry = member.get_mut(&params.id).unwrap();
        entry.forgot_password = true;
      }
    }

    let mut forgot_password = self.forgot_password.write().unwrap();
    forgot_password.insert(forgot_id, params.id);

    Ok(())
  }

  fn recv_forgot_password(&self, id: &str) -> Result<(), String>
  {
    let mut removable = false;
    {
      let forgot_password = self.forgot_password.read().unwrap();
      match forgot_password.get(id) {
        Some(member_id) => {
          // Sending random generated password
          let new_pass = random::alphanumeric(16);
          {
            let member = self.member.read().unwrap();
            let entry = member.get(member_id).unwrap();
            if !mail::send(&entry.mail, &entry.nickname, self.dictionary.get("forgot.information.subject", Language::English),
              strformat::fmt(self.dictionary.get("forgot.information.text", Language::English), &vec![&new_pass])) {
                return Err(self.dictionary.get("general.error.mail_send", Language::English));
            }
          }
          if self.db_main.execute_wparams("UPDATE member SET forgot_password=0, password=:pass WHERE id=:id", params!(
            "pass" => new_pass,
            "id" => *member_id
          )) {
            let mut member = self.member.write().unwrap();
            let entry = member.get_mut(member_id).unwrap();
            entry.forgot_password = false;
            removable = true;
          }
        },
        None => return Err(self.dictionary.get("forgot.error.no_forgot_issued", Language::English))
      }
    }
    if removable {
      let mut forgot_password = self.forgot_password.write().unwrap();
      forgot_password.remove(id);
      return Ok(());
    }
    Err(self.dictionary.get("general.unknown", Language::English))
  }
}