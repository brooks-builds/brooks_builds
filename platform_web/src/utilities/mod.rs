pub mod log;

use rand::distributions::{Alphanumeric, DistString};
use url::{ParseError, Url};
use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;

pub fn set_cookie(key: &str, value: &str) {
    let document = gloo::utils::document().unchecked_into::<HtmlDocument>();
    let cookie = format!("{key}={value}; SameSite=Strict; Secure;");
    match document.set_cookie(&cookie) {
        Ok(_) => {}
        Err(error) => gloo::console::error!("Error setting cookie", error),
    };
}

pub fn create_uri(base_uri: &str, query_params: Vec<UriQueryParam>) -> Result<Url, ParseError> {
    let mut uri = url::Url::parse(base_uri)?;

    for query_param in query_params {
        uri.query_pairs_mut()
            .append_pair(&query_param.name, &query_param.value);
    }

    Ok(uri)
}

pub fn create_random_string(length: i32) -> String {
    let mut rng = rand::thread_rng();
    Alphanumeric.sample_string(&mut rng, 43)
}

pub struct UriQueryParam {
    pub name: String,
    pub value: String,
}

impl UriQueryParam {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_owned(),
            value: value.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use url::ParseError;

    use crate::utilities::UriQueryParam;

    use super::*;

    #[test]
    fn test_create_uri() -> Result<(), ParseError> {
        let query_params = vec![
            UriQueryParam::new("test", "value"),
            UriQueryParam::new("hello", "world"),
        ];
        let uri = create_uri("http://localhost:8080", query_params)?;

        assert_eq!(uri.domain(), Some("http://localhost:8080"));

        let uri_query_pairs = uri.query_pairs();

        assert_eq!(uri_query_pairs.count(), 2);

        Ok(())
    }

    fn test_create_random_string() {
        let random_string: String = create_random_string(43);
        let another_random_string = create_random_string(43);

        assert_ne!(random_string, another_random_string);
        assert_eq!(random_string.len(), 43);
    }
}
