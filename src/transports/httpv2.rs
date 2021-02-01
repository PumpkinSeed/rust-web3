// @TODO proxy support
use crate::{error, rpc};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Http {
    url: String,
    client: reqwest::blocking::Client,
}

impl Http {
    pub fn new(url: &str) -> error::Result<Self> {
        Ok(Http{
            url: String::from(url),
            client: reqwest::blocking::Client::new(),
        })
    }

    pub fn execute(&self, method: &str, params: Option<Vec<rpc::Value>>) {
        let mut builder = self.client.request(reqwest::Method::POST, reqwest::Url::parse(&self.url.clone()[..]).unwrap());
        builder = builder.header(reqwest::header::CONTENT_TYPE, "application/json");
        let resp = builder.send().unwrap().text().unwrap();
        println!("{:?}", resp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    # [test]
    fn test_reqwest() {
        let addr = "127.0.0.1:3001";
        ethock_lib::server::Entry::new(addr.clone()).serve_silent();

        let http = Http::new("http://127.0.0.1:3001").unwrap();
        http.execute(ethock_lib::methods::ETH_ACCOUNTS, None)
    }
}
