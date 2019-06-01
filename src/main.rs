#![feature(proc_macro_hygiene)]

use indoc::indoc;
use std::collections::BTreeMap;
use http::Method;

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

[[_addpet]]
=== POST /pets

==== Description
Creates a new pet in the store. Duplicates are allowed


==== Parameters

[options="header", cols=".^2a,.^3a,.^9a,.^4a"]
|===
|Type|Name|Description|Schema
|**Body**|**pet** +
__required__|Pet to add to the store|<<_newpet,NewPet>>
|===


==== Responses

[options="header", cols=".^2a,.^14a,.^4a"]
|===
|HTTP Code|Description|Schema
|**200**|pet response|<<_pet,Pet>>
|**default**|unexpected error|<<_errormodel,ErrorModel>>
|===


==== Produces

* `application/json`

*/

fn build_path(path: String, method: Method, operation: &openapi::v3_0::Operation) -> String {
  // println!("{}: \"{:#?}\"", path, operation);
  let mut head_section: Vec<String> = vec![];
  if let Some(operation_id) = operation.operation_id.as_ref() {
    head_section.push(format!("[[_{}]]", operation_id));
  }
  head_section.push(
    format!(
      "=== {method} {path}",
      method = method.as_str(),
      path = path
    )
  );

  let heading = head_section.join("\n");

  heading
}

fn build_paths(paths: BTreeMap<String, openapi::v3_0::PathItem>) -> String {
  let mut paths_adoc: Vec<String> = vec![
    indoc!("
      [[_paths]]
      == Paths
    ").to_string()
  ];

  for (path, item) in &paths {
    if let Some(operation) = item.get.as_ref() {
      paths_adoc.push(build_path(path.to_string(), Method::GET, operation))
    }

    // if let Some(&operation) = item.put.as_ref() {
    //   paths_adoc.push(build_path(path.to_string(), Method::PUT, operation))
    // }

    // if let Some(&operation) = item.post.as_ref() {
    //   paths_adoc.push(build_path(path.to_string(), Method::POST, operation))
    // }

    // if let Some(&operation) = item.delete.as_ref() {
    //   paths_adoc.push(build_path(path.to_string(), Method::DELETE, operation))
    // }

    // if let Some(&operation) = item.options.as_ref() {
    //   paths_adoc.push(build_path(path.to_string(), Method::OPTIONS, operation))
    // }

    // if let Some(&operation) = item.head.as_ref() {
    //   paths_adoc.push(build_path(path.to_string(), Method::HEAD, operation))
    // }

    // if let Some(&operation) = item.patch.as_ref() {
    //   paths_adoc.push(build_path(path.to_string(), Method::PATCH, operation))
    // }

    // if let Some(&operation) = item.trace.as_ref() {
    //   paths_adoc.push(build_path(path.to_string(), Method::TRACE, operation))
    // }
  }

  paths_adoc.join("\n\n")
}

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
    
    let paths = build_paths(self.paths.clone());

    sections.push(paths);

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
  let openapi = openapi::from_path("test/simple.openapi.yml")?;
  // println!("spec: {:#?}", spec);
  match openapi {
    openapi::OpenApi::V2(spec) => println!("{}", spec.to_asciidoc()),
    openapi::OpenApi::V3_0(spec) => println!("{}", spec.to_asciidoc()),
  }
  // println!("{}", openapi.v3_0.to_asciidoc());

  Ok(())
}
