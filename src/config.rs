
// use serde_derive:: { Serialize, Deserialize };
// use serde::ser:: { Serialize, Serializer };
use serde_hjson :: { Value, from_str   };
use crate       :: { Merge, EkkeResult };
use std         :: { convert::TryFrom, fs::File, io::BufReader, io::Read, path::Path };
use failure     :: { Error };


#[ derive( Debug, Clone, PartialEq ) ]
//
pub struct Config
{
	data: Value,
}


impl Config
{
	/// See: https://docs.rs/serde-hjson/0.9.0/serde_hjson/value/enum.Value.html
	///
	pub fn find<'a>(&'a self, key: &str) -> Option<&'a Value>
	{
		self.data.find( key )
	}


	/// See: https://docs.rs/serde-hjson/0.9.0/serde_hjson/value/enum.Value.html
	///
	pub fn find_path<'a>(&'a self, keys: &[&str]) -> Option<&'a Value>
	{
		self.data.find_path( keys )
	}


	/// See: https://docs.rs/serde-hjson/0.9.0/serde_hjson/value/enum.Value.html
	///
	pub fn pointer<'a>(&'a self, pointer: &str) -> Option<&'a Value>
	{
		self.data.pointer( pointer )
	}


	/// See: https://docs.rs/serde-hjson/0.9.0/serde_hjson/value/enum.Value.html
	///
	pub fn search<'a>(&'a self, key: &str) -> Option<&'a Value>
	{
		self.data.search( key )
	}
}


impl Merge for Config
{
	fn merge( &mut self, other: Self ) -> EkkeResult<()>
	{
		self.data.merge( other.data )
	}
}


use serde::ser;

impl ser::Serialize for Config
{
	fn serialize< S >( &self, serializer: &mut S ) -> Result< (), S::Error > where S: ser::Serializer
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
"{
  arr:
  [
    some
    strings
    in
    array
  ]
}";

		let cfg = Config::try_from( a_s ).unwrap();

		assert_eq!( cfg.pointer( "/arr/0" ).unwrap().as_str(), Some( "some" ) );

		assert_eq!( a_s, serde_hjson::ser::to_string( &cfg ).unwrap() )
	}

}
