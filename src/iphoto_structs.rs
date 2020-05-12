/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use filetime::FileTime;
use serde::de::{self, Deserializer, Visitor};
use serde_derive::Deserialize;

use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

struct FileTimeVisitor;

impl<'de> Visitor<'de> for FileTimeVisitor {
    type Value = FileTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between TODO and TODO")
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let seconds = (value.floor() + 978307200.0) as i64;
        let nanoseconds = (1e9 * value.fract()) as u32;
        Ok(FileTime::from_unix_time(seconds, nanoseconds))
    }
}

// deserialize CoreData time interval field into a FileTime struct
pub fn time_interval_to_filetime<'de, D>(deserializer: D) -> Result<FileTime, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_f64(FileTimeVisitor)
}

struct AlbumNameFilterVisitor;

impl<'de> Visitor<'de> for AlbumNameFilterVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a str")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value.trim().replace("/", ""))
    }
}

// deserialize album name, but strip out path characters that may be in names
pub fn filter_album_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(AlbumNameFilterVisitor)
}

#[derive(Deserialize)]
pub struct Album {
    #[serde(rename(deserialize = "AlbumId"))]
    pub id: u64,
    #[serde(rename(deserialize = "AlbumName"))]
    #[serde(deserialize_with = "filter_album_name")]
    pub name: String,
    #[serde(rename(deserialize = "KeyList"))]
    pub images: Vec<String>,
}

#[derive(Deserialize)]
pub struct Master {
    #[serde(rename(deserialize = "ImagePath"))]
    pub path: PathBuf,
    #[serde(rename(deserialize = "DateAsTimerIntervalGMT"))]
    #[serde(deserialize_with = "time_interval_to_filetime")]
    pub creation_date: FileTime,
    #[serde(rename(deserialize = "ModDateAsTimerInterval"))]
    #[serde(deserialize_with = "time_interval_to_filetime")]
    pub modification_date: FileTime,
    #[serde(rename(deserialize = "Caption"))]
    pub caption: String,
    #[serde(rename(deserialize = "Comment"))]
    pub comment: String,
}

#[derive(Deserialize)]
pub struct Library {
    #[serde(rename(deserialize = "Application Version"))]
    pub application_version: String,
    #[serde(rename(deserialize = "Archive Path"))]
    pub archive_path: PathBuf,
    #[serde(rename(deserialize = "ArchiveId"))]
    pub archive_id: String,
    #[serde(rename(deserialize = "Major Version"))]
    pub major_version: u64,
    #[serde(rename(deserialize = "Minor Version"))]
    pub minor_version: u64,
    #[serde(rename(deserialize = "List of Albums"))]
    pub albums: Vec<Album>,
    #[serde(rename(deserialize = "Master Image List"))]
    pub master_images: HashMap<String, Master>,
}
