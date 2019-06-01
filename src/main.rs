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


    let mut sections = vec![
      format!("= {}\n", self.info.title),
    ];

    if let Some(desc) = self.info.description.as_ref() {
      sections.push(
        format!(indoc!("
          [[_overview]]
          == Overview
          [%hardbreaks]
          {}
        "), desc)
      );
    }

    sections.push(
      format!(indoc!("
        === Version information
        [%hardbreaks]
        __Version__ : {}
      "
      ), self.info.version)
    );
    
    if let Some(contact) = self.info.contact.as_ref() {
      // let mut contact_url = "".to_owned();
      // if let Some(url) = &contact.url {
      //   println!("{:#?}", url.0);
      // }
      sections.push(
        format!(
          indoc!("
            === Contact information
            [%hardbreaks]
            __Contact__ : {name}
            __Contact Email__ : {email}
            __URL__ : {url}
          "),
          name = contact.name.as_ref().unwrap_or(&"".to_owned()),
          email = contact.email.as_ref().unwrap_or(&"".to_owned()),
          // TODO: weirdness with url wrapping in `openapi`
          url = "".to_owned()
        )
      );
    }

    sections.push(
      format!(indoc!("
        === License information
        [%hardbreaks]
        __License__ : {license}
        __License URL__ : {url}
        __Terms of service__ : {terms}
      "), license = "", url = "", terms = "")
    );
    
    sections.push(
      format!(indoc!("
        === URI scheme
        [%hardbreaks]
        __Host__ : {host}
        __BasePath__ : {base_path}
        __Schemes__ : {schemes}
      "), host = "", base_path = "", schemes = "")
    );

    sections.join("\n\n")
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
  let openapi = openapi::from_path("test/expanded.openapi.yml")?;
  // println!("spec: {:#?}", spec);
  match openapi {
    openapi::OpenApi::V2(spec) => println!("{}", spec.to_asciidoc()),
    openapi::OpenApi::V3_0(spec) => println!("{}", spec.to_asciidoc()),
  }
  // println!("{}", openapi.v3_0.to_asciidoc());

  Ok(())
}
