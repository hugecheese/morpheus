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
use super::notifs::Ruleset;
use serde::Deserialize;
use std::collections::HashMap;

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
    #[serde(rename = "m.widgets")] // TODO: UNDOCUMENTED IN SPEC
    Widgets { // TODO: UNDOCUMENTED IN SPEC
        name: Option<String>, // TODO: UNDOCUMENTED IN SPEC
        url: Option<String>, // TODO: UNDOCUMENTED IN SPEC
        #[serde(rename = "type")] // TODO: UNDOCUMENTED IN SPEC
        type_str: Option<String>, // TODO: UNDOCUMENTED IN SPEC
    }, // TODO: UNDOCUMENTED IN SPEC

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
// TODO: maybe a better name for this struct?
pub struct EventContent {
    pub avatar_url: Option<String>,
    pub displayname: Option<String>,
    pub membership: Option<Membership>, // TODO: this shouldn't be optional
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
    pub join_rule: Option<serde_json::Value>, // TODO: UNDOCUMENTED IN SPEC
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
    pub signed: Option<serde_json::Value>, // TODO: IMPLEMENT ME
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
