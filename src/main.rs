#![feature(proc_macro_hygiene)]

use indoc::indoc;

#[derive(Debug)]
enum MyErr {
    OpenapiError,
}

impl From<openapi::error::Error> for MyErr {
    fn from(_: openapi::error::Error) -> Self {
        MyErr::OpenapiError
    }
}

trait DisplayAsciidoc {
    fn to_asciidoc(&self) -> String {
      // TODO: remove default implementation
      "".to_string()
    }
}

// impl DisplayAsciidoc for openapi::OpenApi {
//   fn to_asciidoc(&self) -> String {
//     self.spec
//     // ""
//   }
// }

impl DisplayAsciidoc for openapi::v2::Spec {
  fn to_asciidoc(&self) -> String {
    "".to_string()
  }
}


/*

*/

impl DisplayAsciidoc for openapi::v3_0::Spec {
  fn to_asciidoc(&self) -> String {

    let mut overview_adoc = "".to_owned();
    if let Some(desc) = self.info.description.as_ref() {
      overview_adoc = format!(indoc!("
        [[_overview]]
        == Overview
        {}
      "), desc);
    }

    let version_adoc = format!(indoc!("
      === Version information
      [%hardbreaks]
      __Version__ : {version}
    "), version = "1.0.0");

    let contact_adoc = format!(indoc!("
      === Contact information
      [%hardbreaks]
      __Contact__ : {name}
      __Contact Email__ : {email}
    "), name = "", email = "");

    let license_adoc = format!(indoc!("
      === License information
      [%hardbreaks]
      __License__ : {license}
      __License URL__ : {url}
      __Terms of service__ : {terms}
    "), license = "", url = "", terms = "");
    
    let uri_adoc = format!(indoc!("
      == URI scheme
      [%hardbreaks]
      __Host__ : {host}
      __BasePath__ : {base_path}
      __Schemes__ : {schemes}
    "), host = "", base_path = "", schemes = "");
    

    return format!(
      indoc!("
        = {title}

        {overview}

        {version}

        {contact}

        {license}

        {uri}
      "),
      title = self.info.title,
      overview = overview_adoc,
      version = version_adoc,
      contact = contact_adoc,
      license = license_adoc,
      uri = uri_adoc
    );
  }
}

impl DisplayAsciidoc for openapi::v3_0::Info {
  fn to_asciidoc(&self) -> String {
    "".to_string()
    // let title format!(indoc!("
    //   = {}
    // "), self.title);
  }
}

fn main() -> std::result::Result<(), MyErr> {
  // let spec = openapi::from_path("expanded.openapi.yml");
  // println!("thing: {}", spec);
  let openapi = openapi::from_path("test/simple.openapi.yml")?;
  // println!("spec: {:#?}", spec);
  match openapi {
    openapi::OpenApi::V2(spec) => println!("{}", spec.to_asciidoc()),
    openapi::OpenApi::V3_0(spec) => println!("{}", spec.to_asciidoc()),
  }
  // println!("{}", openapi.v3_0.to_asciidoc());

  Ok(())
}
