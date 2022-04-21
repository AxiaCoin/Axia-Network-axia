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

//! Prometheus metrics related to the overseer and its channels.

use super::*;
pub use axia_node_metrics::metrics::{self, prometheus, Metrics as MetricsTrait};

use axia_util_mem::MemoryAllocationSnapshot;

/// Overseer Prometheus metrics.
#[derive(Clone)]
struct MetricsInner {
	activated_heads_total: prometheus::Counter<prometheus::U64>,
	deactivated_heads_total: prometheus::Counter<prometheus::U64>,
	messages_relayed_total: prometheus::Counter<prometheus::U64>,
	to_subsystem_bounded_sent: prometheus::GaugeVec<prometheus::U64>,
	to_subsystem_bounded_received: prometheus::GaugeVec<prometheus::U64>,
	to_subsystem_unbounded_sent: prometheus::GaugeVec<prometheus::U64>,
	to_subsystem_unbounded_received: prometheus::GaugeVec<prometheus::U64>,
	signals_sent: prometheus::GaugeVec<prometheus::U64>,
	signals_received: prometheus::GaugeVec<prometheus::U64>,

	memory_stats_resident: prometheus::Gauge<prometheus::U64>,
	memory_stats_allocated: prometheus::Gauge<prometheus::U64>,
}

/// A shareable metrics type for usage with the overseer.
#[derive(Default, Clone)]
pub struct Metrics(Option<MetricsInner>);

impl Metrics {
	pub(crate) fn on_head_activated(&self) {
		if let Some(metrics) = &self.0 {
			metrics.activated_heads_total.inc();
		}
	}

	pub(crate) fn on_head_deactivated(&self) {
		if let Some(metrics) = &self.0 {
			metrics.deactivated_heads_total.inc();
		}
	}

	pub(crate) fn on_message_relayed(&self) {
		if let Some(metrics) = &self.0 {
			metrics.messages_relayed_total.inc();
		}
	}

	pub(crate) fn memory_stats_snapshot(&self, memory_stats: MemoryAllocationSnapshot) {
		if let Some(metrics) = &self.0 {
			metrics.memory_stats_allocated.set(memory_stats.allocated);
			metrics.memory_stats_resident.set(memory_stats.resident);
		}
	}

	pub(crate) fn channel_fill_level_snapshot(
		&self,
		collection: impl IntoIterator<Item = (&'static str, SubsystemMeterReadouts)>,
	) {
		if let Some(metrics) = &self.0 {
			collection
				.into_iter()
				.for_each(|(name, readouts): (_, SubsystemMeterReadouts)| {
					metrics
						.to_subsystem_bounded_sent
						.with_label_values(&[name])
						.set(readouts.bounded.sent as u64);

					metrics
						.to_subsystem_bounded_received
						.with_label_values(&[name])
						.set(readouts.bounded.received as u64);

					metrics
						.to_subsystem_unbounded_sent
						.with_label_values(&[name])
						.set(readouts.unbounded.sent as u64);

					metrics
						.to_subsystem_unbounded_received
						.with_label_values(&[name])
						.set(readouts.unbounded.received as u64);

					metrics
						.signals_sent
						.with_label_values(&[name])
						.set(readouts.signals.sent as u64);

					metrics
						.signals_received
						.with_label_values(&[name])
						.set(readouts.signals.received as u64);
				});
		}
	}
}

impl MetricsTrait for Metrics {
	fn try_register(registry: &prometheus::Registry) -> Result<Self, prometheus::PrometheusError> {
		let metrics = MetricsInner {
			activated_heads_total: prometheus::register(
				prometheus::Counter::new(
					"axia_allychain_activated_heads_total",
					"Number of activated heads.",
				)?,
				registry,
			)?,
			deactivated_heads_total: prometheus::register(
				prometheus::Counter::new(
					"axia_allychain_deactivated_heads_total",
					"Number of deactivated heads.",
				)?,
				registry,
			)?,
			messages_relayed_total: prometheus::register(
				prometheus::Counter::new(
					"axia_allychain_messages_relayed_total",
					"Number of messages relayed by Overseer.",
				)?,
				registry,
			)?,
			to_subsystem_bounded_sent: prometheus::register(
				prometheus::GaugeVec::<prometheus::U64>::new(
					prometheus::Opts::new(
						"axia_allychain_subsystem_bounded_sent",
						"Number of elements sent to subsystems' bounded queues",
					),
					&["subsystem_name"],
				)?,
				registry,
			)?,
			to_subsystem_bounded_received: prometheus::register(
				prometheus::GaugeVec::<prometheus::U64>::new(
					prometheus::Opts::new(
						"axia_allychain_subsystem_bounded_received",
						"Number of elements received by subsystems' bounded queues",
					),
					&["subsystem_name"],
				)?,
				registry,
			)?,
			to_subsystem_unbounded_sent: prometheus::register(
				prometheus::GaugeVec::<prometheus::U64>::new(
					prometheus::Opts::new(
						"axia_allychain_subsystem_unbounded_sent",
						"Number of elements sent to subsystems' unbounded queues",
					),
					&["subsystem_name"],
				)?,
				registry,
			)?,
			to_subsystem_unbounded_received: prometheus::register(
				prometheus::GaugeVec::<prometheus::U64>::new(
					prometheus::Opts::new(
						"axia_allychain_subsystem_unbounded_received",
						"Number of elements received by subsystems' unbounded queues",
					),
					&["subsystem_name"],
				)?,
				registry,
			)?,
			signals_sent: prometheus::register(
				prometheus::GaugeVec::<prometheus::U64>::new(
					prometheus::Opts::new(
						"axia_allychain_overseer_signals_sent",
						"Number of signals sent by overseer to subsystems",
					),
					&["subsystem_name"],
				)?,
				registry,
			)?,
			signals_received: prometheus::register(
				prometheus::GaugeVec::<prometheus::U64>::new(
					prometheus::Opts::new(
						"axia_allychain_overseer_signals_received",
						"Number of signals received by subsystems from overseer",
					),
					&["subsystem_name"],
				)?,
				registry,
			)?,

			memory_stats_allocated: prometheus::register(
				prometheus::Gauge::<prometheus::U64>::new(
					"axia_memory_allocated",
					"Total bytes allocated by the node",
				)?,
				registry,
			)?,
			memory_stats_resident: prometheus::register(
				prometheus::Gauge::<prometheus::U64>::new(
					"axia_memory_resident",
					"Bytes allocated by the node, and held in RAM",
				)?,
				registry,
			)?,
		};
		Ok(Metrics(Some(metrics)))
	}
}

impl fmt::Debug for Metrics {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("Metrics {{...}}")
	}
}
