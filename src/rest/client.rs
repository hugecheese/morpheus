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
use super::events;
use crate::endpoints;
use reqwest::{Method, RequestBuilder};

pub struct Client {
    auth: String,
    req: reqwest::Client,
}

impl Client {
    pub fn new(token: &str) -> Client {
        Client {
            auth: "Bearer ".to_owned() + token,
            req: reqwest::Client::new(),
        }
    }

    fn request(&self, method: Method, url: &str) -> RequestBuilder {
        self.req
            .request(method, url)
            .header("Authorization", &self.auth)
    }

    fn get(&self, url: &str) -> RequestBuilder {
        self.request(Method::GET, url)
    }

    fn post(&self, url: &str) -> RequestBuilder {
        self.request(Method::POST, url)
    }

    // TODO: rewrite this function, it's gross omegalul
    pub async fn sync(&self, next_batch: &str) -> crate::Result<events::Sync> {
        let url = if next_batch.is_empty() {
            endpoints::SYNC.into()
        } else {
            format!("{}?since={}", endpoints::SYNC, next_batch)
        };

        let b = self.get(&url).send().await?.bytes().await?;
        let obj: serde_json::Value = serde_json::from_slice(&b)?;
        let pretty = serde_json::to_string_pretty(&obj)?;
        let res: crate::Result<events::Sync> = serde_json::from_str(&pretty)
            .or_else(|err| Err(Box::new(err) as Box<dyn std::error::Error>));
        match res {
            Ok(val) => Ok(val),
            Err(e) => {
                let r: u64 = rand::random();
                let filename = format!("{}.json", r);
                println!("JSON ERROR: {}\n\nDUMPING FILE: {}", e, filename);
                std::fs::write(&filename, &pretty)?;
                Err(e)
            }
        }
    }
}
