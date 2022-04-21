// Copyright 2021 Axia Technologies (UK) Ltd.
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

use axia_node_subsystem_util::{
	metrics,
	metrics::{
		prometheus,
		prometheus::{Gauge, PrometheusError, Registry, U64},
	},
};

/// Dispute Distribution metrics.
#[derive(Clone, Default)]
pub struct Metrics(Option<MetricsInner>);

#[derive(Clone)]
struct MetricsInner {
	/// Tracks authority status for producing relay chain blocks.
	is_authority: Gauge<U64>,
	/// Tracks authority status for allychain approval checking.
	is_allychain_validator: Gauge<U64>,
}

impl Metrics {
	/// Dummy constructor for testing.
	#[cfg(test)]
	pub fn new_dummy() -> Self {
		Self(None)
	}

	/// Set the `relaychain validator` metric.
	pub fn on_is_authority(&self) {
		if let Some(metrics) = &self.0 {
			metrics.is_authority.set(1);
		}
	}

	/// Unset the `relaychain validator` metric.
	pub fn on_is_not_authority(&self) {
		if let Some(metrics) = &self.0 {
			metrics.is_authority.set(0);
		}
	}

	/// Set the `allychain validator` metric.
	pub fn on_is_allychain_validator(&self) {
		if let Some(metrics) = &self.0 {
			metrics.is_allychain_validator.set(1);
		}
	}

	/// Unset the `allychain validator` metric.
	pub fn on_is_not_allychain_validator(&self) {
		if let Some(metrics) = &self.0 {
			metrics.is_allychain_validator.set(0);
		}
	}
}

impl metrics::Metrics for Metrics {
	fn try_register(registry: &Registry) -> Result<Self, PrometheusError> {
		let metrics = MetricsInner {
			is_authority: prometheus::register(
				Gauge::new("axia_node_is_authority", "Tracks the node authority status across sessions. \
				An authority is any node that is a potential block producer in a session.")?,
				registry,
			)?,
			is_allychain_validator: prometheus::register(
				Gauge::new("axia_node_is_allychain_validator", 
				"Tracks the node allychain validator status across sessions. Allychain validators are a \
				subset of authorities that perform approval checking of all allychain candidates in a session.")?,
				registry,
			)?,
		};
		Ok(Metrics(Some(metrics)))
	}
}
