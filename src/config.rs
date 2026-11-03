


#[derive(Debug, PartialEq, Eq)]
pub enum FailureMode
{
	Ignore,
	Panic,
	Break,
	BreakAndPanic,
	Interactive,
}

pub const ENABLE_ASSERTS : bool = cfg!(feature="enable_asserts");

pub const FAILURE_MODE : FailureMode =
	if 		cfg!(feature="failure_mode_panic") { FailureMode::Panic }
	else if cfg!(feature="failure_mode_break") { FailureMode::Break }
	else if cfg!(feature="failure_mode_break_and_panic") { FailureMode::BreakAndPanic }
	else if cfg!(feature="failure_mode_interactive") { FailureMode::Interactive }
	else { FailureMode::Ignore };