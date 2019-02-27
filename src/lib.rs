mod config;
mod error;
mod pointer;


pub use config::
{
	Config ,
};

pub use pointer::
{
	Pointer ,
};


pub use error::
{
	EkkeResult,
	EkkeCfgError
};
