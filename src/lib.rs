// SPDX-FileCopyrightText: 2024 Phoenix R&D GmbH <hello@phnx.im>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

mod group_info;
mod key_material;
mod notify;
mod submit_message;
mod update;

// TODO: Import this from mls crate
#[repr(u16)]
pub enum CipherSuite {
    Reserved = 0,
    MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519 = 1,
    Custom(u16),
}
// TODO: Import this from mls crate
pub type RequiredCapabilities = ();
// TODO: Import this from mls crate
pub type Capabilities = ();
// TODO: Import this from mls crate
pub type KeyPackage = ();
// TODO: Import this from mls crate
pub type MlsMessage = ();
// TODO: Import this from mls crate
pub type Welcome = ();
// TODO: Import this from mls crate
pub type GroupInfo = ();
// TODO: Import this from mls crate
pub type PartialGroupInfo = ();
// TODO: Import this from mls crate
pub type RatchetTreeOption = ();
// TODO: Import this from mls crate
pub type ProposalRef = ();

#[repr(u8)]
pub enum Protocol {
    Reserved = 0,
    Mls10 = 1,
    Custom(u8),
}

pub struct IdentifierUri(String); // TODO: Or ByteBuf?
