// Copyright 2020 - developers of the `grammers` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Custom types extending those provided by Telegram.
mod chats;
mod dialog;
mod entity;
mod entity_set;
mod input_message;
mod iter_buffer;
mod login_token;
mod message;
mod password_token;
mod update;

pub use chats::{AdminRightsBuilder, BannedRightsBuilder};
pub use dialog::Dialog;
pub use entity::Entity;
pub use entity_set::EntitySet;
pub(crate) use entity_set::Peer;
pub use input_message::InputMessage;
pub use iter_buffer::IterBuffer;
pub use login_token::LoginToken;
pub use message::Message;
pub use password_token::PasswordToken;
pub use update::Update;
