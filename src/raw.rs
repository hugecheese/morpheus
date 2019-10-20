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
    #[serde(flatten)]
    pub content: RoomEventContent,
    //#[serde(rename = "type")]
    //pub type_str: String,
    pub event_id: String,
    pub sender: String,
    pub origin_server_ts: u64,
    pub unsigned: Option<UnsignedData>,
}

// TODO: move to seperate file
// TODO: move to seperate file
// TODO: move to seperate file
#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "content")]
pub enum RoomEventContent {
    #[serde(rename = "m.room.aliases")]
    Aliases { aliases: Vec<String> },
    #[serde(rename = "m.room.canonical_alias")]
    CanonicalAlias { alias: String },
    #[serde(rename = "m.room.create")]
    Create {
        creator: String,
        #[serde(rename = "m.federate")]
        federate: Option<bool>,
        room_version: Option<String>,
        predecessor: Option<PreviousRoom>,
    },
    #[serde(rename = "m.room.join_rules")]
    JoinRules { join_rule: String },
    #[serde(rename = "m.room.member")]
    Member(EventContent),
    #[serde(rename = "m.room.power_levels")]
    PowerLevels {
        ban: Option<u32>,
        events: Option<HashMap<String, u32>>,
        events_default: Option<u32>,
        invite: Option<u32>,
        kick: Option<u32>,
        redact: Option<u32>,
        state_default: Option<u32>,
        users: Option<HashMap<String, u32>>,
        users_default: Option<u32>,
        notifications: Option<Notifications>,
    },
    #[serde(rename = "m.room.redaction")]
    Redaction { reason: Option<String> },
    #[serde(rename = "m.room.history_visibility")]
    HistoryVisibility { history_visibility: String },
    #[serde(rename = "m.room.guest_access")]
    GuestAccess { guest_access: String },
    #[serde(rename = "m.room.message")]
    Message {
        // TODO: redacted can be marked as m.room.message with an empty content
        // TODO: Bug with synapse?
        body: Option<String>,
        //msgtype: Option<String>,
        #[serde(flatten)]
        payload: Option<MessagePayload>,
    },
    #[serde(rename = "m.room.encryption")]
    Encryption {
        algorithm: String,
        rotation_period_ms: Option<u64>,
        rotation_period_msgs: Option<u32>,
    },
    #[serde(rename = "m.room.encrypted")]
    Encrypted {
        #[serde(flatten)]
        ciphertext: Ciphertext,
        sender_key: String,
        device_id: Option<String>,
        session_id: Option<String>,
    },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "algorithm", content = "ciphertext")]
pub enum Ciphertext {
    #[serde(rename = "m.olm.v1.curve25519-aes-sha2")]
    Olm(HashMap<String, CiphertextInfo>),
    #[serde(rename = "m.megolm.v1.aes-sha2")]
    Megolm(String),
}

#[derive(Deserialize, Debug)]
pub struct CiphertextInfo {
    body: String,
    #[serde(rename = "type")]
    olm_type: u64,
}

#[derive(Deserialize, Debug)]
pub struct TextPayload {
    pub format: Option<String>,
    pub formatted_body: Option<String>,
}

// TODO: move to seperate file
// TODO: move to seperate file
// TODO: move to seperate file
#[derive(Deserialize, Debug)]
#[serde(tag = "msgtype")]
pub enum MessagePayload {
    #[serde(rename = "m.text")]
    Text(TextPayload),
    #[serde(rename = "m.emote")]
    Emote(TextPayload),
    #[serde(rename = "m.notice")]
    Notice,
    #[serde(rename = "m.image")]
    Image {
        info: Option<ImageInfo>,
        url: Option<String>,
        file: Option<EncryptedFile>,
    },
    #[serde(rename = "m.file")]
    File {
        filename: Option<String>,
        info: Option<FileInfo>,
        url: Option<String>,
        file: Option<EncryptedFile>,
    },
    #[serde(rename = "m.audio")]
    Audio {
        info: Option<AudioInfo>,
        url: Option<String>,
        file: Option<EncryptedFile>,
    },
    #[serde(rename = "m.location")]
    Location {
        geo_uri: String,
        info: Option<LocationInfo>,
    },
    #[serde(rename = "m.video")]
    Video {
        info: Option<VideoInfo>,
        url: Option<String>,
        file: Option<EncryptedFile>,
    },
}

#[derive(Deserialize, Debug)]
pub struct ImageInfo {
    pub h: Option<u32>,
    pub w: Option<u32>,
    #[serde(flatten)]
    pub file: Option<FileInfo>, // TODO: does this need to be Option?
}

#[derive(Deserialize, Debug)]
pub struct ThumbnailInfo {
    pub h: Option<u32>,
    pub w: Option<u32>,
    pub mimetype: Option<String>,
    pub size: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct FileInfo {
    pub mimetype: Option<String>,
    pub size: Option<u64>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_file: Option<EncryptedFile>,
    pub thumbnail_info: Option<ThumbnailInfo>,
}

#[derive(Deserialize, Debug)]
pub struct AudioInfo {
    duration: Option<u64>,
    mimetype: Option<String>,
    size: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct LocationInfo {
    pub thumbnail_url: Option<String>,
    pub thumbnail_file: Option<EncryptedFile>,
    pub thumbnail_info: Option<ThumbnailInfo>,
}

#[derive(Deserialize, Debug)]
pub struct EncryptedFile {
    pub url: String,
    pub key: JsonWebKey,
    pub iv: String,
    pub hashes: HashMap<String, String>,
    pub v: String,
}

#[derive(Deserialize, Debug)]
pub struct JsonWebKey {
    pub kty: String,
    pub key_ops: Vec<String>,
    pub alg: String,
    pub k: String,
    pub ext: bool,
}

#[derive(Deserialize, Debug)]
pub struct VideoInfo {
    pub duration: Option<u64>,
    pub h: Option<u64>,
    pub w: Option<u64>,
    pub mimetype: String,
    pub size: Option<u64>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_file: Option<EncryptedFile>,
    pub thumbnail_info: Option<ThumbnailInfo>,
}

#[derive(Deserialize, Debug)]
pub struct PreviousRoom {
    pub room_id: String,
    pub event_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Notifications {
    pub room: u32,
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
