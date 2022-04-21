// Copyright 2017-2020 Axia Technologies (UK) Ltd.
// This file is part of Axia.

// Axia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Axia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Axia.  If not, see <http://www.gnu.org/licenses/>.

//! Subsystem accumulation.
//!
//! Node-side types and generated overseer.

#![deny(missing_docs)]
#![deny(unused_crate_dependencies)]

pub use jaeger::*;
pub use axia_node_jaeger as jaeger;

pub use axia_overseer::{
	self as overseer, ActiveLeavesUpdate, OverseerConnector, OverseerSignal,
};

pub use axia_node_subsystem_types::{
	errors::{self, *},
	ActivatedLeaf, LeafStatus,
};

/// Re-export of all messages type, including the wrapper type.
pub mod messages {
	pub use super::overseer::AllMessages;
	pub use axia_node_subsystem_types::messages::*;
}

/// A `Result` type that wraps [`SubsystemError`].
///
/// [`SubsystemError`]: struct.SubsystemError.html
pub type SubsystemResult<T> = Result<T, SubsystemError>;

// Simplify usage without having to do large scale modifications of all
// subsystems at once.

/// Specialized message type originating from the overseer.
pub type FromOverseer<M> = axia_overseer::gen::FromOverseer<M, OverseerSignal>;

/// Specialized subsystem instance type of subsystems consuming a particular message type.
pub type SubsystemInstance<Message> =
	axia_overseer::gen::SubsystemInstance<Message, OverseerSignal>;

/// Sender trait for the `AllMessages` wrapper.
pub trait SubsystemSender: axia_overseer::gen::SubsystemSender<messages::AllMessages> {}

impl<T> SubsystemSender for T where T: axia_overseer::gen::SubsystemSender<messages::AllMessages>
{}

/// Spawned subsystem.
pub type SpawnedSubsystem = axia_overseer::gen::SpawnedSubsystem<SubsystemError>;

/// Convenience trait specialization.
pub trait SubsystemContext:
	axia_overseer::gen::SubsystemContext<
	Signal = OverseerSignal,
	AllMessages = messages::AllMessages,
	Error = SubsystemError,
>
{
	/// The message type the subsystem consumes.
	type Message: std::fmt::Debug + Send + 'static;
	/// Sender type to communicate with other subsystems.
	type Sender: SubsystemSender + Send + Clone + 'static;
}

impl<T> SubsystemContext for T
where
	T: axia_overseer::gen::SubsystemContext<
		Signal = OverseerSignal,
		AllMessages = messages::AllMessages,
		Error = SubsystemError,
	>,
{
	type Message = <Self as axia_overseer::gen::SubsystemContext>::Message;
	type Sender = <Self as axia_overseer::gen::SubsystemContext>::Sender;
}
