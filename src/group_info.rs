// SPDX-FileCopyrightText: 2024 Phoenix R&D GmbH <hello@phnx.im>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use serde_bytes::ByteBuf;

use super::{CipherSuite, GroupInfo, RatchetTreeOption};

// "to-be-signed"
#[repr(u8)]
pub enum GroupInfoRequestTBS {
    Mls10 {
        cipher_suite: CipherSuite,
        requesting_signature_key: SignaturePublicKey,
        requesting_credential: Credential,
        group_info_public_key: HPKEPublicKey,
        joining_code: Option<String>, // TODO: Or ByteBuf?
    } = 1,
}

// TODO: The signed variant's joining code is not optional?

// "to-be-encrypted"
pub struct GroupInfoRatchetTreeTBE {
    group_info: GroupInfo,
    ratchet_tree_option: RatchetTreeOption,
}

// "to-be-signed"
#[repr(u8)]
pub enum GroupInfoResponseTBS {
    Mls10 {
        status: GroupInfoCode,
        cipher_suite: CipherSuite,
        room_id: ByteBuf,
        hub_sender: ExternalSender,
        encrypted_groupinfo_and_tree: ByteBuf,
    } = 1,
}

// TODO: How to get signed/encrypted structs? Macros?
