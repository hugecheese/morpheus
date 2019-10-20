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

// TODO: does everything need to be pub??

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Sync {
    next_batch: String,
    rooms: Option<Rooms>,
    presence: Option<Presence>,
    account_data: Option<AccountData>,
    to_device: Option<ToDevice>,
    device_lists: Option<DeviceLists>,
    device_one_time_keys_count: Option<HashMap<String, u32>>,
    groups: Option<serde_json::Value>, // TODO: UNDOCUMENTED IN SPEC
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
    events: Option<Vec<Event>>,
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
    pub content: RoomEventContent,
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
    pub events: Option<Vec<RoomEvent>>,
    pub limited: Option<bool>,
    pub prev_batch: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RoomEvent {
    #[serde(flatten)]
    pub content: RoomEventContent,
    pub event_id: String,
    pub sender: String,
    pub origin_server_ts: u64,
    pub unsigned: Option<UnsignedData>,
    pub state_key: Option<String>, // TODO: UNDOCUMENTED IN SPEC
    pub redacts: Option<String>,   // TODO: UNDOCUMENTED IN SPEC
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
    JoinRules { join_rule: JoinRule },
    #[serde(rename = "m.room.member")]
    Member(EventContent),
    #[serde(rename = "m.room.power_levels")]
    PowerLevels {
        ban: Option<i32>,
        events: Option<HashMap<String, i32>>,
        events_default: Option<i32>,
        invite: Option<i32>,
        kick: Option<i32>,
        redact: Option<i32>,
        state_default: Option<i32>,
        users: Option<HashMap<String, i32>>,
        users_default: Option<i32>,
        notifications: Option<Notifications>,
    },
    #[serde(rename = "m.room.redaction")]
    Redaction { reason: Option<String> },
    #[serde(rename = "m.room.history_visibility")]
    HistoryVisibility {
        history_visibility: HistoryVisibility,
    },
    #[serde(rename = "m.room.guest_access")]
    GuestAccess { guest_access: GuestAccess },
    #[serde(rename = "m.room.message")]
    Message {
        // TODO: these fields shouldn't be marked as Option as they are
        // required by the spec. However, when they are redacted, an empty
        // content is served, which is stupidly annoying to parse.
        body: Option<String>,
        //msgtype: Option<String>,
        #[serde(flatten)]
        payload: Option<MessagePayload>,
    },
    #[serde(rename = "m.room.name")]
    Name { name: String },
    #[serde(rename = "m.room.topic")]
    Topic { topic: String },
    #[serde(rename = "m.room.avatar")]
    Avatar { info: Option<FileInfo>, url: String },
    #[serde(rename = "m.room.third_party_invite")]
    ThirdPartyInvite {
        // TODO: these fields shouldn't be marked as Option as they are
        // required by the spec. However, when they are redacted, an empty
        // content is served, which is stupidly annoying to parse.
        display_name: Option<String>,
        key_validity_url: Option<String>,
        public_key: Option<String>,
        public_keys: Option<Vec<PublicKey>>,
    },
    #[serde(rename = "m.room.encryption")]
    Encryption {
        algorithm: String,
        rotation_period_ms: Option<u64>,
        rotation_period_msgs: Option<u32>,
    },
    // TODO: other encryption events like m.room_key
    #[serde(rename = "m.room.encrypted")]
    Encrypted {
        #[serde(flatten)]
        ciphertext: Ciphertext,
        sender_key: String,
        device_id: Option<String>,
        session_id: Option<String>,
    },
    #[serde(rename = "m.room.pinned_events")]
    PinnedEvents { pinned: Vec<String> },
    #[serde(rename = "m.room.server_acl")]
    ServerAcl {
        allow_ip_literals: Option<bool>,
        allow: Option<Vec<String>>,
        deny: Option<Vec<String>>,
    },
    #[serde(rename = "m.room.related_groups")] // TODO: UNDOCUMENTED IN SPEC
    RelatedGroups { groups: Option<Vec<String>> }, // TODO: UNDOCUMENTED IN SPEC

    #[serde(rename = "im.vector.modular.widgets")] // TODO DELETE ME
    _DELETEME1(serde_json::Value), // TODO DELETE ME
    #[serde(rename = "org.matrix.room.preview_urls")] // TODO DELETE ME
    _DELETEME2(serde_json::Value), // TODO DELETE ME
    #[serde(rename = "m.room.bridging")] // TODO DELETE ME
    _DELETEME3(serde_json::Value), // TODO DELETE ME
    #[serde(rename = "m.room.plumbing")]
    _DELETEME4(serde_json::Value), // TODO DELETE ME
    #[serde(rename = "m.room.bot.options")]
    _DELETEME5(serde_json::Value), // TODO DELETE ME
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PublicKey {
    key_validity_url: Option<String>,
    public_key: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum HistoryVisibility {
    Invited,
    Joined,
    Shared,
    WorldReadable,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum GuestAccess {
    CanJoin,
    Forbidden,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum JoinRule {
    Public,
    Knock,
    Invite,
    Private,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "algorithm", content = "ciphertext", deny_unknown_fields)]
pub enum Ciphertext {
    #[serde(rename = "m.olm.v1.curve25519-aes-sha2")]
    Olm(HashMap<String, CiphertextInfo>),
    #[serde(rename = "m.megolm.v1.aes-sha2")]
    Megolm(String),
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CiphertextInfo {
    body: String,
    #[serde(rename = "type")]
    olm_type: u64,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TextPayload {
    pub format: Option<String>,
    pub formatted_body: Option<String>,
}

// TODO: move to seperate file
// TODO: move to seperate file
// TODO: move to seperate file
#[derive(Deserialize, Debug)]
#[serde(tag = "msgtype", deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct ImageInfo {
    pub h: Option<u32>,
    pub w: Option<u32>,
    #[serde(flatten)]
    pub file: Option<FileInfo>, // TODO: does this need to be Option?
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ThumbnailInfo {
    pub h: Option<u32>,
    pub w: Option<u32>,
    pub mimetype: Option<String>,
    pub size: Option<u64>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FileInfo {
    pub mimetype: Option<String>,
    pub size: Option<u64>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_file: Option<EncryptedFile>,
    pub thumbnail_info: Option<ThumbnailInfo>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AudioInfo {
    duration: Option<u64>,
    mimetype: Option<String>,
    size: Option<u64>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct LocationInfo {
    pub thumbnail_url: Option<String>,
    pub thumbnail_file: Option<EncryptedFile>,
    pub thumbnail_info: Option<ThumbnailInfo>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EncryptedFile {
    pub url: String,
    pub key: JsonWebKey,
    pub iv: String,
    pub hashes: HashMap<String, String>,
    pub v: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonWebKey {
    pub kty: String,
    pub key_ops: Vec<String>,
    pub alg: String,
    pub k: String,
    pub ext: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct VideoInfo {
    pub duration: Option<u64>,
    #[serde(flatten)]
    pub file: Option<FileInfo>, // TODO: is Option necessary?
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PreviousRoom {
    pub room_id: String,
    pub event_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Notifications {
    pub room: i32,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct UnsignedData {
    pub age: Option<u64>,
    pub redacted_because: Option<Event>,
    pub transaction_id: Option<String>,
    pub prev_sender: Option<String>, // TODO: UNDOCUMENTED IN SPEC
    pub replaces_state: Option<String>, // TODO: UNDOCUMENTED IN SPEC
    // TODO: this should be an Option<EventContent> but that would make the two
    // types infinitely recursive
    pub prev_content: Option<serde_json::Value>, // TODO: UNDOCUMENTED IN SPEC
    pub redacted_by: Option<String>,             // TODO: UNDOCUMENTED IN SPEC
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
pub struct Event {
    #[serde(flatten)]
    pub content: OtherEventData,

    // TODO DRAGONS BEWARE!
    // TODO DRAGONS BEWARE!
    // The following fields are only present for "redacted_because"
    // even though it's specified as a normal "Event" which should only have
    // "content" and "type".
    pub event_id: Option<String>, // TODO: UNDOCUMENTED IN SPEC
    pub origin_server_ts: Option<u64>, // TODO: UNDOCUMENTED IN SPEC
    pub redacts: Option<String>,  // TODO: UNDOCUMENTED IN SPEC
    pub sender: Option<String>,   // TODO: UNDOCUMENTED IN SPEC
    // TODO: should be Option<UnsignedData> but would be infinitely recursive
    pub unsigned: Option<serde_json::Value>, // TODO: UNDOCUMENTED IN SPEC
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "content", deny_unknown_fields)]
// TODO: actual enum name??
pub enum OtherEventData {
    #[serde(rename = "m.typing")]
    Typing { user_ids: Vec<String> },
    // TODO: this is not documented in the spec (remove Option?)
    #[serde(rename = "m.accepted_terms")]
    AcceptedTerms { accepted: Option<Vec<String>> },
    #[serde(rename = "m.direct")]
    Direct(HashMap<String, Vec<String>>),
    #[serde(rename = "m.push_rules")]
    PushRules {
        global: Option<Ruleset>,
        device: Option<Ruleset>, // TODO: UNDOCUMENTED IN SPEC
    },
    #[serde(rename = "m.fully_read")]
    FullyRead { event_id: String },
    #[serde(rename = "m.receipt")]
    Receipt(Option<HashMap<String, Receipts>>),
    #[serde(rename = "m.room.redaction")]
    Redaction { reason: Option<String> },

    // TODO: remove dummies after event probing is complete
    #[serde(rename = "im.vector.web.settings")]
    _DUMMY1(serde_json::Value),
    #[serde(rename = "im.vector.riot.breadcrumb_rooms")]
    _DUMMY2(serde_json::Value),
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Receipts {
    #[serde(rename = "m.read")]
    pub read: Option<HashMap<String, Option<Receipt>>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    pub ts: Option<u64>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Ruleset {
    pub content: Option<Vec<PushRule>>,
    #[serde(rename = "override")]
    pub overriden: Option<Vec<PushRule>>,
    pub room: Option<Vec<PushRule>>,
    pub sender: Option<Vec<PushRule>>,
    pub underride: Option<Vec<PushRule>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PushRule {
    // TODO: implementation of this stupid spec rule
    pub actions: serde_json::Value,
    pub default: bool,
    pub enabled: bool,
    pub rule_id: String,
    pub conditions: Option<Vec<PushCondition>>,
    pub pattern: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PushCondition {
    pub kind: String,
    pub key: Option<String>,
    pub pattern: Option<String>,
    pub is: Option<String>,
}

// TODO DIFFERENT FILE FOR THIS DATA
// TODO DIFFERENT FILE FOR THIS DATA
// TODO DIFFERENT FILE FOR THIS DATA
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventContent {
    pub avatar_url: Option<String>,
    pub displayname: Option<String>,
    pub membership: Membership,
    pub is_direct: Option<bool>,
    pub third_party_invite: Option<Invite>,
    pub unsigned: Option<UnsignedData>,
    pub kind: Option<String>, // TODO: UNDOCUMENTED IN SPEC, seen as "guest"
    pub inviter: Option<String>, // TODO: UNDOCUMENTED IN SPEC
    pub reason: Option<String>, // TODO: UNDOCUMENTED IN SPEC
    #[serde(rename = "uk.half-shot.discord.member")] // TODO: UNDOCUMENTED
    pub deleteme1: Option<serde_json::Value>, // TODO: UNDOCUMENTED IN SPEC
    pub third_party_signed: Option<serde_json::Value>, // TODO: UNDOCUMENTED IN SPEC
    #[serde(rename = "")] // TODO: UNDOCUMENTED
    pub deleteme2: Option<serde_json::Value>, // TODO: UNDOCUMENTED IN SPEC
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Membership {
    Invite,
    Join,
    Knock,
    Leave,
    Ban,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Invite {
    pub display_name: String,
    pub signed: Option<serde_json::Value>, // TODO: REMOVE ME
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ToDevice {
    pub events: Option<Vec<ToDeviceEvent>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ToDeviceEvent {
    pub content: Option<EventContent>,
    #[serde(rename = "type")]
    pub type_str: Option<String>,
    pub sender: Option<String>,
}

// TODO PUT THESE IN E2EE FILE
// TODO PUT THESE IN E2EE FILE
// TODO PUT THESE IN E2EE FILE
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DeviceLists {
    changed: Option<Vec<String>>,
    left: Option<Vec<String>>,
}
