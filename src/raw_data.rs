/*
 * Morpheus is a client for the Matrix open standard.
 * Copyright (C) 2019 Morpheus Contributors
 *
 * Morpheus is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License, version 3,
 * as published by the Free Software Foundation.

 * Morpheus is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.

 * You should have received a copy of the GNU Affero General Public License
 * along with Morpheus. If not, see <https://www.gnu.org/licenses/>.
 */
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Sync {
    pub rooms: Rooms,
}

#[derive(Deserialize, Debug)]
pub struct Rooms {
    pub join: HashMap<String, JoinedRoom>,
}

#[derive(Deserialize, Debug)]
pub struct JoinedRoom {
    pub state: RoomState,
}

#[derive(Deserialize, Debug)]
pub struct RoomState {
    pub events: Vec<RoomEvent>,
}

#[derive(Deserialize, Debug)]
pub struct RoomEvent {
    #[serde(rename = "type")]
    pub type_str: String,
    pub event_id: String,
    pub sender: String,
    pub origin_server_ts: u64,
}
