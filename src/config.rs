use failure     :: { Error                                                                          } ;
use std         :: { convert::TryFrom, fs::File, io::BufReader, io::Read, path::Path, path::PathBuf } ;
use serde       :: { ser::Serialize, Deserialize, /*ser::Serializer,*/ de::DeserializeOwned                          } ;
use serde_yaml  :: { Value, Mapping, from_str                                                                } ;
use crate       :: { EkkeCfgError                                                                     } ;
use ekke_merge  :: { Merge, MergeResult                                                                        } ;


/// A configuration object that can be created from multiple layers of yaml input. Later
/// input that is added by merge will merge into the earlier data and override options
/// that are already set. Objects will be merged recursively.
/// Arrays contents will be replaced.
///
#[ derive( Debug, Clone, PartialEq, Eq, Default, Deserialize ) ]
//
pub struct Config<T> where T: Merge + Clone + Serialize
{
	defaults : T               ,
	settings : T               ,
	meta     : Mapping         ,
	userset  : Option< Mapping > ,
	runtime  : Option< Mapping > ,
}


impl<T> Config<T> where T: Merge + Clone + DeserializeOwned + Serialize
{
	pub fn merge_runtime( &mut self, input: &str ) -> MergeResult<()>
	{
		let rt2: Value = from_str( input )?;

		let rt: Mapping = match rt2
		{
			Value::Mapping(x) => x,
			_                 => return Err( EkkeCfgError::ConfigParse.into() )
		};

		let data: &mut Mapping = match self.meta.get_mut( &Value::from( "default" ) ).unwrap()
		{
			Value::Mapping(x) => x,
			_                 => return Err( EkkeCfgError::ConfigParse.into() )
		};

		data.merge( rt.clone() ).unwrap();

		self.runtime = Some( rt );

		self.settings =  from_str( &serde_yaml::to_string( &data )? )? ;

		Ok(())
	}

	pub fn merge_userset( &mut self, _input: &str ) -> MergeResult<()>
	{

		Ok(())
	}


	pub fn get( &self ) -> &T
	{
		&self.settings
	}


	pub fn default( &self ) -> &T
	{
		&self.defaults
	}


	pub fn userset( &self ) -> Option< &Mapping >
	{
		match &self.userset
		{
			Some( value ) => Some( &value ),
			None          => None          ,
		}
	}


	pub fn runtime( &self ) -> Option< &Mapping >
	{
		match &self.runtime
		{
			Some( value ) => Some( &value ),
			None          => None          ,
		}
	}
}

/*
impl<T> Merge for Config<T> where T: Merge + Clone + DeserializeOwned + Serialize
{
	fn merge( &mut self, other: Self ) -> EkkeResult<()>
	{
		self.data.merge( other.data )
	}
}*/


/*
impl<T> Serialize for Config<T> where T: Merge + Clone + DeserializeOwned + Serialize
{
	fn serialize< S >( &self, serializer: S ) -> Result< S::Ok, S::Error > where S: Serializer
	{
		self.data.serialize( serializer )
	}
}*/




/// Convert from yaml string
///
impl<T> TryFrom< &str > for Config<T> where T: Merge + Clone + DeserializeOwned + Serialize
{
	type Error = Error;

	fn try_from( input: &str ) -> Result< Self, Self::Error >
	{
		let mut meta    : serde_yaml::Mapping = from_str( input )? ;
		let data: &mut Mapping;
		let mut userset: Option< Mapping > = None;

		let mut user_conf = None;

		// Get the userset config file
		//
		if let Some( path ) = meta.get( &Value::from( "userset" ) )
		{
			if let Value::String( path ) = path
			{
				user_conf = Some( path.clone() )
			}

			else
			{
				return Err( EkkeCfgError::ConfigParse.into() )
			}
		}

		// Separate metas from actual client settings
		//
		if let Some( map ) = meta.get_mut( &Value::from( "default" ) )
		{
			data = match map
			{
				Value::Mapping(x) => x,
				_                 => return Err( EkkeCfgError::ConfigParse.into() )
			};
		}

		else
		{
			return Err( EkkeCfgError::ConfigParse.into() )
		}


		let defaults: T     = from_str( &serde_yaml::to_string( &data )? )? ;


		// Get the userset config file
		//
		if let Some( path ) = user_conf
		{
			let file = File::open( path )?;
			let mut buf_reader = BufReader::new( file );
			let mut contents = String::new();
			buf_reader.read_to_string( &mut contents )?;

			let users: Value = from_str( &contents )?;

			let users2 = match users
			{
				Value::Mapping(x) => x,
				_                 => return Err( EkkeCfgError::ConfigParse.into() )
			};


			userset = Some( users2.clone() );

			data.merge( users2 )?;
		}

		let settings: T     = from_str( &serde_yaml::to_string( &data )? )? ;

		Ok( Config
		{
			defaults                         ,
			settings                         ,
			userset                          ,
			meta    : meta ,
			runtime : None                   ,
		})
	}
}



/// Convert from a file containing an yaml string
///
impl<T> TryFrom< &File > for Config<T> where T: Merge + Clone + DeserializeOwned + Serialize
{
	type Error = Error;

	fn try_from( file: &File ) -> Result< Self, Self::Error >
	{
		let mut buf_reader = BufReader::new(file);
		let mut contents = String::new();
		buf_reader.read_to_string( &mut contents )?;

		Config::try_from( contents.as_str() )
	}
}



/// Convert from a file containing an yaml string
///
impl<T> TryFrom< &Path > for Config<T> where T: Merge + Clone + DeserializeOwned + Serialize
{
	type Error = Error;

	fn try_from( path: &Path ) -> Result< Self, Self::Error >
	{
		let file = File::open( path )?;
		Config::try_from( &file )
	}
}



/// Convert from a file containing an yaml string
///
impl<T> TryFrom< &PathBuf > for Config<T> where T: Merge + Clone + DeserializeOwned + Serialize
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
	// use super::*;



}
