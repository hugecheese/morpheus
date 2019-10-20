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
#[serde(deny_unknown_fields)]
pub struct PublicKey {
    key_validity_url: Option<String>,
    public_key: String,
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
pub struct DeviceLists {
    changed: Option<Vec<String>>,
    left: Option<Vec<String>>,
}
