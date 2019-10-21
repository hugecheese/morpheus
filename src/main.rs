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
#![allow(clippy::large_enum_variant)]
#![allow(clippy::module_inception)]
#![feature(decl_macro)]
#![feature(type_alias_impl_trait)]

mod client;
mod endpoints;
mod rest;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let mut c = client::Client::new(env!("MATRIX_TOKEN"));

    c.on_message(|msg| {
        println!("{}", msg.content);
    });

    c.run().await
}
