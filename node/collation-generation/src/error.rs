// Copyright 2020 Axia Technologies (UK) Ltd.
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

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	Subsystem(#[from] axia_node_subsystem::SubsystemError),
	#[error(transparent)]
	OneshotRecv(#[from] futures::channel::oneshot::Canceled),
	#[error(transparent)]
	Runtime(#[from] axia_node_subsystem::errors::RuntimeApiError),
	#[error(transparent)]
	Util(#[from] axia_node_subsystem_util::Error),
	#[error(transparent)]
	Erasure(#[from] axia_erasure_coding::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
