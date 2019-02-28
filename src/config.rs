use failure     :: { Error, Fail                                                                    } ;
use std         :: { convert::TryFrom, fs::File, io::BufReader, io::Read, path::Path, path::PathBuf } ;
use serde       :: { ser::Serialize, Deserialize, /*ser::Serializer,*/ de::DeserializeOwned         } ;
use serde_yaml  :: { Value, Mapping, from_str                                                       } ;
use crate       :: { EkkeResult, EkkeCfgError                                                       } ;
use ekke_merge  :: { Merge, MergeResult                                                             } ;


/// A configuration object that can be created from multiple layers of yaml input. Later
/// input will merge into the earlier data and override options that are already set.
/// Objects will be merged recursively. Arrays contents will be replaced.
///
#[ derive( Debug, Clone, PartialEq, Eq, Default, Deserialize ) ]
//
pub struct Config<T> where T: Merge + Clone + Serialize
{
	settings : T                 ,

	// Meta settings
	//
	user_conf: Option< String  > ,

	default  :         Mapping   ,
	userset  : Option< Mapping > ,
	runtime  : Option< Mapping > ,
}


impl<T> Config<T> where T: Merge + Clone + DeserializeOwned + Serialize
{
	/// Merge userset settings into this config. Usually userset configuration comes
	/// from a file in the users home directory, but in case the program allows modifying
	/// configuration from a dialog, user configuration might change on runtime.
	///
	pub fn merge_userset( &mut self, input: &str ) -> MergeResult<()>
	{
		let us: Mapping = from_str( input )?;

		// Store runtime for later reference
		//
		match &mut self.userset
		{
			None        => { self.userset = Some( us ); }
			Some( cfg ) => { cfg.merge( us.clone() )? ; }
		}

		// Regenerate self.settings.
		//
		self.regen()?;

		Ok(())
	}


	/// Add runtime configuration to the Config object. This will automatically be reflected
	/// in the output of .get()
	///
	pub fn merge_runtime( &mut self, input: &str ) -> MergeResult<()>
	{
		let rt: Mapping = from_str( input )?;

		// Store runtime for later reference
		//
		match &mut self.runtime
		{
			None        => { self.runtime = Some( rt ); }
			Some( cfg ) => { cfg.merge( rt.clone() )? ; }
		}

		// Regenerate the settings with runtime merged in.
		//
		self.regen()?;

		Ok(())
	}


	/// Get a reference to the actual settings. These are a result of merging in defaults,
	/// userset and runtime.
	///
	pub fn get( &self ) -> &T
	{
		&self.settings
	}


	/// Get a copy of the defaults.
	///
	pub fn default( &self ) -> Value
	{
		self.default.clone().into()
	}


	/// Get a copy of the settings that where set by the user.
	///
	pub fn userset( &self ) -> Option< Value >
	{
		match &self.userset
		{
			Some( value ) => Some( value.clone().into() ),
			None          => None          ,
		}
	}


	/// Get a copy of the settings that where added at runtime
	///
	pub fn runtime( &self ) -> Option< Value >
	{
		match &self.runtime
		{
			Some( value ) => Some( value.clone().into() ),
			None          => None          ,
		}
	}


	// Regenerate the final settings from intermediate values. For when userset or runtime
	// have changed.
	//
	fn regen( &mut self ) -> MergeResult<()>
	{
		let mut settings = self.default.clone();

		if let Some( us ) = &self.userset { settings.merge( us.clone() )?; }
		if let Some( rt ) = &self.runtime { settings.merge( rt.clone() )?; }

		self.settings = from_str( &serde_yaml::to_string( &settings )? )?;

		Ok(())
	}
}


/*fn val2map( val: Value ) -> EkkeResult< Mapping >
{
	match val
	{
		Value::Mapping(x) => Ok( x ),
		_                 => return Err( EkkeCfgError::ConfigParse.into() )
	}
}


fn val2map_ref( val: &Value ) -> EkkeResult< &Mapping >
{
	match val
	{
		Value::Mapping(x) => Ok( x ),
		_                 => return Err( EkkeCfgError::ConfigParse.into() )
	}
}*/


fn val2map_mut( val: &mut Value ) -> EkkeResult< &mut Mapping >
{
	match val
	{
		Value::Mapping(x) => Ok ( x                                ),
		_                 => Err( EkkeCfgError::ConfigParse.into() ),
	}
}


fn read_file( path: &str ) -> EkkeResult< String >
{
	let     file       = File::open( path )?;
	let mut buf_reader = BufReader::new( file );
	let mut contents   = String::new();

	buf_reader.read_to_string( &mut contents )?;

	Ok( contents )
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
		let mut meta   : Mapping           = from_str( input )? ;
		let mut userset: Option< Mapping > = None;

		let mut user_conf = None;

		// Get the userset config file
		// If it's present...
		//
		if let Some( path ) = meta.get( &"userset".into() )
		{
			// ...it has to be a string
			//
			match path
			{
				Value::String( path ) => user_conf = Some( path.clone() ),
				_                     => return Err( EkkeCfgError::ConfigParse.context( "user_conf must be a string" ).into() )
			}
		}

		// Get client settings as &mut Mapping without the metas
		//
		let data =

			val2map_mut( meta.get_mut( &"default".into() )

				.ok_or( EkkeCfgError::ConfigParse )? )?;


		// Store the actual settings as defaults
		//
		let mut default = Mapping::new();
		std::mem::swap( &mut default, data );


		// Read the userset config file
		//
		if let Some( path ) = &user_conf
		{
			let users: Mapping = from_str( &read_file( &path )? )?;

			userset = Some( users.clone() );
		}


		// Generate the final settings
		//
		let mut def = default.clone();

		// Merge userset
		//
		if let Some( us ) = &userset { def.merge( us.clone() )?; }

		// use deserialize, serialize to convert Mapping to T
		//
		let settings: T = from_str( &serde_yaml::to_string( &def )? )?;


		Ok( Config
		{
			default         ,
			settings        ,
			user_conf       ,
			userset         ,
			runtime : None  ,
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
