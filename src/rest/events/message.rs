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
use super::encryption::EncryptedFile;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "msgtype", deny_unknown_fields)]
pub enum Content {
    #[serde(rename = "m.text")]
    Text(Text),
    #[serde(rename = "m.emote")]
    Emote(Text),
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
pub struct Text {
    pub format: Option<String>,
    pub formatted_body: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ImageInfo {
    pub h: Option<u32>,
    pub w: Option<u32>,
    #[serde(flatten)]
    pub file: FileInfo,
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
    pub duration: Option<u64>,
    pub mimetype: Option<String>,
    pub size: Option<u64>,
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
pub struct VideoInfo {
    pub duration: Option<u64>,
    #[serde(flatten)]
    pub file: FileInfo,
}
