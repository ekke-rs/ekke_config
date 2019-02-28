use serde_yaml  :: { Value, Number } ;
use ekke_config :: { Pointer       } ;

mod common;
use common::*;



#[ test ] fn test_defaults()
{
	let cfg = runtime_data();

	assert_eq!( cfg.default().jptr( "/my_app/db_path"  ).unwrap(), "data/db.sqlite"   );
	assert_eq!( cfg.default().jptr( "/my_app/log_lvl"  ).unwrap(), "debug"            );
	assert_eq!( cfg.default().jptr( "/other_comp/algo" ).unwrap(), "fournier"         );


	assert_eq!
	(
		cfg.default().jptr( "/other_comp/primes" ).unwrap().as_sequence().unwrap().clone(),

		vec!
		[
			Value::Number( Number::from( 1 ) ),
			Value::Number( Number::from( 3 ) ),
			Value::Number( Number::from( 5 ) ),
			Value::Number( Number::from( 7 ) ),
		]
	);
}


#[ test ] fn test_userset()
{
	let cfg = runtime_data();

	assert_eq!( cfg.userset().unwrap().jptr( "/my_app/log_lvl"  ).unwrap(), "warn"  );
	assert_eq!( cfg.userset().unwrap().jptr( "/other_comp/algo" ).unwrap(), "euler" );


	assert_eq!
	(
		cfg.userset().unwrap().jptr( "/other_comp/primes" ).unwrap().as_sequence().unwrap().clone(),

		vec!
		[
			Value::Number( Number::from( 1  ) ),
			Value::Number( Number::from( 3  ) ),
			Value::Number( Number::from( 5  ) ),
			Value::Number( Number::from( 7  ) ),
			Value::Number( Number::from( 11 ) ),
		]
	);
}


#[ test ] fn test_runtime()
{
	let cfg = runtime_data();

	assert_eq!( cfg.runtime().unwrap().jptr( "/my_app/log_lvl" ).unwrap(), "info" );
}


#[ test ] fn test_settings()
{
	let cfg = runtime_data();

	assert_eq!( cfg.get().my_app.db_path , "data/db.sqlite" );
	assert_eq!( cfg.get().my_app.log_lvl , "info"           );
	assert_eq!( cfg.get().other_comp.algo, "euler"          );

	assert_eq!( cfg.get().other_comp.primes, vec![ 1, 3, 5, 7, 11 ] );
}


#[ test ] fn test_paths()
{
	let cfg = runtime_data();

	assert_eq!( cfg.usr_path().clone().unwrap().to_str().unwrap(), "data/userset.yml" );
	assert_eq!( cfg.def_path().clone().unwrap().to_str().unwrap(), "data/defaults.yml" );
}
