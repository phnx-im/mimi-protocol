// SPDX-FileCopyrightText: 2024 Phoenix R&D GmbH <hello@phnx.im>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

mod consent;
mod frank;
mod group_info;
mod identifier_query;
mod key_material;
mod notify;
mod report_abuse;
mod submit_message;
mod update;

// TODO: openmls doesn't implement this yet?
// https://datatracker.ietf.org/doc/draft-mahy-mls-ratchet-tree-options/
type RatchetTreeOption = ();
type PartialGroupInfo = ();

#[repr(u8)]
pub enum Protocol {
    Reserved = 0,
    Mls10 = 1,
    Custom(u8),
}

pub struct IdentifierUri(String); // TODO: Or ByteBuf?
