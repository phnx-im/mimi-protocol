// SPDX-FileCopyrightText: 2024 Phoenix R&D GmbH <hello@phnx.im>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use openmls::prelude::MlsMessageBodyOut;

use crate::{frank::Frank, RatchetTreeOption};

pub enum Mls10FanoutMessage {
    Application {
        timestamp: u64,
        message: MlsMessageBodyOut, // Must have wire_format = Application
        frank: Option<Frank>,       // TODO: In RFC, optional doesn't have the <>
    },
    Welcome {
        timestamp: u64,
        message: MlsMessageBodyOut, // Must have wire_format = Welcome
        ratchet_tree_option: RatchetTreeOption,
    },
}
