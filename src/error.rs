use failure     :: { Error, Fail  } ;
use serde_hjson :: { Value } ;


/// Custom result type, Allows to omit error type since it's always
/// [`failure::Error`](https://docs.rs/failure/0.1.5/failure/struct.Error.html).
///
pub type EkkeResult<T> = Result< T, Error >;


/// The specific errors ekke_io can return.
///
#[ derive( Debug, Fail ) ]
//
pub enum EkkeCfgError
{
	#[ fail( display = "Cannot merge two config values of different types: {:#?} and {:#?}", _0, _1 ) ]
	//
	MergeWrongType( Value, Value ),

	#[ fail( display = "Cannot unset configuration value by setting it to Null" ) ]
	//
	UnsetConfig,
}
