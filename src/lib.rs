//! An opinionated configuration library.
//!
//! The basic scenario for using this library is as follows:
//!
//! - ship a defaults configuration file with the program.
//! - allow users to set configuration in files in /etc or $HOME
//! - receive runtime configuration from env variables or command line options.
//!
//! The configuration object provided by ekke_config allows you to merge all of the above sources and
//! deserialize into your custom Settings object which defines the layout of your configuration. This
//! allows convenient access to your configuration, knowing that once the parsing has succeeded,
//! there is no more options and results to process, just access your configuration as properties on your
//! settings struct, knowing that they are guaranteed to exist.
//!
//! Limitations:
//! - currently only works with serde_yaml
//! - no configuration profiles (debug, production, staging...) -> use different config files for these for now.
//!
//! See examples/basic.rs for an introductory example.
//!

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


mod import
{
	#[ allow( unused_imports ) ]
	//
	pub( crate ) use
	{
		failure     :: { Error, Fail, ResultExt                                                                     } ,
		std         :: { convert::TryFrom, fs::File, io::BufReader, io::Read, path::Path, path::PathBuf, fmt::Debug } ,
		serde       :: { ser::Serialize, Deserialize,  de::DeserializeOwned                                         } ,
		serde_yaml  :: { Value, Mapping, from_str                                                                   } ,
		shellexpand :: { tilde                                                                                      } ,

		ekke_merge  :: { Merge, MergeResult                                                                         } ,
	};
}
