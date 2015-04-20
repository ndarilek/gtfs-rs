extern crate gtfs;

use std::fs::File;

fn main() {
  let file = File::open("examples/data/gtfs.zip");
  let mut feed = gtfs::Feed::new(file.unwrap());
  println!("Agencies: {}", feed.agencies().unwrap().len());
}