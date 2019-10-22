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
use super::{Message, Room, User};
use crate::rest;
// TODO: make better-er
use crate::rest::events::room::Content;
use crate::Result;
use std::pin::Pin;
use std::rc::Rc;

type Future = Pin<Box<dyn std::future::Future<Output = Result<()>>>>;

pub struct InnerClient {
    pub req: rest::Client,
}

// TODO: innerclient??? this is DISGUSTING. REFACTOR!!!
impl InnerClient {
    // TODO: make it return the actual type
    pub async fn send_message(
        &self,
        room_id: &str,
        content: &str,
    ) -> crate::Result<()> {
        self.req.send_message().body(content).send(room_id).await?;
        Ok(())
    }
}

pub struct Client {
    message_handlers: Vec<fn(Message) -> Future>,
    next_batch: String,
    data: Rc<self::InnerClient>,
}

impl Client {
    pub fn new(token: &str) -> Self {
        Self {
            data: Rc::new(self::InnerClient {
                req: rest::Client::new(token),
            }),
            message_handlers: Vec::new(),
            next_batch: String::new(),
        }
    }

    pub fn on_message(&mut self, handler: fn(Message) -> Future) {
        self.message_handlers.push(handler);
    }

    async fn handle_sync(&mut self, sync: rest::events::Sync) -> Option<()> {
        self.next_batch = sync.next_batch;

        for (id, room) in sync.rooms?.join? {
            for event in room.timeline?.events? {
                let msg = match event.content {
                    Content::Message { body, .. } => body?,
                    _ => continue,
                };

                for handler in &self.message_handlers {
                    // TODO: this is super gross
                    let res = handler(Message {
                        // TODO: avoid clone somehow?
                        content: msg.clone(),
                        author: User {},
                        room: Room { id: id.clone() },
                        client: Rc::clone(&self.data),
                    })
                    .await;

                    // TODO: this is also super gross
                    match res {
                        Ok(_) => (),
                        Err(e) => {
                            dbg!(&e);
                        }
                    }
                }
            }
        }

        Some(())
    }

    pub async fn run(&mut self) -> Result<()> {
        let res = self.data.req.sync().send().await?;
        self.next_batch = res.next_batch;

        loop {
            let res =
                self.data.req.sync().since(&self.next_batch).send().await?;
            self.handle_sync(res).await;
        }
    }
}
