use serde::{Serialize, Deserialize};
use validator::Validate;

// schemas/запрос-авторизация.xsd

/// Блок авторизации
#[derive(Debug, Validate, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthData {
    /// Номер заявления ОУ
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
    use validator::Validate;
    use serde_xml_rs::to_string;

    use crate::models::AuthData;

    #[test]
    fn auth_data_some() {
        let a = AuthData {
            login: "trixie".to_string(),
            pass: "Trixie is Best Pony".to_string(),
            institution_id: Some(42),
        };
        let xml = to_string(&a).unwrap();
        assert_eq!(xml, "<AuthData><Login>trixie</Login><Pass>Trixie is Best Pony</Pass><InstitutionID>42</InstitutionID></AuthData>");
    }

    #[test]
    fn auth_data_none() {
        let a = AuthData {
            login: "trixie".to_string(),
            pass: "Trixie is Best Pony".to_string(),
            institution_id: None,
        };
        let xml = to_string(&a).unwrap();
        assert_eq!(xml, "<AuthData><Login>trixie</Login><Pass>Trixie is Best Pony</Pass></AuthData>");
    }

    #[test]
    fn auth_data_validator_login() {
        let login = "trixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixietrixie";
        assert!(login.len() >= 50);
        let a = AuthData {
            login: login.to_string(),
            pass: "Trixie is Best Pony".to_string(),
            institution_id: None,
        };
        assert!(a.validate().is_err());  //todo get message
    }

    #[test]
    fn auth_data_validator_pass() {
        let pass = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        assert!(pass.len() >= 50);
        let a = AuthData {
            login: "trixie".to_string(),
            pass: pass.to_string(),
            institution_id: None,
        };
        assert!(a.validate().is_err());  //todo get message
    }
}
