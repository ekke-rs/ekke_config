use ekke_config      :: { Config, Pointer                     };
use std              :: { convert::TryFrom, fs::File };
use ekke_merge       :: { MergeResult                      };
use ekke_merge_derive:: { Merge                      };

use serde       :: { Serialize, Deserialize                          } ;
use serde_yaml  :: { Value,                                                                 } ;


#[ derive( Serialize, Deserialize, Debug, Clone, Merge ) ]
//
struct Settings
{
	pub my_app    : MyAppOpts,
	pub other_comp: OtherCompOpts,
}

#[ derive( Serialize, Deserialize, Debug, Clone, Merge ) ]
//
struct MyAppOpts
{
	pub db_path: String,
	pub log_lvl: String,
}

#[ derive( Serialize, Deserialize, Debug, Clone, Merge ) ]
//
struct OtherCompOpts
{
	pub primes: Vec<usize>,
	pub algo  : String    ,
}


fn main() -> Result<(), failure::Error>
{
	// imagine a defaults.yml, normally we would write:
	//
	//   Config::try_from( &File::open( "defaults.yml" )? )?;
	//
	// For the example we will use inline strings instead.
	//
	// Config can implements std::convert::TryFrom: &str, &std::fs::File, &std::path::Path, &std::path::Pathbuf
	//
	let mut settings: Config<Settings> = Config::try_from( &File::open( "data/defaults.yml" )? )?;

	settings.merge_runtime( "my_app: { log_lvl: info }" )?;


	// new values are merged in
	//
	assert_eq!( settings.get().other_comp.algo    , "euler"           );
	assert_eq!( settings.get().my_app.log_lvl     , "info"            );

	// defaults and userset are still available
	//
	assert_eq!( settings.default().my_app.log_lvl , "debug"           );


	// This is ugly at the moment. Since userset and runtime can pass in incomplete settings, we cannot return a Settings object, so you get a serde_yaml::Value.
	//
	assert_eq!( Value::Mapping( settings.userset().unwrap().clone() ).pointer( "/my_app/log_lvl" ).unwrap() , "warn"            );
	assert_eq!( Value::Mapping( settings.runtime().unwrap().clone() ).pointer( "/my_app/log_lvl" ).unwrap() , "info"            );

	// defaults bubble through if not overridden
	//
	assert_eq!( settings.get().my_app.db_path     , "data/db.sqlite"  );

	Ok(())
}
