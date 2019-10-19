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
use crate::{endpoints, raw};
use reqwest::Method;
use std::future::Future;

pub struct RestClient {
    auth: String,
    req: reqwest::Client,
}

pub type Resp = impl Future<Output = reqwest::Result<reqwest::Response>>;

impl RestClient {
    pub fn new(token: &str) -> RestClient {
        RestClient {
            auth: "Bearer ".to_owned() + token,
            req: reqwest::Client::new(),
        }
    }

    fn request(&self, method: Method, url: &str) -> Resp {
        self.req
            .request(method, url)
            .header("Authorization", &self.auth)
            .send()
    }

    fn get(&self, url: &str) -> Resp {
        self.request(Method::GET, url)
    }

    fn post(&self, url: &str) -> Resp {
        self.request(Method::POST, url)
    }

    pub async fn sync(&self) -> reqwest::Result<raw::Sync> {
        self.get(endpoints::SYNC).await?.json().await
    }
}
