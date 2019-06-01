#[derive(Debug)]
enum MyErr {
    OpenapiError,
}

impl From<openapi::error::Error> for MyErr {
    fn from(_: openapi::error::Error) -> Self {
        MyErr::OpenapiError
    }
}

trait ToAsciidoc {
    // Traits can provide default method definitions.
    fn convert(&self) -> &'static str {
        ""
    }
}

type SpecInfo = String

fn main() -> std::result::Result<(), MyErr> {
  // let spec = openapi::from_path("expanded.openapi.yml");
  // println!("thing: {}", spec);
  let spec = openapi::from_path("test/simple.openapi.yml")?;
  println!("spec: {:#?}", spec);
  // let spec = match openapi::from_path("expanded.openapi.yml") {
  //   Ok(spec) => spec,
  //   Err(err) => panic!(err),
  // };
  Ok(())
}
