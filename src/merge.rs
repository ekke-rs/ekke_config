use serde_hjson::{ Value };
use std::mem;
use std::mem::discriminant as discri;
use std::collections::{ BTreeMap, btree_map::Entry };
use std::vec::{ Vec };
use crate:: { EkkeResult, EkkeCfgError };



pub trait Merge
{
	fn merge( &mut self, other: Self ) -> EkkeResult<()>;
}



impl Merge for Value
{
	fn merge( &mut self, other: Self ) -> EkkeResult<()>
	{
		// We do not allow overriding values of a different type. This allows the code
		// to count on type safety.
		//
		if discri( self ) != discri( &other )
		{
			return Err( EkkeCfgError::MergeWrongType( self.clone(), other ).into() );
		}


		match other
		{
			// We do not allow unsetting configuration values. This allows setting a default
			// value and knowing that a value of the correct type will always exist, whatever
			// the user overrides.
			//
			Value::Null      => {},

			Value::Bool(_)   => { mem::replace( self, other ); } ,

			Value::I64(_)    => { mem::replace( self, other ); } ,

			Value::U64(_)    => { mem::replace( self, other ); } ,

			Value::F64(_)    => { mem::replace( self, other ); } ,

			Value::String(_) => { mem::replace( self, other ); } ,

			Value::Array(y)  =>
			{
				if let Value::Array( x ) = self
				{
					x.merge( y )?;
				}
			},

			Value::Object(y) =>
			{
				if let Value::Object( x ) = self
				{
					x.merge( y )?;
				}
			},
		};

		Ok(())
	}
}



impl Merge for BTreeMap< String, Value >
{
	fn merge( &mut self, other: Self ) -> EkkeResult<()>
	{
		for (k, v) in other.into_iter()
		{
			match self.entry( k.to_string() )
			{
				Entry::Occupied( mut e ) => { e.get_mut().merge( v )?; },
				Entry::Vacant  (     e ) => { e.insert( v )         ; },
			};
		}

		Ok(())
	}
}



impl Merge for Vec< Value >
{
	fn merge( &mut self, other: Self ) -> EkkeResult<()>
	{
		for v in other.into_iter()
		{
			if !self.contains( &v )
			{
				self.push( v );
			}
		}

		Ok(())
	}
}



#[ cfg( test ) ]
//
mod tests
{
	use super::*;

	use serde_hjson::{ from_str, Value };


	// Takes away some boilerplate
	//
	fn cmp( a: &str, b: &str, expect: &str )
	{
		let mut a : Value = from_str( a      ).unwrap();
		let b     : Value = from_str( b      ).unwrap();
		let c     : Value = from_str( expect ).unwrap();

		a.merge( b ).unwrap();

		assert_eq!( a, c );
	}



	#[test]
	//
	fn basic_array()
	{
		cmp( "[ 1 ]", "[ 2 ]", "[ 1, 2 ]" );
	}



	#[test]
	//
	fn empty_array()
	{
		cmp( "[]", "[ 1, 2 ]", "[ 1, 2 ]" );
	}



	#[test]
	//
	fn merge_empty_array()
	{
		cmp( "[ 1, 2 ]", "[]", "[ 1, 2 ]" );
	}



	#[test]
	//
	fn complex_array()
	{
		cmp( "[ 1, 3 ]", "[ 2, 4 ]", "[ 1, 3, 2, 4 ]" );
	}



	#[test]
	//
	fn overlap_array()
	{
		cmp( "[ 1, 3 ]", "[ 1, 2, 4 ]", "[ 1, 3, 2, 4 ]" );
	}


	#[ test         ]
	#[ should_panic ]
	//
	fn basic_object_null()
	{
		let mut a = Value::String( "bli".to_string() );
		let     b = Value::Null;

		a.merge( b ).unwrap();
	}


	#[test]
	//
	fn basic_object_bool()
	{
		cmp( "{ a: true\n }", "{ a: false\n }", "{ a: false\n }" );
	}


	#[test]
	//
	fn basic_object_empty()
	{
		cmp( "{}", "{ a: true\n }", "{ a: true\n }" );
	}


	#[test]
	//
	fn basic_object_merge_empty()
	{
		cmp( "{ a: true\n }", "{}", "{ a: true\n }" );
	}


	#[ test         ]
	#[ should_panic ]
	//
	fn basic_object_string_bool()
	{
		let mut a = Value::String( "bli".to_string() );
		let     b = Value::Bool( true );

		a.merge( b ).unwrap();
	}


	#[test]
	//
	fn basic_object_u64()
	{
		cmp( "{ a: 1 }", "{ a: 2 }", "{ a: 2 }" );
	}


	#[test]
	//
	fn basic_object_i64()
	{
		cmp( "{ a: -1 }", "{ a: -2 }", "{ a: -2 }" );
	}


	#[test]
	//
	fn basic_object_f64()
	{
		cmp( "{ a: -1.3 }", "{ a: -2.5 }", "{ a: -2.5 }" );
	}


	#[test]
	//
	fn basic_object_string()
	{
		cmp( "{ a: bli\n }", "{ a: bla\n }", "{ a: bla\n }" );
	}


	#[test]
	//
	fn nested_object()
	{
		let a = "
		{
			a: 1
			obj:
			{
				bli: bli
			}
		}";

		let b = "
		{
			obj:
			{
				bli: bla
			}
		}";

		let expect = "
		{
			a: 1
			obj:
			{
				bli: bla
			}
		}";

		cmp( a, b, expect );
	}


	#[test]
	//
	fn u64_nested_object()
	{
		let a = "
		{
			a: 1
			obj:
			{
				bli: bli
			}
		}";

		let b = "
		{
			a: 2
			obj:
			{
				bli: bla
			}
		}";

		let expect = "
		{
			a: 2
			obj:
			{
				bli: bla
			}
		}";

		cmp( a, b, expect );
	}


	#[test]
	//
	fn u64_nested_array()
	{
		let a = "
		{
			a: 1
			obj:
			{
				bli: [ 1 ]
			}
		}";

		let b = "
		{
			a: 2
			obj:
			{
				bli: [ 2, 4 ]
			}
		}";

		let expect = "
		{
			a: 2
			obj:
			{
				bli: [ 1, 2, 4 ]
			}
		}";

		cmp( a, b, expect );
	}
}

