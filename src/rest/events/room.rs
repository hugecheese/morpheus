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
use super::encryption::{Ciphertext, PublicKey};
use super::event::{EventContent, UnsignedData};
use super::message;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Event {
    #[serde(flatten)]
    pub content: Content,
    pub event_id: String,
    pub sender: String,
    pub origin_server_ts: u64,
    pub unsigned: Option<UnsignedData>,
    pub state_key: Option<String>, // TODO: UNDOCUMENTED IN SPEC
    pub redacts: Option<String>,   // TODO: UNDOCUMENTED IN SPEC
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "content")]
pub enum Content {
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
        #[serde(flatten)]
        content: Option<message::Content>,
    },
    #[serde(rename = "m.room.name")]
    Name { name: String },
    #[serde(rename = "m.room.topic")]
    Topic { topic: String },
    #[serde(rename = "m.room.avatar")]
    Avatar {
        info: Option<message::FileInfo>,
        url: String,
    },
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
pub struct Notifications {
    pub room: i32,
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
#[serde(deny_unknown_fields)]
pub struct PreviousRoom {
    pub room_id: String,
    pub event_id: String,
}
