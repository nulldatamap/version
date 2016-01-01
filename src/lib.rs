use std::fmt;
use std::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, Eq, PartialEq)]
pub struct Version {
  pub major : u32,
  pub minor : u32,
  pub patch : u32
}

/// Gets the current version. If the enviroment is invalid it will panic.
pub fn current() -> Version {
  try_current().expect( "Failed to get current version" )
}

/// Tries to get the current version, returning a parse error if it fails.
pub fn try_current() -> Result<Version, ParseIntError> {
  Ok( Version {
    major : try!( env!( "CARGO_PKG_VERSION_MAJOR" ).parse::<u32>() ),
    minor : try!( env!( "CARGO_PKG_VERSION_MINOR" ).parse::<u32>() ),
    patch : try!( env!( "CARGO_PKG_VERSION_PATCH" ).parse::<u32>() ),
  } )
}

impl Display for Version {
  fn fmt( &self, fmtr : &mut fmt::Formatter ) -> fmt::Result {
    write!( fmtr, "{}.{}.{}", self.major, self.minor, self.patch )
  }
}

/// Gets the current version as a string.
#[macro_export]
macro_rules! version(
  () => (
    format!( "{}.{}.{}", env!( "CARGO_PKG_VERSION_MAJOR" )
                       , env!( "CARGO_PKG_VERSION_MINOR" )
                       , env!( "CARGO_PKG_VERSION_PATCH" ) )
  )
);

#[test]
fn does_it_work() {
  let ver = try_current();
  assert_eq!( ver, Ok( Version { major: 1, minor: 0, patch: 1 } ) );
  // Bad test is bad.
  assert_eq!( version!(), "1.0.1" );
}
