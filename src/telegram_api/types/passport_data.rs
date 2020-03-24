use super::{EncryptedCredentials, EncryptedPassportElement};

#[derive(Deserialize, Debug)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}
