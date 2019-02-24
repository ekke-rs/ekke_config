#![ feature( try_from ) ]


mod config;
mod error;

pub use config::
{
	Config ,
};


pub use error::
{
	EkkeResult,
	EkkeCfgError
};
