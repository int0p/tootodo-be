use derive_more::From;
use crate::{db,auth,error::ErrorResponse};
use super::memo;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
	// -- Modules
	#[from]
	Auth(auth::error::Error),
    #[from]
    DB(db::error::Error),
   #[from]
   Memo(memo::error::Error),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate