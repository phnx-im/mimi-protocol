// SPDX-FileCopyrightText: 2024 Phoenix R&D GmbH <hello@phnx.im>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlsMessage, RatchetTreeOption};

struct Frank {
    franking_tag: [u8; 32], // TODO: The RFC starts mixing camelCase and snake_case
    server_frank: [u8; 32],
    server_frank_context_hash: [u8; 32],
}

enum Mls10FanoutMessage {
    Application {
        timestamp: u64,
        message: MlsMessage,  // Must have wire_format = Application
        frank: Option<Frank>, // In RFC, optional doesn't have the <>
    },
    Welcome {
        timestamp: u64,
        message: MlsMessage, // Must have wire_format = Welcome
        ratchet_tree_option: RatchetTreeOption,
    },
}
