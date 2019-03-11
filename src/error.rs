use crate :: { import::* };


/// Custom result type, Allows to omit error type since it's always
/// [`failure::Error`](https://docs.rs/failure/0.1.5/failure/struct.Error.html).
///
pub type EkkeResult<T> = Result< T, Error >;


/// The specific errors ekke_config can return.
///
#[ derive( Debug, Fail ) ]
//
pub enum EkkeCfgError
{
	#[ fail( display = "Cannot unset default configuration value by setting it to Null" ) ]
	//
	UnsetConfig,

	#[ fail( display = "Failed to parse Configuration" ) ]
	//
	ConfigParse,
}
