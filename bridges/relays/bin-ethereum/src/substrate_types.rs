// Copyright 2020-2021 AXIA Technologies (UK) Ltd.
// This file is part of AXIA Bridges Common.

// AXIA Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// AXIA Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with AXIA Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Converting between Ethereum headers and bridge module types.

use bp_eth_poa::{
	AuraHeader as AxlibEthereumHeader, LogEntry as AxlibEthereumLogEntry, Receipt as AxlibEthereumReceipt,
	TransactionOutcome as AxlibEthereumTransactionOutcome,
};
use relay_ethereum_client::types::{
	Header as EthereumHeader, Receipt as EthereumReceipt, HEADER_ID_PROOF as ETHEREUM_HEADER_ID_PROOF,
};

/// Convert Ethereum header into Ethereum header for Axlib.
pub fn into_axlib_ethereum_header(header: &EthereumHeader) -> AxlibEthereumHeader {
	AxlibEthereumHeader {
		parent_hash: header.parent_hash,
		timestamp: header.timestamp.as_u64(),
		number: header.number.expect(ETHEREUM_HEADER_ID_PROOF).as_u64(),
		author: header.author,
		transactions_root: header.transactions_root,
		uncles_hash: header.uncles_hash,
		extra_data: header.extra_data.0.clone(),
		state_root: header.state_root,
		receipts_root: header.receipts_root,
		log_bloom: header.logs_bloom.unwrap_or_default().data().into(),
		gas_used: header.gas_used,
		gas_limit: header.gas_limit,
		difficulty: header.difficulty,
		seal: header.seal_fields.iter().map(|s| s.0.clone()).collect(),
	}
}

/// Convert Ethereum transactions receipts into Ethereum transactions receipts for Axlib.
pub fn into_axlib_ethereum_receipts(
	receipts: &Option<Vec<EthereumReceipt>>,
) -> Option<Vec<AxlibEthereumReceipt>> {
	receipts
		.as_ref()
		.map(|receipts| receipts.iter().map(into_axlib_ethereum_receipt).collect())
}

/// Convert Ethereum transactions receipt into Ethereum transactions receipt for Axlib.
pub fn into_axlib_ethereum_receipt(receipt: &EthereumReceipt) -> AxlibEthereumReceipt {
	AxlibEthereumReceipt {
		gas_used: receipt.cumulative_gas_used,
		log_bloom: receipt.logs_bloom.data().into(),
		logs: receipt
			.logs
			.iter()
			.map(|log_entry| AxlibEthereumLogEntry {
				address: log_entry.address,
				topics: log_entry.topics.clone(),
				data: log_entry.data.0.clone(),
			})
			.collect(),
		outcome: match (receipt.status, receipt.root) {
			(Some(status), None) => AxlibEthereumTransactionOutcome::StatusCode(status.as_u64() as u8),
			(None, Some(root)) => AxlibEthereumTransactionOutcome::StateRoot(root),
			_ => AxlibEthereumTransactionOutcome::Unknown,
		},
	}
}
