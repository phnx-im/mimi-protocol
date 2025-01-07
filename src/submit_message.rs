// SPDX-FileCopyrightText: 2024 Phoenix R&D GmbH <hello@phnx.im>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use openmls::prelude::MlsMessageBodyOut;

use super::IdentifierUri;

#[repr(u8)]
pub enum SubmitMessageRequest {
    Mls10 {
        app_message: MlsMessageBodyOut, // Must be PrivateMessage
        sending_uri: IdentifierUri,
    } = 1,
}

#[repr(u8)]
pub enum SubmitResponseStatus {
    Success {
        // TODO: RFC also calls this "Accepted"
        accepted_timestamp: u64,
        server_frank: Option<[u8; 32]>,
    } = 0,
    NotAllowed {} = 1,
    EpochTooOld {
        current_epoch: u64,
    } = 2,
}

#[repr(u8)]
pub enum SubmitMessageResponse {
    Mls10 { status: SubmitResponseStatus } = 1,
}
