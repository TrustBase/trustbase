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

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Structs to easily compose inspect sub-command for CLI.

use std::fmt::Debug;
use sc_cli::{ImportParams, SharedParams};
use structopt::StructOpt;

/// The `inspect` command used to print decoded chain data.
#[derive(Debug, StructOpt)]
pub struct InspectCmd {
	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub command: InspectSubCmd,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub shared_params: SharedParams,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub import_params: ImportParams,
}

/// A possible inspect sub-commands.
#[derive(Debug, StructOpt)]
pub enum InspectSubCmd {
	/// Decode block with native version of runtime and print out the details.
	Block {
		/// Address of the block to print out.
		///
		/// Can be either a block hash (no 0x prefix) or a number to retrieve existing block,
		/// or a 0x-prefixed bytes hex string, representing SCALE encoding of
		/// a block.
		#[structopt(value_name = "HASH or NUMBER or BYTES")]
		input: String,
	},
	/// Decode extrinsic with native version of runtime and print out the details.
	Extrinsic {
		/// Address of an extrinsic to print out.
		///
		/// Can be either a block hash (no 0x prefix) or number and the index, in the form
		/// of `{block}:{index}` or a 0x-prefixed bytes hex string,
		/// representing SCALE encoding of an extrinsic.
		#[structopt(value_name = "BLOCK:INDEX or BYTES")]
		input: String,
	},
}
