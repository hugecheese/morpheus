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
use super::encryption::DeviceLists;
use super::event::{Event, EventContent, UnsignedData};
use super::room;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Sync {
    pub next_batch: String,
    pub rooms: Option<Rooms>,
    pub presence: Option<Presence>,
    pub account_data: Option<AccountData>,
    pub to_device: Option<ToDevice>,
    pub device_lists: Option<DeviceLists>,
    pub device_one_time_keys_count: Option<HashMap<String, u32>>,
    pub groups: Option<serde_json::Value>, // TODO: UNDOCUMENTED IN SPEC
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Rooms {
    pub join: Option<HashMap<String, JoinedRoom>>,
    pub invite: Option<HashMap<String, InvitedRoom>>,
    pub leave: Option<HashMap<String, LeftRoom>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JoinedRoom {
    pub summary: Option<RoomSummary>,
    pub state: Option<State>,
    pub timeline: Option<Timeline>,
    pub ephemeral: Option<Ephemeral>,
    pub account_data: Option<AccountData>,
    pub unread_notifications: Option<UnreadNotificationCounts>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RoomSummary {
    #[serde(rename = "m.heroes")]
    pub heroes: Option<Vec<String>>,
    #[serde(rename = "m.joined_member_count")]
    pub joined_member_count: Option<u32>,
    #[serde(rename = "m.invited_member_count")]
    pub invited_member_count: Option<u32>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Ephemeral {
    pub events: Option<Vec<Event>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct UnreadNotificationCounts {
    pub highlight_count: Option<u32>,
    pub notification_count: Option<u32>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct InvitedRoom {
    pub invite_state: Option<InviteState>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct InviteState {
    pub events: Option<Vec<StrippedState>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct StrippedState {
    pub content: EventContent,
    pub state_key: String,
    // TODO: this type should have some effect on the event content, right?
    #[serde(rename = "type")]
    pub type_str: String,
    pub sender: String,
    pub event_id: Option<String>, // TODO: UNDOCUMENTED IN SPEC
    pub origin_server_ts: Option<u64>, // TODO: UNDOCUMENTED IN SPEC
    pub unsigned: Option<serde_json::Value>, // TODO: UNDOCUMENTED IN SPEC
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct LeftRoom {
    pub state: Option<State>,
    pub timeline: Option<Timeline>,
    pub account_data: Option<AccountData>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub events: Option<Vec<StateEvent>>,
}

#[derive(Deserialize, Debug)]
pub struct StateEvent {
    #[serde(flatten)]
    pub content: room::Content,
    pub event_id: String,
    pub sender: String,
    pub origin_server_ts: u64,
    pub unsigned: Option<UnsignedData>,
    pub prev_content: Option<EventContent>,
    pub state_key: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Timeline {
    pub events: Option<Vec<room::Event>>,
    pub limited: Option<bool>,
    pub prev_batch: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Presence {
    pub events: Option<Vec<Event>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AccountData {
    pub events: Option<Vec<Event>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ToDevice {
    pub events: Option<Vec<ToDeviceEvent>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ToDeviceEvent {
    pub content: Option<serde_json::Value>, // TODO: actually implement fields
    #[serde(rename = "type")]
    pub type_str: Option<String>,
    pub sender: Option<String>,
}
