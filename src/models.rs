use serde::{Serialize, Deserialize};
use validator::Validate;

/// Блок авторизации
#[derive(Debug, Validate, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthData {
    #[validate(length(max = 50))]
    pub login: String,
    /// Пароль
    #[validate(length(max = 50))]
    pub pass: String,
    /// Идентификатор ВУЗа
    #[serde(rename = "InstitutionID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    name: String,
    source: String,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub name: String,
    #[serde(rename = "Item", default)]
    pub items: Vec<Item>,
}

#[cfg(test)]
mod tests {
    use serde_json;
    // use validator::Validate;

    use crate::models::AuthData;

    #[test]
    fn auth_data_some() {
        let a = AuthData {
            login: "trixie".to_string(),
            pass: "Trixie is Best Pony".to_string(),
            institution_id: Some(42),
        };
        let xml = serde_json::to_string(&a).unwrap();
        assert_eq!(xml, r#"{"Login":"trixie","Pass":"Trixie is Best Pony","InstitutionID":42}"#);
    }

    #[test]
    fn auth_data_none() {
        let a = AuthData {
            login: "trixie".to_string(),
            pass: "Trixie is Best Pony".to_string(),
            institution_id: None,
        };
        let xml = serde_json::to_string(&a).unwrap();
        assert_eq!(xml, r#"{"Login":"trixie","Pass":"Trixie is Best Pony"}"#);
    }

    // #[test]
    // fn auth_data_validator_login() {
    //     let login = "trixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixie";
    //     assert!(login.len() >= 50);
    //     let a = AuthData {
    //         login: login.to_string(),
    //         pass: "Trixie is Best Pony".to_string(),
    //         institution_id: None,
    //     };
    //     assert!(a.validate().is_err());
    // }
    //
    // #[test]
    // fn auth_data_validator_pass() {
    //     let pass = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    //     assert!(pass.len() >= 50);
    //     let a = AuthData {
    //         login: "trixie".to_string(),
    //         pass: pass.to_string(),
    //         institution_id: None,
    //     };
    //     assert!(a.validate().is_err());
    // }
}
