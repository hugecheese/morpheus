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
use super::{Room, User, InnerClient};
use std::rc::Rc;

pub struct Message {
    pub content: String,
    pub author: User,
    pub room: Room,
    pub client: Rc<InnerClient>,
}

impl Message {
    pub async fn reply(&self, content: &str) -> crate::Result<()> {
        self.client.send_message(&self.room.id, content).await?;
        Ok(())
    }
}
