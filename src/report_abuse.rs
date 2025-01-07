// SPDX-FileCopyrightText: 2025 Phoenix R&D GmbH <hello@phnx.im>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use serde_bytes::ByteBuf;

use crate::{frank::Frank, IdentifierUri};

pub struct AbusiveMessage {
    mimi_content: ByteBuf, // TODO: Why camelcase in rfc?
    frank: Frank,
}

pub enum AbuseType {
    Reserved = 0,
}

pub struct AbuseReport {
    reporting_user: IdentifierUri,
    alleged_abuser_uri: IdentifierUri, // TODO: Why uri in field name?
    reason_code: AbuseType,
    note: String, // TODO: Or ByteBuf?
    messages: Vec<AbusiveMessage>,
}
