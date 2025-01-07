// SPDX-FileCopyrightText: 2024 Phoenix R&D GmbH <hello@phnx.im>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use openmls::prelude::{Capabilities, Ciphersuite, KeyPackage, RequiredCapabilitiesExtension};

use super::IdentifierUri;

#[repr(u8)]
pub enum KeyMaterialRequest {
    Mls10 {
        requesting_user: IdentifierUri,
        target_user: IdentifierUri,
        room_id: IdentifierUri,
        acceptable_ciphersuites: Vec<Ciphersuite>,
        required_capabilities: RequiredCapabilitiesExtension,
    } = 1,
}

#[repr(u8)]
pub enum KeyMaterialUserCode {
    Success = 0,
    PartialSuccess = 1,
    IncompatibleProtocol = 2,
    NoCompatibleMaterial = 3,
    UserUnknown = 4,
    NoConsent = 5,
    NoConsentForThisRoom = 6,
    UserDeleted = 7,
    Custom(u8),
}

#[repr(u8)]
pub enum Mls10ClientKeyMaterial {
    Success {
        // TODO: Ask if field order of client_status and client_uri can be swapped
        client_uri: IdentifierUri,
        key_package: KeyPackage,
    } = 0,
    KeyMaterialExhausted {
        client_uri: IdentifierUri,
    } = 1,
    NothingCompatible {
        client_uri: IdentifierUri,
        client_capabilities: Option<Capabilities>,
    } = 2,
}

#[repr(u8)]
pub enum KeyMaterialResponse {
    Mls10 {
        user_status: KeyMaterialUserCode,
        user_uri: IdentifierUri,
        clients: Vec<Mls10ClientKeyMaterial>,
    } = 1,
}
