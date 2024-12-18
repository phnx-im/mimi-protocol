// SPDX-FileCopyrightText: 2024 Phoenix R&D GmbH <hello@phnx.im>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{GroupInfo, MlsMessage, PartialGroupInfo, ProposalRef, RatchetTreeOption, Welcome};

pub enum HandshakeBundle {
    Commit {
        proposal_or_commit: MlsMessage, // TODO: This must be a commit. Can we make this type safe?
        welcome: Option<Welcome>,
        group_info_option: GroupInfoOption,
        ratchet_tree_option: RatchetTreeOption,
    },
    Proposal {
        proposal_or_commit: MlsMessage, // This must be a proposal
        more_proposals: Vec<MlsMessage>,
    },
}

#[repr(u8)]
pub enum GroupInfoOption {
    Full {
        group_info: GroupInfo,
    } = 1,
    Partial {
        partial_group_info: PartialGroupInfo,
    } = 2,
}

// TODO: Can the RFC rename this to UpdateRoomRequest?
pub enum UpdateRequest {
    // Version is not transmitted, use the version of the room
    Mls10 { bundle: HandshakeBundle },
}

#[repr(u8)]
pub enum UpdateRoomResponse {
    // This uses the version of the room
    Success {
        error_description: String, // TODO: Switch error_description and response_code in RFC? But why does Success need it?
        accepted_timestamp: u64,
    } = 0,
    WrongEpoch {
        error_description: String,
        current_epoch: u64,
    } = 1,
    NotAllowed {
        error_description: String,
    } = 2,
    InvalidProposal {
        error_description: String,
        invalid_proposals: Vec<ProposalRef>,
    } = 3,
}
