
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
  // Bad test is bad.
  assert_eq!( version!(), "1.0.0" );
}
