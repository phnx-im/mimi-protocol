// SPDX-FileCopyrightText: 2025 Phoenix R&D GmbH <hello@phnx.im>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use openmls::prelude::KeyPackage;
use serde_bytes::ByteBuf;

use crate::IdentifierUri;

#[repr(u8)]
pub enum ConsentEntry {
    // TODO: Factor our common fields
    Cancel {
        requester_uri: IdentifierUri,
        target_uri: IdentifierUri,
        room_id: Option<ByteBuf>,
    } = 0,
    Request {
        requester_uri: IdentifierUri,
        target_uri: IdentifierUri,
        room_id: Option<ByteBuf>,
    } = 1,
    Grant {
        requester_uri: IdentifierUri,
        target_uri: IdentifierUri,
        room_id: Option<ByteBuf>,
        client_key_packages: KeyPackage,
    } = 2,
    Revoke {
        requester_uri: IdentifierUri,
        target_uri: IdentifierUri,
        room_id: Option<ByteBuf>,
    } = 3,
}

pub struct ConsentScope {
    requester_uri: IdentifierUri,
    target_uri: IdentifierUri,
    room_id: Option<ByteBuf>,
}
