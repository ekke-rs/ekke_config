fn main(){}

// #![ feature( try_from ) ]

// use ekke_config :: { Config, Merge };
// use std         :: { convert::TryFrom, fs::File };


// fn main() -> Result<(), failure::Error>
// {
// 	// imagine a defaults.yml, normally we would write:
// 	//
// 	//   Config::try_from( &File::open( "defaults.yml" )? )?;
// 	//
// 	// For the example we will use inline strings instead.
// 	//
// 	// Config can implements std::convert::TryFrom: &str, &std::file::File, &std::path::Path, &std::path::Pathbuf
// 	//
// 	let mut defaults = Config::try_from(
// "
// user_conf: /home/user/myapp.yml

// MyApp:
//   db_path: /home/user/db.sqlite
//   log_lvl: debug

// OtherComponent:
//   primes: [ 1, 3, 5, 7 ]
//   algo  : fournier
// ")?;


// 	// This won't do anything here since the file doesn't exist. We just use inline strings for the example.
// 	// Shows how to have an optional user config. This just does nothing if errors occur. You might want to
// 	// handle them.
// 	//
// 	if let Some( value ) = defaults.get( "/user_conf" ) {
// 	if let Some( path  ) = value.as_str()               {
// 	if let Ok  ( file  ) = File::open( &path )
// 	{

// 		defaults.merge( Config::try_from( &file )? )?;

// 	}}}


// 	let user_conf = Config::try_from(
// "
// MyApp:
//   db_path: /home/user/myapp.sqlite
//   log_lvl: warn

// OtherComponent:
//   primes: [ 1, 3, 5, 7, 11 ]
//   algo  : euler
// ")?;

// 	// Clone the defaults if you want to keep the defaults for reference.
// 	//
// 	let mut settings = defaults.clone();
// 	settings.merge( user_conf )?;

// 	// new values are merged in
// 	//
// 	assert_eq!( settings.get( "/OtherComponent/algo" ).unwrap().as_str().unwrap(), "euler" );

// 	// new values are merged in
// 	//
// 	assert_eq!( settings.get( "/OtherComponent/primes/4" ).unwrap().as_u64().unwrap(), 11 );

// 	// unexisting values in the user_conf remain untouched
// 	//
// 	assert_eq!( settings.get( "/user_conf" ).unwrap().as_str().unwrap(), "/home/user/myapp.yml" );


// 	Ok(())
// }
