// Copyright 2017-2021 Axia Technologies (UK) Ltd.
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

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error(transparent)]
	AxiaService(#[from] service::Error),

	#[error(transparent)]
	AxlibCli(#[from] sc_cli::Error),

	#[error(transparent)]
	AxlibService(#[from] sc_service::Error),

	#[error(transparent)]
	AxlibTracing(#[from] sc_tracing::logging::Error),

	#[error(transparent)]
	PerfCheck(#[from] axia_performance_test::PerfCheckError),

	#[error("Other: {0}")]
	Other(String),
}
