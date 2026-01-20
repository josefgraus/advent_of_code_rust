use std::io;
use std::fs::File;

pub fn download_input(url: &str) -> String {
   let resp = reqwest::blocking::get(url).expect("request failed");
   let body = resp.text().expect("body invalid");

   // Save out a copy to inspect
   let mut out = File::create("input.txt").expect("failed to create file");
   io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");

   // Return copy received in response string
   return body;
}