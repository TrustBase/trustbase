// Copyright 2019 TrustBase Network
// This file is part of TrustBase library.
//
// The TrustBase library is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// The TrustBase library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with the TrustBase library. If not, see <http://www.gnu.org/licenses/>.
use wasm_builder_runner::WasmBuilder;

fn main() {
	WasmBuilder::new()
		.with_current_project()
		.with_wasm_builder_from_crates("2.0.0")
		.export_heap_base()
		.import_memory()
		.build()
}