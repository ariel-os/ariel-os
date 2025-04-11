//! A CoAP security tool for embedded devices, supporting OSCORE/EDHOC and managing credentials.
//!
//! This crate is under active development; breaking changes will be made as necessary. It
//! currently only handles the server side of CoAP exchanges. At runtime, there is more copying of
//! messages than is generally preferred; those result from limitations of underlying tools and are
//! being addressed there.
//!
//! This crate builds on several components technically and logically:
//!
//! * [libOSCORE](https://gitlab.com/oscore/liboscore/) provides the OSCORE implementation.
//! * [Lakers](https://github.com/openwsn-berkeley/lakers) provides the EDHOC implementation.
//! * The combined handling of OSCORE and EDHOC was originally explored in [EDF's CoAP/ACE-OAuth
//!   proof-of-concept firmware](https://gitlab.com/oscore/coap-ace-poc-firmware/). Since this
//!   crate matured, that firmware now uses coapcore.
//! * The crate is maintained as part of [Ariel OS](https://ariel-os.org/), whose CoAP stack
//!   integrates it and [manages server access
//!   policies](https://ariel-os.github.io/ariel-os/dev/docs/book/tooling/coap.html#server-access-policy).
//!   Nothing in this crate depends on Ariel OS, but some examples may refer to it.
//!
//! # Usage
//!
//! This crate is mainly used with a CoAP stack (something that takes a [`coap_handler::Handler`])
//! and a CoAP server application (an implementation of a [`coap_handler::Handler`]). Rather than
//! passing the handler directly to the stack (which then only applies security mechanisms built
//! into that concrete stack, if any), a [`OscoreEdhocHandler`] is
//! [created][OscoreEdhocHandler::new] from the application, and passed into the stack.
//!
//! The arguments passed to the [`OscoreEdhocHandler`] at construction guide its behavior.
//!
//! # Logging
//!
//! Extensive logging is available in this crate through [`defmt_or_log`], depending on features
//! enabled.
//!
//! Errors from CoAP are currently logged through its [`Debug2Format`](defmt_or_log::Debug2Format)
//! facility, representing a compromise between development and runtime complexity. Should
//! benchmarks show this to be a significant factor in code size in applications that need error
//! handling, more fine grained control can be implemented (eg. offering an option to make
//! Debug2Format merely print the type name or even make it empty).
//!
//! This crate mainly logs on the trace, debug and error level; the latter provides details when an
//! error is sent over the network and the details are not visible to the peer.
//!
//! See the book for [how defmt is configured in
//! Ariel OS](https://ariel-os.github.io/ariel-os/dev/docs/book/tooling/defmt.html); outside of
//! that, regular [`defmt_or_log`] practica applies.
//!
//! **Warning**: At the Debug level, this module may show cryptographic key material. This will be
//! revised once all components have been interop-tested.
//!
//! # Caveats
//!
//! Currently, this has hidden dependencies on a particular implementation of the [`coap-message`]
//! provided (it needs to be a [`coap_message_implementations::inmemory_write::Message`]) by the
//! stack. There are plans for removing this limitation by integrating deeper with libOSCORE.
#![doc = document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#)]
#![no_std]
#![cfg_attr(feature = "_nightly_docs", feature(doc_auto_cfg))]
#![deny(missing_docs)]
#![deny(clippy::pedantic)]
#![allow(clippy::too_many_lines)]

mod iana;

mod helpers;
mod log_helpers;

pub mod time;

pub mod ace;
mod generalclaims;
pub mod scope;
pub use generalclaims::GeneralClaims;
pub mod seccfg;

// Might warrant a standalone crate at some point
//
// This is pub only to make the doctests run (but the crate's pub-ness needs a major overhaul
// anyway)
#[doc(hidden)]
pub mod oluru;
mod seccontext;
pub use seccontext::*;

mod error;
pub use error::CredentialError;
