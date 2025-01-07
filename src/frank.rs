// SPDX-FileCopyrightText: 2025 Phoenix R&D GmbH <hello@phnx.im>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

pub struct Frank {
    franking_tag: [u8; 32], // TODO: The RFC starts mixing camelCase and snake_case
    server_frank: [u8; 32],
    server_frank_context_hash: [u8; 32],
}
