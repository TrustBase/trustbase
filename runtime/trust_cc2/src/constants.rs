//! A set of constant values used in TrustBase runtime.

/// An index to a block.
/// 32-bits will allow for 136 years of blocks assuming 1 block per second.
pub type BlockNumber = u32;

/// An instant or duration in time.
pub type Moment = u64;

/// Time and blocks.
pub mod time {
	// --- substrate ---
	use sp_staking::SessionIndex;

	#[cfg(feature = "dev")]
	pub const MILLISECS_PER_BLOCK: Moment = 3000;
	#[cfg(not(feature = "dev"))]
	pub const MILLISECS_PER_BLOCK: Moment = 6000;

	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

	#[cfg(feature = "dev")]
	pub const BLOCKS_PER_SESSION: BlockNumber = MINUTES / 2;
	#[cfg(not(feature = "dev"))]
	pub const BLOCKS_PER_SESSION: BlockNumber = 4 * HOURS;

	#[cfg(feature = "dev")]
	pub const SESSIONS_PER_ERA: SessionIndex = 3;
	#[cfg(not(feature = "dev"))]
	pub const SESSIONS_PER_ERA: SessionIndex = 6;

	// These time units are defined in number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = 60 * MINUTES;
	pub const DAYS: BlockNumber = 24 * HOURS;

	// 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
	pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);
}