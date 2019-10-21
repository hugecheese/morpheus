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
use crate::rest::{events, Client};
use reqwest::RequestBuilder;
use serde::Serialize;

#[derive(Serialize, Default)]
pub struct Sync {
    filter: Option<String>,
    since: Option<String>,
    full_state: Option<bool>,
    // TODO: remaining fields
}

pub struct SyncBuilder {
    req: RequestBuilder,
    sync: self::Sync,
}

impl SyncBuilder {
    pub fn new(req: RequestBuilder) -> Self {
        Self {
            req,
            sync: Default::default(),
        }
    }

    pub fn filter<T>(mut self, input: T) -> Self
    where
        T: Into<String>,
    {
        self.sync.filter = Some(input.into());
        self
    }

    pub fn since<T>(mut self, input: T) -> Self
    where
        T: Into<String>,
    {
        self.sync.since = Some(input.into());
        self
    }

    pub fn full_state<T>(mut self, input: bool) -> Self {
        self.sync.full_state = Some(input);
        self
    }

    pub async fn send(self) -> crate::Result<events::Sync> {
        let builder = self.req.query(&self.sync);
        Client::debug_request(builder).await
    }
}
