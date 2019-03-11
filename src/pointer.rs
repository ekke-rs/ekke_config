use crate :: { import::* };


pub trait Pointer
{
	fn jptr<'a>( &'a self, pointer: &str ) -> Option< &'a Value >;
}

impl Pointer for Value
{
	/// See: https://docs.rs/serde-hjson/0.9.0/serde_hjson/value/enum.Value.html
	///
	fn jptr<'a>( &'a self, pointer: &str ) -> Option< &'a Value >
	{
		fn parse_index( s: &str ) -> Option< usize >
		{
			if s.starts_with( '+' ) || ( s.starts_with( '0' ) && s.len() != 1 )
			{
				return None;
			}

			s.parse().ok()
		}


		if  pointer == ""              { return Some( self ) }
		if !pointer.starts_with( '/' ) { return None               }


		let mut target = self;

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


/*
impl Pointer for Mapping
{
	/// See: https://docs.rs/serde-hjson/0.9.0/serde_hjson/value/enum.Value.html
	///
	fn pointer<'a>( &'a self, pointer: &str ) -> Option< &'a Value >
	{
		fn parse_index( s: &str ) -> Option< usize >
		{
			if s.starts_with( '+' ) || ( s.starts_with( '0' ) && s.len() != 1 )
			{
				return None;
			}

			s.parse().ok()
		}


		if  pointer == ""              { return Some( &Value::Mapping( self.clone() ) ) }
		if !pointer.starts_with( '/' ) { return None               }


		let mut target = Value::Mapping( *self );

		for escaped_token in pointer.split( '/' ).skip(1)
		{
			let token = escaped_token.replace( "~0", "~" ).replace( "~1", "/" );


			let target_opt = match target
			{
				Value::Mapping ( ref map  ) => map.get    ( &token.into() )                             ,
				Value::Sequence( ref list ) => parse_index( &token        ).and_then( |x| list.get(x) ) ,
				_                           => return None                                              ,
			};

			match target_opt
			{
				Some( t ) => target = *t ,
				None      => return None,
			}
		}


		Some( &target )
	}
}
*/
