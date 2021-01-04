#[cfg(test)]
mod tests {
    use serde_xml_rs::to_string;
    use crate::models::AuthData;

    #[test]
    fn auth_data() {
        let a = AuthData {
            login: "trixie".to_string(),
            pass: "Trixie is Best Pony".to_string(),
            institution_id: None,
        };
        let xml = to_string(&a).unwrap();
        assert_eq!(xml, "<AuthData><Login>trixie</Login><Pass>Trixie is Best Pony</Pass></AuthData>");
    }

    #[test]
    fn auth_data2() {
        let a = AuthData {
            login: "trixie".to_string(),
            pass: "Trixie is Best Pony".to_string(),
            institution_id: Some(42),
        };
        let xml = to_string(&a).unwrap();
        assert_eq!(xml, "<AuthData><Login>trixie</Login><Pass>Trixie is Best Pony</Pass><InstitutionId>42</InstitutionId></AuthData>");
    }

    // todo use validator::Validate;
}
