// Copyright 2026 Scale Invariant, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(private_interfaces)]

//! Dictionary refresh — downloads public domain name/city/word data,
//! transforms to the format consumed by apcrd_dictionaries.
//! Usage: cargo run --bin apcad

// RCG output discipline: all emission via apcrl_*! — no direct println!/eprintln!

use std::collections::{BTreeSet, HashMap};
use std::io::Read;

// Output directory — baked at compile time from Cargo.toml location
const ZAPCAD_DICT_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/dictionaries");

// SSA staging path — user places manually downloaded zip here on 403
const ZAPCAD_SSA_STAGING: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/dictionaries/.ssa_names_staging.zip");

// Source URLs
const ZAPCAD_URL_SURNAMES:  &str =
    "https://www2.census.gov/topics/genealogy/2010surnames/names.zip";
const ZAPCAD_URL_SSA_NAMES: &str =
    "https://www.ssa.gov/oact/babynames/names.zip";
const ZAPCAD_URL_CITIES:    &str =
    "https://www2.census.gov/programs-surveys/popest/datasets/2020-2023/cities/totals/sub-est2023.csv";
const ZAPCAD_URL_ENGLISH:   &str =
    "https://norvig.com/ngrams/count_1w.txt";

// Limits
const ZAPCAD_SURNAME_LIMIT:   usize = 1000;
const ZAPCAD_FIRSTNAME_LIMIT: usize = 1000;
const ZAPCAD_CITY_POP_LIMIT:  usize = 100;
const ZAPCAD_ENGLISH_LIMIT:   usize = 5000;

// Municipal suffixes to strip (case-insensitive, trailing)
const ZAPCAD_CITY_SUFFIXES: &[&str] = &[
    " city", " town", " village", " borough", " municipality", " cdp",
];

// 50 US state capitals
const ZAPCAD_STATE_CAPITALS: &[&str] = &[
    "albany", "annapolis", "atlanta", "augusta", "austin",
    "baton rouge", "bismarck", "boise", "boston", "carson city",
    "charleston", "cheyenne", "columbia", "columbus", "concord",
    "denver", "des moines", "dover", "frankfort", "harrisburg",
    "hartford", "helena", "honolulu", "indianapolis", "jackson",
    "jefferson city", "juneau", "lansing", "lincoln", "little rock",
    "madison", "montgomery", "montpelier", "nashville", "oklahoma city",
    "olympia", "phoenix", "pierre", "providence", "raleigh",
    "richmond", "sacramento", "salem", "salt lake city", "santa fe",
    "springfield", "st. paul", "tallahassee", "topeka", "trenton",
];

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    apcd::apcrl_info_now!("dictionary refresh starting");
    apcd::apcrl_info_now!("output directory: {}", ZAPCAD_DICT_DIR);

    let surnames = zapcad_fetch_surnames();
    zapcad_write_dict("surnames.txt", &surnames);

    let firstnames = zapcad_fetch_firstnames();
    zapcad_write_dict("firstnames.txt", &firstnames);

    let cities = zapcad_fetch_cities();
    zapcad_write_dict("cities.txt", &cities);

    let english = zapcad_fetch_english();
    zapcad_write_dict("english_whitelist.txt", &english);

    apcd::apcrl_info_now!("dictionary refresh complete — 4 files written");
}

// ---------------------------------------------------------------------------
// Surnames — Census 2010 (top 1000)
// ---------------------------------------------------------------------------

fn zapcad_fetch_surnames() -> Vec<String> {
    apcd::apcrl_info_now!("fetching surnames from Census 2010");
    let zip_bytes = zapcad_download_bytes(ZAPCAD_URL_SURNAMES);
    let csv_text = zapcad_extract_zip_entry(&zip_bytes, "Names_2010Census.csv");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_text.as_bytes());

    let headers = rdr.headers()
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("surnames CSV headers: {}", e))
        .clone();
    let name_idx = headers.iter().position(|h| h == "name")
        .unwrap_or_else(|| apcd::apcrl_fatal_now!("surnames CSV missing 'name' column"));

    let mut names = Vec::new();
    for result in rdr.records() {
        if names.len() >= ZAPCAD_SURNAME_LIMIT { break; }
        let record = result
            .unwrap_or_else(|e| apcd::apcrl_fatal_now!("surnames CSV record: {}", e));
        let name = record.get(name_idx)
            .unwrap_or_else(|| apcd::apcrl_fatal_now!("surnames CSV missing name field"));
        names.push(name.to_lowercase());
    }

    apcd::apcrl_info_now!("surnames: {} entries extracted", names.len());
    zapcad_sort_dedup(names)
}

// ---------------------------------------------------------------------------
// First names — SSA Baby Names (top 1000 by aggregate frequency)
// ---------------------------------------------------------------------------

fn zapcad_fetch_firstnames() -> Vec<String> {
    apcd::apcrl_info_now!("fetching first names from SSA Baby Names");
    let zip_bytes = zapcad_fetch_ssa_zip();

    let cursor = std::io::Cursor::new(&zip_bytes);
    let mut archive = zip::ZipArchive::new(cursor)
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("SSA zip open: {}", e));

    let mut counts: HashMap<String, u64> = HashMap::new();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .unwrap_or_else(|e| apcd::apcrl_fatal_now!("SSA zip entry {}: {}", i, e));
        let fname = file.name().to_string();
        if !fname.starts_with("yob") || !fname.ends_with(".txt") { continue; }

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .unwrap_or_else(|e| apcd::apcrl_fatal_now!("SSA read {}: {}", fname, e));

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 3 { continue; }
            let name = parts[0].to_lowercase();
            let count: u64 = parts[2].parse().unwrap_or(0);
            *counts.entry(name).or_insert(0) += count;
        }
    }

    let mut ranked: Vec<(String, u64)> = counts.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1));
    let names: Vec<String> = ranked.into_iter()
        .take(ZAPCAD_FIRSTNAME_LIMIT)
        .map(|(name, _)| name)
        .collect();

    apcd::apcrl_info_now!("firstnames: {} entries extracted", names.len());
    zapcad_sort_dedup(names)
}

fn zapcad_fetch_ssa_zip() -> Vec<u8> {
    match ureq::get(ZAPCAD_URL_SSA_NAMES).call() {
        Ok(resp) => {
            let mut bytes = Vec::new();
            resp.into_reader().read_to_end(&mut bytes)
                .unwrap_or_else(|e| apcd::apcrl_fatal_now!("SSA download read: {}", e));
            apcd::apcrl_info_now!("SSA download: {} bytes", bytes.len());
            bytes
        }
        Err(ureq::Error::Status(code, _)) => {
            apcd::apcrl_error_now!("SSA download returned HTTP {}", code);
            zapcad_load_ssa_staging()
        }
        Err(e) => {
            apcd::apcrl_error_now!("SSA download failed: {}", e);
            zapcad_load_ssa_staging()
        }
    }
}

fn zapcad_load_ssa_staging() -> Vec<u8> {
    let path = std::path::Path::new(ZAPCAD_SSA_STAGING);
    if path.exists() {
        apcd::apcrl_info_now!("loading SSA data from staging: {}", ZAPCAD_SSA_STAGING);
        std::fs::read(path)
            .unwrap_or_else(|e| apcd::apcrl_fatal_now!("staging read: {}", e))
    } else {
        apcd::apcrl_fatal_now!(
            "SSA blocked download (Akamai bot protection).\n\
             Manual workaround:\n\
             1. Open in browser: {}\n\
             2. Save the downloaded file to: {}\n\
             3. Re-run this tool",
            ZAPCAD_URL_SSA_NAMES, ZAPCAD_SSA_STAGING
        );
    }
}

// ---------------------------------------------------------------------------
// Cities — Census Population Estimates (top 100 by pop + 50 state capitals)
// ---------------------------------------------------------------------------

fn zapcad_fetch_cities() -> Vec<String> {
    apcd::apcrl_info_now!("fetching cities from Census population estimates");
    let csv_text = zapcad_download_text(ZAPCAD_URL_CITIES);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_text.as_bytes());

    let headers = rdr.headers()
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("cities CSV headers: {}", e))
        .clone();
    let sumlev_idx = headers.iter().position(|h| h == "SUMLEV")
        .unwrap_or_else(|| apcd::apcrl_fatal_now!("cities CSV missing SUMLEV"));
    let name_idx = headers.iter().position(|h| h == "NAME")
        .unwrap_or_else(|| apcd::apcrl_fatal_now!("cities CSV missing NAME"));
    let pop_idx = headers.iter().position(|h| h == "POPESTIMATE2023")
        .unwrap_or_else(|| apcd::apcrl_fatal_now!("cities CSV missing POPESTIMATE2023"));

    let mut places: Vec<(String, u64)> = Vec::new();
    for result in rdr.records() {
        let record = result
            .unwrap_or_else(|e| apcd::apcrl_fatal_now!("cities CSV record: {}", e));
        if record.get(sumlev_idx).unwrap_or("") != "162" { continue; }
        let raw_name = record.get(name_idx).unwrap_or("").to_string();
        let pop: u64 = record.get(pop_idx).unwrap_or("0").parse().unwrap_or(0);
        places.push((raw_name, pop));
    }

    places.sort_by(|a, b| b.1.cmp(&a.1));
    let top_cities: Vec<String> = places.into_iter()
        .take(ZAPCAD_CITY_POP_LIMIT)
        .map(|(name, _)| zapcad_strip_city_suffix(&name))
        .collect();

    let mut all: BTreeSet<String> = BTreeSet::new();
    for city in &top_cities {
        all.insert(city.clone());
    }
    for capital in ZAPCAD_STATE_CAPITALS {
        all.insert(capital.to_string());
    }

    let result: Vec<String> = all.into_iter().collect();
    apcd::apcrl_info_now!("cities: {} entries", result.len());
    result
}

fn zapcad_strip_city_suffix(name: &str) -> String {
    let trimmed = name.trim_matches('"').trim();
    let lower = trimmed.to_lowercase();
    for suffix in ZAPCAD_CITY_SUFFIXES {
        if let Some(stripped) = lower.strip_suffix(suffix) {
            return stripped.trim().to_string();
        }
    }
    lower
}

// ---------------------------------------------------------------------------
// English whitelist — Norvig/Google (top 5000)
// ---------------------------------------------------------------------------

fn zapcad_fetch_english() -> Vec<String> {
    apcd::apcrl_info_now!("fetching English whitelist from Norvig/Google n-grams");
    let text = zapcad_download_text(ZAPCAD_URL_ENGLISH);

    let mut words = Vec::new();
    for line in text.lines() {
        if words.len() >= ZAPCAD_ENGLISH_LIMIT { break; }
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.is_empty() { continue; }
        let word = parts[0].to_lowercase();
        if word.chars().any(|c| c.is_ascii_digit()) { continue; }
        if word.len() < 2 { continue; }
        words.push(word);
    }

    apcd::apcrl_info_now!("english whitelist: {} entries extracted", words.len());
    zapcad_sort_dedup(words)
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

fn zapcad_download_bytes(url: &str) -> Vec<u8> {
    apcd::apcrl_info_now!("downloading: {}", url);
    let resp = ureq::get(url).call()
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("download failed {}: {}", url, e));
    let mut bytes = Vec::new();
    resp.into_reader().read_to_end(&mut bytes)
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("download read {}: {}", url, e));
    apcd::apcrl_info_now!("downloaded {} bytes from {}", bytes.len(), url);
    bytes
}

fn zapcad_download_text(url: &str) -> String {
    apcd::apcrl_info_now!("downloading: {}", url);
    let resp = ureq::get(url).call()
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("download failed {}: {}", url, e));
    let text = resp.into_string()
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("download text {}: {}", url, e));
    apcd::apcrl_info_now!("downloaded {} bytes from {}", text.len(), url);
    text
}

fn zapcad_extract_zip_entry(zip_bytes: &[u8], entry_name: &str) -> String {
    let cursor = std::io::Cursor::new(zip_bytes);
    let mut archive = zip::ZipArchive::new(cursor)
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("zip open: {}", e));
    let mut file = archive.by_name(entry_name)
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("zip entry '{}': {}", entry_name, e));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("zip read '{}': {}", entry_name, e));
    contents
}

fn zapcad_sort_dedup(mut items: Vec<String>) -> Vec<String> {
    items.sort();
    items.dedup();
    items
}

fn zapcad_write_dict(filename: &str, entries: &[String]) {
    let path = std::path::Path::new(ZAPCAD_DICT_DIR).join(filename);
    let content = entries.join("\n") + "\n";
    std::fs::write(&path, &content)
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("write {}: {}", path.display(), e));
    apcd::apcrl_info_now!("wrote {} ({} entries, {} bytes)", filename, entries.len(), content.len());
}
