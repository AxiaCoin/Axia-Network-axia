// Copyright 2019-2021 Axia Technologies (UK) Ltd.
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

//! Mocking utilities for testing.

use crate::traits::Registrar;
use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	weights::Weight,
};
use axia_scale_codec::{Decode, Encode};
use primitives::v1::{HeadData, Id as AllyId, ValidationCode};
use sp_runtime::{traits::SaturatedConversion, Permill};
use std::{cell::RefCell, collections::HashMap};

thread_local! {
	static OPERATIONS: RefCell<Vec<(AllyId, u32, bool)>> = RefCell::new(Vec::new());
	static ALLYCHAINS: RefCell<Vec<AllyId>> = RefCell::new(Vec::new());
	static ALLYTHREADS: RefCell<Vec<AllyId>> = RefCell::new(Vec::new());
	static LOCKS: RefCell<HashMap<AllyId, bool>> = RefCell::new(HashMap::new());
	static MANAGERS: RefCell<HashMap<AllyId, Vec<u8>>> = RefCell::new(HashMap::new());
}

pub struct TestRegistrar<T>(sp_std::marker::PhantomData<T>);

impl<T: frame_system::Config> Registrar for TestRegistrar<T> {
	type AccountId = T::AccountId;

	fn manager_of(id: AllyId) -> Option<Self::AccountId> {
		MANAGERS.with(|x| x.borrow().get(&id).and_then(|v| T::AccountId::decode(&mut &v[..]).ok()))
	}

	fn allychains() -> Vec<AllyId> {
		ALLYCHAINS.with(|x| x.borrow().clone())
	}

	fn is_allythread(id: AllyId) -> bool {
		ALLYTHREADS.with(|x| x.borrow().binary_search(&id).is_ok())
	}

	fn apply_lock(id: AllyId) {
		LOCKS.with(|x| x.borrow_mut().insert(id, true));
	}

	fn remove_lock(id: AllyId) {
		LOCKS.with(|x| x.borrow_mut().insert(id, false));
	}

	fn register(
		manager: Self::AccountId,
		id: AllyId,
		_genesis_head: HeadData,
		_validation_code: ValidationCode,
	) -> DispatchResult {
		// Should not be allychain.
		ALLYCHAINS.with(|x| {
			let allychains = x.borrow_mut();
			match allychains.binary_search(&id) {
				Ok(_) => Err(DispatchError::Other("Already Allychain")),
				Err(_) => Ok(()),
			}
		})?;
		// Should not be allythread, then make it.
		ALLYTHREADS.with(|x| {
			let mut allythreads = x.borrow_mut();
			match allythreads.binary_search(&id) {
				Ok(_) => Err(DispatchError::Other("Already Allythread")),
				Err(i) => {
					allythreads.insert(i, id);
					Ok(())
				},
			}
		})?;
		MANAGERS.with(|x| x.borrow_mut().insert(id, manager.encode()));
		Ok(())
	}

	fn deregister(id: AllyId) -> DispatchResult {
		// Should not be allychain.
		ALLYCHAINS.with(|x| {
			let allychains = x.borrow_mut();
			match allychains.binary_search(&id) {
				Ok(_) => Err(DispatchError::Other("cannot deregister allychain")),
				Err(_) => Ok(()),
			}
		})?;
		// Remove from allythread.
		ALLYTHREADS.with(|x| {
			let mut allythreads = x.borrow_mut();
			match allythreads.binary_search(&id) {
				Ok(i) => {
					allythreads.remove(i);
					Ok(())
				},
				Err(_) => Err(DispatchError::Other("not allythread, so cannot `deregister`")),
			}
		})?;
		MANAGERS.with(|x| x.borrow_mut().remove(&id));
		Ok(())
	}

	fn make_allychain(id: AllyId) -> DispatchResult {
		ALLYTHREADS.with(|x| {
			let mut allythreads = x.borrow_mut();
			match allythreads.binary_search(&id) {
				Ok(i) => {
					allythreads.remove(i);
					Ok(())
				},
				Err(_) => Err(DispatchError::Other("not allythread, so cannot `make_allychain`")),
			}
		})?;
		ALLYCHAINS.with(|x| {
			let mut allychains = x.borrow_mut();
			match allychains.binary_search(&id) {
				Ok(_) => Err(DispatchError::Other("already allychain, so cannot `make_allychain`")),
				Err(i) => {
					allychains.insert(i, id);
					Ok(())
				},
			}
		})?;
		OPERATIONS.with(|x| {
			x.borrow_mut().push((
				id,
				frame_system::Pallet::<T>::block_number().saturated_into(),
				true,
			))
		});
		Ok(())
	}
	fn make_allythread(id: AllyId) -> DispatchResult {
		ALLYCHAINS.with(|x| {
			let mut allychains = x.borrow_mut();
			match allychains.binary_search(&id) {
				Ok(i) => {
					allychains.remove(i);
					Ok(())
				},
				Err(_) => Err(DispatchError::Other("not allychain, so cannot `make_allythread`")),
			}
		})?;
		ALLYTHREADS.with(|x| {
			let mut allythreads = x.borrow_mut();
			match allythreads.binary_search(&id) {
				Ok(_) =>
					Err(DispatchError::Other("already allythread, so cannot `make_allythread`")),
				Err(i) => {
					allythreads.insert(i, id);
					Ok(())
				},
			}
		})?;
		OPERATIONS.with(|x| {
			x.borrow_mut().push((
				id,
				frame_system::Pallet::<T>::block_number().saturated_into(),
				false,
			))
		});
		Ok(())
	}

	#[cfg(test)]
	fn worst_head_data() -> HeadData {
		vec![0u8; 1000].into()
	}

	#[cfg(test)]
	fn worst_validation_code() -> ValidationCode {
		let validation_code = vec![0u8; 1000];
		validation_code.into()
	}

	#[cfg(test)]
	fn execute_pending_transitions() {}
}

impl<T: frame_system::Config> TestRegistrar<T> {
	pub fn operations() -> Vec<(AllyId, T::BlockNumber, bool)> {
		OPERATIONS
			.with(|x| x.borrow().iter().map(|(p, b, c)| (*p, (*b).into(), *c)).collect::<Vec<_>>())
	}

	#[allow(dead_code)]
	pub fn allychains() -> Vec<AllyId> {
		ALLYCHAINS.with(|x| x.borrow().clone())
	}

	#[allow(dead_code)]
	pub fn allythreads() -> Vec<AllyId> {
		ALLYTHREADS.with(|x| x.borrow().clone())
	}

	#[allow(dead_code)]
	pub fn clear_storage() {
		OPERATIONS.with(|x| x.borrow_mut().clear());
		ALLYCHAINS.with(|x| x.borrow_mut().clear());
		ALLYTHREADS.with(|x| x.borrow_mut().clear());
		MANAGERS.with(|x| x.borrow_mut().clear());
	}
}

/// A very dumb implementation of `EstimateNextSessionRotation`. At the moment of writing, this
/// is more to satisfy type requirements rather than to test anything.
pub struct TestNextSessionRotation;

impl frame_support::traits::EstimateNextSessionRotation<u32> for TestNextSessionRotation {
	fn average_session_length() -> u32 {
		10
	}

	fn estimate_current_session_progress(_now: u32) -> (Option<Permill>, Weight) {
		(None, 0)
	}

	fn estimate_next_session_rotation(_now: u32) -> (Option<u32>, Weight) {
		(None, 0)
	}
}
