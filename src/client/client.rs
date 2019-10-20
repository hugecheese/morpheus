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
use super::Message;
use crate::rest;
use crate::Result;

pub struct Client {
    req: rest::Client,
    message_handlers: Vec<fn(Message)>,
    next_batch: String,
}

impl Client {
    pub fn new(token: &str) -> Client {
        Client {
            req: rest::Client::new(token),
            message_handlers: Vec::new(),
            next_batch: String::new(),
        }
    }

    pub fn on_message(&mut self, handler: fn(Message)) {
        self.message_handlers.push(handler);
    }

    pub async fn run(&mut self) -> Result<()> {
        let res = self.req.sync(&self.next_batch).await?;
        //dbg!(res);
        Ok(())
    }
}
