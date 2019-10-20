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
