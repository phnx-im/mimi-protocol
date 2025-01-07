// SPDX-FileCopyrightText: 2024 Phoenix R&D GmbH <hello@phnx.im>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use openmls::prelude::{
    group_info::GroupInfo, Ciphersuite, Credential, ExternalSender, HpkePublicKey,
    SignaturePublicKey,
};
use serde_bytes::ByteBuf;

use super::RatchetTreeOption;

#[repr(u8)]
pub enum GroupInfoRequest {
    Mls10 {
        cipher_suite: Ciphersuite,
        requesting_signature_key: SignaturePublicKey,
        requesting_credential: Credential,
        group_info_public_key: HpkePublicKey,
        joining_code: Option<String>, // TODO: Or ByteBuf?
        signature: ByteBuf, // TODO: Or only create signature when serializing, for example macro #[signature]?
    } = 1,
}

// TODO: The signed variant's joining code is not optional?

// TODO: typo groupInfoPubl[i]cKey

#[repr(u8)]
pub enum GroupInfoCode {
    Reserved = 0,
    Success = 1,
    NotAuthorized = 2,
    NoSuchRoom = 3,
    Custom(u8),
}

// "to-be-encrypted"
pub struct GroupInfoRatchetTreeTBE {
    group_info: GroupInfo,
    ratchet_tree_option: RatchetTreeOption,
}

// "to-be-signed"
#[repr(u8)]
pub enum GroupInfoResponse {
    Mls10 {
        status: GroupInfoCode,
        cipher_suite: Ciphersuite,
        room_id: ByteBuf,
        hub_sender: ExternalSender,
        encrypted_groupinfo_and_tree: ByteBuf,
        signature: ByteBuf, // TODO: Or only create signature when serializing, for example macro #[signature]?
    } = 1,
}

// TODO: How to get signed/encrypted structs? Macros?
