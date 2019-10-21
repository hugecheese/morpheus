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
use super::{Message, User};
use crate::rest;
// TODO: make better-er
use crate::rest::events::room::Content;
use crate::Result;

pub struct Client {
    req: rest::Client,
    message_handlers: Vec<fn(&Message)>,
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

    pub fn on_message(&mut self, handler: fn(&Message)) {
        self.message_handlers.push(handler);
    }

    fn handle_sync(&mut self, sync: &rest::events::Sync) -> Option<()> {
        // TODO: fix for this absurd as_ref spam
        for (id, room) in sync.rooms.as_ref()?.join.as_ref()? {
            for event in room.timeline.as_ref()?.events.as_ref()? {
                let msg = match &event.content {
                    Content::Message { body, .. } => body,
                    _ => continue,
                };

                for handler in &self.message_handlers {
                    handler(&Message {
                        content: msg.clone()?.into(),
                        author: User {},
                    });
                }
            }
        }

        Some(())
    }

    pub async fn run(&mut self) -> Result<()> {
        let res = self.req.sync(&self.next_batch).await?;
        self.next_batch = res.next_batch;

        loop {
            let res = self.req.sync(&self.next_batch).await?;
            self.handle_sync(&res);
            self.next_batch = res.next_batch;
        }
    }
}
