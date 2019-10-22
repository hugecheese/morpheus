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
use self::rest::{endpoints, Client};
use crate::rest;
use serde::{Deserialize, Serialize};
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

// TODO: move me to some general location
#[derive(Deserialize)]
pub struct SendResponse {
    event_id: String,
}

// TODO: shared representation of the same structs between events and reqs
#[derive(Serialize, Default, Debug)]
pub struct SendMessage {
    body: String,
    msgtype: String,
}

pub struct SendMessageBuilder<'a> {
    req: &'a rest::Client,
    value: SendMessage,
}

impl<'a> SendMessageBuilder<'a> {
    pub fn new(req: &'a rest::Client) -> Self {
        Self {
            req,
            value: Default::default(),
        }
    }

    pub fn body<T>(mut self, input: T) -> Self
    where
        T: Into<String>,
    {
        self.value.body = input.into();
        self
    }

    pub fn msgtype<T>(mut self, input: T) -> Self
    where
        T: Into<String>,
    {
        self.value.msgtype = input.into();
        self
    }

    pub async fn send(self, room_id: &str) -> crate::Result<SendResponse> {
        let url = format!(
            endpoints::send!(),
            percent_encode(room_id.as_bytes(), NON_ALPHANUMERIC).to_string(),
            "m.room.message",
            rand::random::<u64>()
        );
        dbg!(&url);
        dbg!(&self.value);
        let builder = self.req.put(&url).json(&self.value);

        Client::debug_request(builder).await
    }
}
