use ekke_config :: { Config, Pointer               } ;
use std         :: { convert::TryFrom, path::Path  } ;
use serde       :: { Serialize, Deserialize        } ;



#[ derive( Serialize, Deserialize, Debug, Clone ) ]
//
struct Settings
{
	pub my_app    : MyAppOpts,
	pub other_comp: OtherCompOpts,
}

#[ derive( Serialize, Deserialize, Debug, Clone ) ]
//
struct MyAppOpts
{
	pub db_path: String,
	pub log_lvl: String,
}

#[ derive( Serialize, Deserialize, Debug, Clone ) ]
//
struct OtherCompOpts
{
	pub primes: Vec<usize>,
	pub algo  : String    ,
}


fn main() -> Result<(), failure::Error>
{
	// Config implements std::convert::TryFrom:
	// - &str
	// - &std::fs::File
	// - &std::path::Path
	// - &std::path::Pathbuf
	//
	// This will read userset key from the root of the defaults.yml for a filename and merge
	// in the user configuration file automatically.
	//
	// userset and runtime configuration can be incomplete. Not all keys in Settings need to
	// be present, but they must have the same layout so they can be merged.
	//
	let mut config: Config< Settings > = Config::try_from( Path::new( "data/defaults.yml" ) )?;

	config.merge_runtime( "my_app: { log_lvl: info }" )?;


	// new values are merged in
	//
	assert_eq!( config.get().my_app.log_lvl   , "info"                 );
	assert_eq!( config.get().other_comp.algo  , "euler"                );
	assert_eq!( config.get().other_comp.primes, vec![ 1, 3, 5, 7, 11 ] );

	// defaults and userset are still available
	//


	// This is ugly at the moment. Since userset and runtime can pass in incomplete config,
	// we cannot return a Settings object, so you get a Option< serde_yaml::Value >. ekke_config
	// adds a json pointer lookup to serde_yaml::Value for more convenient lookup.
	//
	// default() does not return an option, since it has to exist as soon as a Config is created.
	//
	assert_eq!( config.default()         .jptr( "/my_app/log_lvl" ).unwrap(), "debug" );
	assert_eq!( config.userset().unwrap().jptr( "/my_app/log_lvl" ).unwrap() , "warn" );
	assert_eq!( config.runtime().unwrap().jptr( "/my_app/log_lvl" ).unwrap() , "info" );

	// defaults bubble up if not overridden
	//
	assert_eq!( config.get().my_app.db_path, "data/db.sqlite" );

	Ok(())
}
