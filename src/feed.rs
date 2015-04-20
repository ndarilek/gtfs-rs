extern crate csv;
extern crate zip;

use std::fs;
use std::io;
use std::io::Read;

#[derive(RustcDecodable)]
pub struct Agency {
  pub id: Option<String>,
  pub name: String,
  pub url: String,
  pub timezone: String,
  pub lang: Option<String>,
  pub phone: Option<String>,
  pub fare_url: Option<String>,
}

pub struct Feed {
  archive: zip::ZipArchive<fs::File>,
}

impl Feed {

  pub fn new(file: fs::File) -> Feed {
    Feed { archive: zip::ZipArchive::new(file).unwrap() }
  }

  pub fn agencies(&mut self) -> io::Result<Vec<Agency>> {
    let mut file = try!(self.archive.by_name("agency.txt"));
    let mut txt = String::new();
    try!(file.read_to_string(&mut txt));
    let mut data = csv::Reader::from_string(txt).has_headers(true).flexible(true);
    Ok(data.decode().map(|r| {
      println!("Here");
      let (id, name, url, timezone, lang, phone, fare_url) : (String, String, String, String, String, String, String) = r.unwrap();
      println!("Here2");
      Agency {
        id: if id.is_empty() { None } else { Some(id) },
        name: name,
        url: url,
        timezone: timezone,
        lang: if lang.is_empty() { None } else { Some(lang) },
        phone: if phone.is_empty() { None } else { Some(phone) },
        fare_url: if fare_url.is_empty() { None } else { Some(fare_url) }
      }
    }).collect())
  }

}