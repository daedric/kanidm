use url::Url;
use webauthn_rs::WebauthnConfig;

pub struct WebauthnDomainConfig {
    pub rp_name: String,
    pub origin: Url,
    pub rp_id: String,
}

impl WebauthnConfig for WebauthnDomainConfig {
    fn get_relying_party_name(&self) -> &str {
        self.rp_name.as_str()
    }

    fn get_origin(&self) -> &Url {
        &self.origin
    }

    fn get_relying_party_id(&self) -> &str {
        self.rp_id.as_str()
    }

    fn allow_subdomains_origin(&self) -> bool {
        true
    }
}
