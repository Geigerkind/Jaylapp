#[cfg(test)]
mod tests {
  use crate::account::material::account::Account;
  use crate::account::tools::create::Create;
  use crate::account::service::create::PostCreateMember;
  use crate::database::tools::mysql::execute::Execute;
  use crate::account::service::update::PostChangeStr;
  use crate::account::tools::update::Update;
  use crate::account::domainvalue::validation_pair::ValidationPair;

  #[test]
  fn change_name() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "ijofsdiojsdfgiuhig".to_string(),
      mail: "ijofsdiojsdfgiuhig@jaylappTest.dev".to_string(),
      password: "Password123456".to_string()
    };

    let val_pair = account.create(&post_obj).unwrap();
    let changed_name = account.change_name(&PostChangeStr {
      content: "Some Username".to_string(),
      validation: val_pair
    });
    assert!(changed_name.is_ok());
    assert_eq!(changed_name.unwrap().nickname, "Some Username".to_string());

    account.db_main.execute("DELETE FROM member WHERE mail='ijofsdiojsdfgiuhig@jaylappTest.dev'");
  }

  #[test]
  fn change_name_invalid_validation() {
    let account = Account::default();
    let val_pair = ValidationPair {
      hash: "someHash".to_string(),
      id: 42
    };

    let changed_name = account.change_name(&PostChangeStr {
      content: "Some Username".to_string(),
      validation: val_pair
    });

    assert!(changed_name.is_err());
  }

  #[test]
  fn change_name_empty_content() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "siodjfijsiojiospq".to_string(),
      mail: "siodjfijsiojiospq@jaylappTest.dev".to_string(),
      password: "Password123456".to_string()
    };

    let val_pair = account.create(&post_obj).unwrap();
    let changed_name = account.change_name(&PostChangeStr {
      content: "".to_string(),
      validation: val_pair
    });
    assert!(changed_name.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='siodjfijsiojiospq@jaylappTest.dev'");
  }

  #[test]
  fn change_name_name_taken() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "oasijidhaais".to_string(),
      mail: "oasijidhaais@jaylappTest.dev".to_string(),
      password: "Password123456".to_string()
    };

    let post_obj_two = PostCreateMember {
      nickname: "guhzasooas".to_string(),
      mail: "guhzasooas@jaylappTest.dev".to_string(),
      password: "Password123456".to_string()
    };

    let val_pair = account.create(&post_obj).unwrap();
    let _ = account.create(&post_obj_two).unwrap();
    let changed_name = account.change_name(&PostChangeStr {
      content: post_obj_two.nickname,
      validation: val_pair
    });
    assert!(changed_name.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='oasijidhaais@jaylappTest.dev'");
    account.db_main.execute("DELETE FROM member WHERE mail='guhzasooas@jaylappTest.dev'");
  }

  #[test]
  fn change_password_invalid_validation() {
    let account = Account::default();
    let val_pair = ValidationPair {
      hash: "someHash".to_string(),
      id: 42
    };

    let changed_password = account.change_password(&PostChangeStr {
      content: "Some Username".to_string(),
      validation: val_pair
    });

    assert!(changed_password.is_err());
  }

  #[test]
  fn change_password_empty_content() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "mvfhhbvidsd".to_string(),
      mail: "mvfhhbvidsd@jaylappTest.dev".to_string(),
      password: "Password123456".to_string()
    };

    let val_pair = account.create(&post_obj).unwrap();
    let changed_password = account.change_password(&PostChangeStr {
      content: "".to_string(),
      validation: val_pair
    });
    assert!(changed_password.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='mvfhhbvidsd@jaylappTest.dev'");
  }

  #[test]
  fn change_password() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "xdsdfgsdgs".to_string(),
      mail: "xdsdfgsdgs@jaylappTest.dev".to_string(),
      password: "Password123456".to_string()
    };

    let val_pair = account.create(&post_obj).unwrap();
    let val_pair_hash = val_pair.hash.clone();
    let val_pair_id = val_pair.id;
    let changed_password = account.change_password(&PostChangeStr {
      content: "SomeWeirdPassword".to_string(),
      validation: val_pair
    });
    assert!(changed_password.is_ok());
    let new_val_pair = changed_password.unwrap();
    assert_ne!(new_val_pair.hash, val_pair_hash);
    assert_eq!(new_val_pair.id, val_pair_id);

    account.db_main.execute("DELETE FROM member WHERE mail='xdsdfgsdgs@jaylappTest.dev'");
  }

  #[test]
  fn change_mail_invalid_validation() {
    let account = Account::default();
    let val_pair = ValidationPair {
      hash: "someHash".to_string(),
      id: 42
    };

    let changed_mail = account.change_mail(&PostChangeStr {
      content: "Some Username".to_string(),
      validation: val_pair
    });

    assert!(changed_mail.is_err());
  }

  #[test]
  fn change_mail_empty_content() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "nsigsvbsdsd".to_string(),
      mail: "nsigsvbsdsd@jaylappTest.dev".to_string(),
      password: "Password123456".to_string()
    };

    let val_pair = account.create(&post_obj).unwrap();
    let changed_mail = account.change_mail(&PostChangeStr {
      content: "".to_string(),
      validation: val_pair
    });
    assert!(changed_mail.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='nsigsvbsdsd@jaylappTest.dev'");
  }

  #[test]
  fn change_mail_invalid_content() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "asiudfuhisduifs".to_string(),
      mail: "asiudfuhisduifs@jaylappTest.dev".to_string(),
      password: "Password123456".to_string()
    };

    let val_pair = account.create(&post_obj).unwrap();
    let changed_mail = account.change_mail(&PostChangeStr {
      content: "asiudfuhisduifs".to_string(),
      validation: val_pair
    });
    assert!(changed_mail.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='asiudfuhisduifs@jaylappTest.dev'");
  }

  #[test]
  fn change_mail_mail_taken() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "csdazgtsdczas".to_string(),
      mail: "csdazgtsdczas@jaylappTest.dev".to_string(),
      password: "Password123456".to_string()
    };

    let post_obj_two = PostCreateMember {
      nickname: "bdvshudvbsdv".to_string(),
      mail: "bdvshudvbsdv@jaylappTest.dev".to_string(),
      password: "Password123456".to_string()
    };

    let val_pair = account.create(&post_obj).unwrap();
    let _ = account.create(&post_obj_two).unwrap();
    let changed_mail = account.change_mail(&PostChangeStr {
      content: post_obj_two.mail,
      validation: val_pair
    });
    assert!(changed_mail.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='csdazgtsdczas@jaylappTest.dev'");
    account.db_main.execute("DELETE FROM member WHERE mail='bdvshudvbsdv@jaylappTest.dev'");
  }

  #[test]
  fn change_mail() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "xdssdfsdfg".to_string(),
      mail: "xdssdfsdfg@jaylappTest.dev".to_string(),
      password: "Password123456".to_string()
    };

    let val_pair = account.create(&post_obj).unwrap();
    let val_pair_hash = val_pair.hash.clone();
    let val_pair_id = val_pair.id;
    let changed_mail = account.change_mail(&PostChangeStr {
      content: "bla@bla.de".to_string(),
      validation: val_pair
    });
    assert!(changed_mail.is_ok());
    let new_val_pair = changed_mail.unwrap();
    assert_ne!(new_val_pair.hash, val_pair_hash);
    assert_eq!(new_val_pair.id, val_pair_id);

    account.db_main.execute("DELETE FROM member WHERE mail='xdssdfsdfg@jaylappTest.dev'");
  }
}