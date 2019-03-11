use crate :: { import::*, EkkeResult, EkkeCfgError };


/// A configuration object that can be created from multiple layers of yaml input. Later
/// input will merge into the earlier data and override options that are already set.
/// Objects will be merged recursively. Arrays contents will be replaced.
///
#[ derive( Debug, Clone, PartialEq, Eq, Default, Deserialize ) ]
//
pub struct Config<T> where T: Clone + Serialize + Debug
{
	settings : T                 ,

	// Meta settings
	//
	usr_path : Option< PathBuf > ,
	def_path : Option< PathBuf > ,

	default  :         Mapping   ,
	userset  : Option< Mapping > ,
	runtime  : Option< Mapping > ,
}



impl<T> Config<T> where T: Clone + DeserializeOwned + Serialize + Debug
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
			None          => None                        ,
		}
	}


	/// Get a copy of the settings that where added at runtime
	///
	pub fn runtime( &self ) -> Option< Value >
	{
		match &self.runtime
		{
			Some( value ) => Some( value.clone().into() ),
			None          => None                        ,
		}
	}



	/// Getter for the path to the default configuration file
	///
	pub fn def_path( &self ) -> &Option< PathBuf >
	{
		&self.def_path
	}



	/// Getter for the path to the user configuration file
	///
	pub fn usr_path( &self) -> &Option< PathBuf >
	{
		&self.usr_path
	}



	/// Update the path to the default configuration file.
	/// Updating this has no side-effects. It's just stored for future reference.
	/// Notably, this will not reparse the new file. This setter is mainly meant for
	/// when the Config was made from a string or File object and ekke_config doesn't
	/// know on object creation where the defaults come from.
	///
	pub fn set_def_path( &mut self, path: Option< PathBuf > )
	{
		self.def_path = path
	}



	/// Update the path to the user configuration file.
	/// Updating this has no side-effects. It's just stored for future reference.
	/// Notably, this will not reparse the new file.
	///
	pub fn set_usr_path( &mut self, path: Option< PathBuf > )
	{
		self.usr_path = path
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



/// Convert from yaml string
///
impl<T> TryFrom< &str > for Config<T> where T: Clone + DeserializeOwned + Serialize + Debug
{

	type Error = Error;

	fn try_from( input: &str ) -> Result< Self, Self::Error >
	{
		let mut meta   : Mapping           = from_str( input )? ;
		let mut userset: Option< Mapping > = None;

		let mut usr_path = None;

		// Get the userset config file
		// If it's present...
		//
		if let Some( path ) = meta.get( &"userset".into() )
		{
			// ...it has to be a string
			//
			match path
			{
				Value::String( path ) => usr_path = Some( PathBuf::from( path ) ),
				_                     => return Err( EkkeCfgError::ConfigParse.context( "usr_path must be a string" ).into() )
			}
		}

		// Get client settings as &mut Mapping without the metas
		//
		let data =

			val2map_mut
			(
				meta.get_mut( &"default".into() )

				.ok_or( EkkeCfgError::ConfigParse.context( "Default configuration must have a 'default' key in the root." ) )?

			).context( "The 'default' entry in the configuration root must be an object." )?;


		// Store the actual settings as defaults
		//
		let mut default = Mapping::new();
		std::mem::swap( &mut default, data );


		// Read the userset config file
		//
		if let Some( path ) = &usr_path
		{
			let users: Mapping =

				// This reads the file from the userset property as given in defaults.
				// shellexpand::tilde wil expand the home directory.
				// TODO: we should probably use path.to_str and throw an error if it's not valid unicode
				// TODO: make cross platform
				//
				from_str( &read_file( &Path::new( tilde( path.to_string_lossy().as_ref() ).as_ref() ) )

				.context( format!( "{:?}", path ) )? ).context( format!( "Failed to parse yaml at: {:?}", path ) )?
			;

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

		dbg!( &settings );

		Ok( Config
		{
			default         ,
			settings        ,
			userset         ,

			usr_path        ,
			def_path: None  ,
			runtime : None  ,
		})
	}
}



/// Convert from a file containing an yaml string
///
impl<T> TryFrom< &File > for Config<T> where T: Clone + DeserializeOwned + Serialize + Debug
{
	type Error = Error;

	fn try_from( file: &File ) -> Result< Self, Self::Error >
	{
		let mut buf_reader = BufReader::new(file);
		let mut contents   = String::new();

		buf_reader.read_to_string( &mut contents )?;

		Config::try_from( contents.as_str() )
	}
}



/// Convert from a file containing an yaml string
///
impl<T> TryFrom< &Path > for Config<T> where T: Clone + DeserializeOwned + Serialize + Debug
{
	type Error = Error;

	fn try_from( path: &Path ) -> Result< Self, Self::Error >
	{
		let file = File::open( path ).context( format!( "{:?}", path ) )?;
		let mut cfg = Config::try_from( &file )?;

		cfg.def_path = Some( PathBuf::from( path ) );

		Ok( cfg )
	}
}



/// Convert from a file containing an yaml string
///
impl<T> TryFrom< &PathBuf > for Config<T> where T: Clone + DeserializeOwned + Serialize + Debug
{
	type Error = Error;

	fn try_from( path: &PathBuf ) -> Result< Self, Self::Error >
	{
		Config::try_from( Path::new( path ) )
	}
}



// Helper methods
//
#[ inline( always ) ]
//
fn val2map_mut( val: &mut Value ) -> EkkeResult< &mut Mapping >
{
	match val
	{
		Value::Mapping(x) => Ok ( x                                ),
		_                 => Err( EkkeCfgError::ConfigParse.into() ),
	}
}


fn read_file( path: &Path ) -> EkkeResult< String >
{
	let     file       = File::open( path )?;
	let mut buf_reader = BufReader::new( file );
	let mut contents   = String::new();

	buf_reader.read_to_string( &mut contents )?;

	Ok( contents )
}




#[ cfg( test ) ]
//
mod tests
{
	// See tests folder
	//
	// Test:
	// - basic usage from strings
	// - basic usage from paths
	// - adding runtime
	// - override userset in runtime
	// - reading: defaults, userset, runtime, usr_path, def_path,



}
