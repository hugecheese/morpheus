/*
 * Morpheus is a client for the Matrix open standard.
 * Copyright (C) 2019 Morpheus Contributors
 *
 * Morpheus is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License, version 3,
 * as published by the Free Software Foundation.
 *
 * Morpheus is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License,
 * version 3, along with Morpheus. If not, see <https://www.gnu.org/licenses/>.
 */
use super::reqs::{SendMessageBuilder, SyncBuilder};
use reqwest::{Method, RequestBuilder};
use serde::de::DeserializeOwned;

pub struct Client {
    auth: String,
    req: reqwest::Client,
}

impl Client {
    pub fn new(token: &str) -> Self {
        Self {
            auth: "Bearer ".to_owned() + token,
            req: reqwest::Client::new(),
        }
    }

    fn request(&self, method: Method, url: &str) -> RequestBuilder {
        self.req
            .request(method, url)
            .header("Authorization", &self.auth)
    }

    pub fn get(&self, url: &str) -> RequestBuilder {
        self.request(Method::GET, url)
    }

    pub fn post(&self, url: &str) -> RequestBuilder {
        self.request(Method::POST, url)
    }

    pub fn put(&self, url: &str) -> RequestBuilder {
        self.request(Method::PUT, url)
    }

    pub fn sync(&self) -> SyncBuilder {
        SyncBuilder::new(&self)
    }

    pub fn send_message(&self) -> SendMessageBuilder {
        SendMessageBuilder::new(&self)
    }

    // TODO: remove me when more stable
    // TODO: DRAGONS BEWARE
    pub async fn debug_request<T>(req: RequestBuilder) -> crate::Result<T>
    where
        T: DeserializeOwned + Sized,
    {
        let raw = req.send().await?.bytes().await?;
        let obj: serde_json::Value = serde_json::from_slice(&raw)?;
        let pretty = serde_json::to_string_pretty(&obj)?;
        let res = serde_json::from_str::<T>(&pretty);

        match res {
            Ok(val) => Ok(val),
            Err(e) => {
                let r: u64 = rand::random();
                let filename = format!("{}.json", r);
                println!("JSON ERROR: {}\n\nDUMPING FILE: {}", e, filename);
                std::fs::write(&filename, &pretty)?;
                Err(Box::new(e))
            }
        }
    }
}
