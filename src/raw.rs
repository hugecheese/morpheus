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
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Sync {
    pub next_batch: String,
    pub rooms: Option<Rooms>,
    pub presence: Option<Presence>,
    pub account_data: Option<AccountData>,
    pub to_device: Option<ToDevice>,
    pub device_lists: Option<DeviceLists>,
    pub device_one_time_keys_count: Option<HashMap<String, u32>>,
}

#[derive(Deserialize, Debug)]
pub struct Rooms {
    pub join: Option<HashMap<String, JoinedRoom>>,
    pub invite: Option<HashMap<String, InvitedRoom>>,
    pub leave: Option<HashMap<String, LeftRoom>>,
}

#[derive(Deserialize, Debug)]
pub struct JoinedRoom {
    pub summary: Option<RoomSummary>,
    pub state: Option<State>,
    pub timeline: Option<Timeline>,
    pub ephemeral: Option<Ephemeral>,
    pub account_data: Option<AccountData>,
    pub unread_notifications: Option<UnreadNotificationCounts>,
}

#[derive(Deserialize, Debug)]
pub struct RoomSummary {
    #[serde(rename = "m.heroes")]
    pub heroes: Option<Vec<String>>,
    #[serde(rename = "m.joined_member_count")]
    pub joined_member_count: Option<u32>,
    #[serde(rename = "m.invited_member_count")]
    pub invited_member_count: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Ephemeral {
    events: Option<Vec<Event>>,
}

#[derive(Deserialize, Debug)]
pub struct UnreadNotificationCounts {
    pub highlight_count: Option<u32>,
    pub notification_count: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct InvitedRoom {
    pub invite_state: Option<InviteState>,
}

#[derive(Deserialize, Debug)]
pub struct InviteState {
    pub events: Option<Vec<StrippedState>>,
}

#[derive(Deserialize, Debug)]
pub struct StrippedState {
    pub content: EventContent,
    pub state_key: String,
    #[serde(rename = "type")] // TODO enum setup
    pub type_str: String,
    pub sender: String,
}

#[derive(Deserialize, Debug)]
pub struct LeftRoom {
    pub state: Option<State>,
    pub timeline: Option<Timeline>,
    pub account_data: Option<AccountData>,
}

#[derive(Deserialize, Debug)]
pub struct State {
    pub events: Option<Vec<StateEvent>>,
}

#[derive(Deserialize, Debug)]
pub struct StateEvent {
    // TODO pub content: Object,
    #[serde(rename = "type")]
    pub type_str: String,
    pub event_id: String,
    pub sender: String,
    pub origin_server_ts: u64,
    pub unsigned: Option<UnsignedData>,
    pub prev_content: Option<EventContent>,
    pub state_key: String,
}

#[derive(Deserialize, Debug)]
pub struct Timeline {
    pub events: Option<Vec<RoomEvent>>,
    pub limited: Option<bool>,
    pub prev_batch: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RoomEvent {
    // TODO pub content: Object,
    #[serde(rename = "type")] // TODO enum setup
    pub type_str: String,
    pub event_id: String,
    pub sender: String,
    pub origin_server_ts: u64,
    pub unsigned: Option<UnsignedData>,
}

#[derive(Deserialize, Debug)]
pub struct UnsignedData {
    pub age: Option<u64>,
    pub redacted_because: Option<Event>,
    pub transaction_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Presence {
    pub events: Option<Vec<Event>>,
}

#[derive(Deserialize, Debug)]
pub struct AccountData {
    pub events: Option<Vec<Event>>,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    // TODO pub content: Object,
    #[serde(rename = "type")] // TODO enum setup
    pub type_str: String,
}

// TODO DIFFERENT FILE FOR THIS DATA
// TODO DIFFERENT FILE FOR THIS DATA
// TODO DIFFERENT FILE FOR THIS DATA
#[derive(Deserialize, Debug)]
pub struct EventContent {
    pub avatar_url: Option<String>,
    pub displayname: Option<String>,
    pub membership: String, // TODO: enum?
    pub is_direct: Option<bool>,
    pub third_party_invite: Option<Invite>,
    pub unsigned: Option<UnsignedData>,
}

#[derive(Deserialize, Debug)]
pub struct Invite {
    pub display_name: String,
    // TODO pub signed: signed,
}

#[derive(Deserialize, Debug)]
pub struct ToDevice {
    pub events: Option<Vec<ToDeviceEvent>>,
}

#[derive(Deserialize, Debug)]
pub struct ToDeviceEvent {
    pub content: Option<EventContent>,
    #[serde(rename = "type")] // TODO enum setup
    pub type_str: Option<String>,
    pub sender: Option<String>,
}

// TODO PUT THESE IN E2EE FILE
// TODO PUT THESE IN E2EE FILE
// TODO PUT THESE IN E2EE FILE
#[derive(Deserialize, Debug)]
pub struct DeviceLists {
    changed: Option<Vec<String>>,
    left: Option<Vec<String>>,
}
