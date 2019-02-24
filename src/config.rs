use failure     :: { Error                                                           } ;
use std         :: { convert::TryFrom, fs::File, io::BufReader, io::Read, path::Path, path::PathBuf } ;
use serde       :: { ser, /*de*/                                                         } ;
use serde_yaml  :: { Value, from_str                                                 } ;
use crate       :: { EkkeResult                                               } ;
use ekke_merge :: { Merge };


/// A configuration object that can be created from multiple layers of yaml input. Later
/// input that is added by merge will merge into the earlier data and override options
/// that are already set. Objects will be merged recursively.
/// Arrays contents will be replaced.
///
#[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
//
pub struct Config
{
	data: Value,
}


impl Config
{
	/// See: https://docs.rs/serde-hjson/0.9.0/serde_hjson/value/enum.Value.html
	///
	pub fn get<'a>( &'a self, pointer: &str ) -> Option< &'a Value >
	{
		fn parse_index( s: &str ) -> Option< usize >
		{
			if s.starts_with( '+' ) || ( s.starts_with( '0' ) && s.len() != 1 )
			{
				return None;
			}

			s.parse().ok()
		}


		if  pointer == ""              { return Some( &self.data ) }
		if !pointer.starts_with( '/' ) { return None               }


		let mut target = &self.data;

		for escaped_token in pointer.split( '/' ).skip(1)
		{
			let token = escaped_token.replace( "~0", "~" ).replace( "~1", "/" );


			let target_opt = match *target
			{
				Value::Mapping ( ref map  ) => map.get    ( &token.into() )                             ,
				Value::Sequence( ref list ) => parse_index( &token        ).and_then( |x| list.get(x) ) ,
				_                           => return None                                              ,
			};

			match target_opt
			{
				Some( t ) => target = t ,
				None      => return None,
			}
		}


		Some( target )
	}
}


impl Merge for Config
{
	fn merge( &mut self, other: Self ) -> EkkeResult<()>
	{
		self.data.merge( other.data )
	}
}



impl ser::Serialize for Config
{
	fn serialize< S >( &self, serializer: S ) -> Result< S::Ok, S::Error > where S: ser::Serializer
	{
		self.data.serialize( serializer )
	}
}



/// Convert from hjson string
///
impl TryFrom< &str > for Config
{
	type Error = Error;

	fn try_from( input: &str ) -> Result< Self, Self::Error >
	{
		let data: Value = from_str( input )?;

		Ok( Config { data } )
	}
}


/// Convert from a file containing an hjson string
///
impl TryFrom< &File > for Config
{
	type Error = Error;

	fn try_from( file: &File ) -> Result< Self, Self::Error >
	{
		let mut buf_reader = BufReader::new(file);
		let mut contents = String::new();
		buf_reader.read_to_string(&mut contents)?;

		let data: Value = from_str( &contents )?;

		Ok( Config { data } )
	}
}



/// Convert from a file containing an hjson string
///
impl TryFrom< &Path > for Config
{
	type Error = Error;

	fn try_from( path: &Path ) -> Result< Self, Self::Error >
	{
		let file = File::open( path )?;
		Config::try_from( &file )
	}
}



/// Convert from a file containing an hjson string
///
impl TryFrom< &PathBuf > for Config
{
	type Error = Error;

	fn try_from( path: &PathBuf ) -> Result< Self, Self::Error >
	{
		let file = File::open( path )?;
		Config::try_from( &file )
	}
}



#[ cfg( test ) ]
//
mod tests
{
	use super::*;

	// Test try_from( &str )
	// Test pointer()
	// Test serialize
	//
	#[test]
	//
	fn pointer_basic()
	{
		let a_s =
"---
arr:
  - some
  - strings
  - in
  - array";

		let cfg = Config::try_from( a_s ).unwrap();

		assert_eq!( cfg.get( "/arr/0" ).unwrap().as_str(), Some( "some" ) );

		assert_eq!( a_s, serde_yaml::to_string( &cfg ).unwrap() )
	}



	// Test try_from( &str )
	// Test pointer()
	// Test serialize
	//
	#[test]
	//
	fn pointer_nested_object()
	{
		let a_s =
"---
arr:
  - some
  - bli: bla
    blo:
      - 44
      - 66
      - 77
  - in
  - array";

		let cfg = Config::try_from( a_s ).unwrap();

		assert_eq!( cfg.get( "/arr/1/blo/2" ).unwrap(), 77 );

		assert_eq!( a_s, serde_yaml::to_string( &cfg ).unwrap() )
	}

}
