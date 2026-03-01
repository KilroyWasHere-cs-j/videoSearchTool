mod aipipe;

use crate::aipipe::gemini_api::ask_gemini;
use serca;
use serca::web::puppeteer::Puppeteer;
use tokio;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use anyhow::Result;
use std::fs;

async fn run() {
    fs::remove_file("spent_urls.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    println!("Launching Puppeteer");

    let puppeteer = Puppeteer::new();
    match puppeteer.await.control().await {
        Ok(()) => println!("The loop exited safely, but it still shouldn't have ended"),
        Err(e) => println!("The loop exited with an error {}", e)
    }

    println!("DONE");
}

#[tokio::main]
async fn main() -> Result<()> {

    let resp = ask_gemini("IzaSyB_dXpxFgXVhsBkrx-MaIfikWhMD2xUlT0", "Hello World").await;
    println!("{:#?}", resp);
    Ok(())
}

