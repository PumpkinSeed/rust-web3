// @TODO proxy support
use crate::{error};

#[derive(Debug, Clone)]
pub struct Http {
    url: String,
    client: reqwest::Client,
}

impl Http {
    pub fn new(url: &str) -> error::Result<Self> {
        Ok(Http{
            url: String::from(url),
            client: reqwest::Client::new(),
        })
    }

    pub fn execute(&self, method: &str, params: Vec<rpc::Value>) {
        let mut builder = self.client.request(reqwest::Method::POST, self.url.clone());
        builder = builder.header(reqwest::header::CONTENT_TYPE, "application/json");
        builder.send()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    # [test]
    fn test_reqwest() {
        let addr = "127.0.0.1:3001";
        ethock_lib::server::Entry::new(addr.clone()).serve_silent();

        let http = Http::new(addr.clone()).unwrap();

    }
}
