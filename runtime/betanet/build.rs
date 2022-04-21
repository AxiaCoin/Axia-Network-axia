// Copyright 2020 Axia Technologies (UK) Ltd.
// This file is part of Axia.

// Axlib is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Axlib is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Axlib.  If not, see <http://www.gnu.org/licenses/>.

use axlib_wasm_builder::WasmBuilder;

fn main() {
	WasmBuilder::new()
		.with_current_project()
		.import_memory()
		.export_heap_base()
		.build()
}
