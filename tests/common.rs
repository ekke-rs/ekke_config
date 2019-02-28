// Unfortunately rust doesn't get that this file has no tests, and is thus an included file...
//
#![allow(dead_code)]


use serde       :: { Serialize, Deserialize       } ;
use std         :: { convert::TryFrom, path::Path } ;
use ekke_config :: { Config                       } ;


#[ derive( Serialize, Deserialize, Debug, Clone ) ]
//
pub struct Settings
{
	pub my_app    : MyAppOpts,
	pub other_comp: OtherCompOpts,
}

#[ derive( Serialize, Deserialize, Debug, Clone ) ]
//
pub struct MyAppOpts
{
	pub db_path: String,
	pub log_lvl: String,
}

#[ derive( Serialize, Deserialize, Debug, Clone ) ]
//
pub struct OtherCompOpts
{
	pub primes: Vec<usize>,
	pub algo  : String    ,
}




pub fn basic_data() -> Config<Settings>
{
	let def =
"
default:
  my_app:
    db_path: data/db.sqlite
    log_lvl: debug

  other_comp:
    primes: [ 1, 3, 5, 7 ]
    algo  : fournier
";

	let user =
"
my_app:
  log_lvl: warn

other_comp:
  primes: [ 1, 3, 5, 7, 11 ]
  algo  : euler
";

	let mut cfg = Config::try_from( def ).unwrap();

	cfg.merge_userset( user ).unwrap();
	cfg.set_def_path( Some( "data/defaults.yml".into() ) );
	cfg.set_usr_path( Some( "data/userset.yml".into()  ) );

	cfg
}


pub fn file_data() -> Config<Settings>
{
	Config::try_from( Path::new( "data/defaults.yml" ) ).unwrap()
}



pub fn runtime_data() -> Config<Settings>
{
	let mut cfg = file_data();

	cfg.merge_runtime( "my_app: { log_lvl: info }" ).unwrap();

	cfg
}



pub fn runtime_userset_data() -> Config<Settings>
{
	let mut cfg = runtime_data();

	cfg.merge_userset( "{ my_app: { log_lvl: error }, other_comp: { algo: Gauss } }" ).unwrap();

	cfg
}

