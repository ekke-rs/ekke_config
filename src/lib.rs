#![ feature( try_from ) ]


mod config;
mod error;
mod merge;

pub use config::
{
	Config ,
};

pub use merge::
{
	Merge ,
};

pub use error::
{
	EkkeResult,
	EkkeCfgError
};
