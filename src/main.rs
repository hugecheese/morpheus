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
#![allow(dead_code)]
#![feature(decl_macro)]
#![feature(type_alias_impl_trait)]

mod client;
mod endpoints;
mod message;
mod raw;
mod rest_client;
mod user;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use client::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let c = rest_client::RestClient::new(env!("MATRIX_TOKEN"));
    println!("{:?}", c.sync().await);

    Ok(())
}
