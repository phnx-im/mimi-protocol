// SPDX-FileCopyrightText: 2025 Phoenix R&D GmbH <hello@phnx.im>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use serde_bytes::ByteBuf;

use crate::IdentifierUri;

#[repr(u8)]
pub enum IdentifierRequest {
    Reserved {
        search_value: String,
    } = 0,
    Handle {
        search_value: String,
    } = 1,
    Nick {
        search_value: String,
    } = 2,
    Email {
        search_value: String,
    } = 3,
    Phone {
        search_value: String,
    } = 4,
    PartialName {
        search_value: String,
    } = 5,
    WholeProfile {
        search_value: String,
    } = 6,
    OidcStdClaim {
        search_value: String,
        claim_name: String,
    } = 7,
    VcardField {
        search_value: String,
        field_name: String,
    } = 8,
}

#[repr(u8)]
pub enum IdentifierQueryCode {
    Success = 0,
    NotFound = 1,
    Ambiguous = 2,
    Forbidden = 3,
    UnsupportedField = 4,
    Custom(u8),
}

#[repr(u8)]
pub enum FieldSource {
    Reserved = 0,
    OidcStdClaim = 7,
    VCardField = 8,
    Custom(u8),
}

struct ProfileField {
    field_source: FieldSource,
    field_name: String, // TODO: Why does it say string in the rfc instead of opaque <V>
    field_value: ByteBuf,
}

struct UserProfile {
    stable_uri: IdentifierUri,
    fields: Vec<ProfileField>,
}

pub struct IdentifierResponse {
    response_code: IdentifierQueryCode,
    uri: Vec<IdentifierUri>,
    found_profiles: Vec<UserProfile>,
}
