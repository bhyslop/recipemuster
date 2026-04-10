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

//! Dictionary loading — surname, firstname, city blacklists and medical/english whitelists.

use aho_corasick::{AhoCorasick, MatchKind};
use std::collections::HashSet;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum apcrd_WordClass {
    Blacklisted,
    Whitelisted,
    Collision,
    Neutral,
}

pub struct apcrd_Dictionaries {
    automaton:     AhoCorasick,
    blacklist_set: HashSet<String>,
    whitelist:     HashSet<String>,
}

// ---------------------------------------------------------------------------
// Constants — embedded dictionary data
// ---------------------------------------------------------------------------

const ZAPCRD_SURNAMES:          &str = include_str!("../dictionaries/surnames.txt");
const ZAPCRD_FIRSTNAMES:        &str = include_str!("../dictionaries/firstnames.txt");
const ZAPCRD_CITIES:            &str = include_str!("../dictionaries/cities.txt");
const ZAPCRD_MEDICAL_WHITELIST: &str = include_str!("../dictionaries/medical_whitelist.txt");
const ZAPCRD_ENGLISH_WHITELIST: &str = include_str!("../dictionaries/english_whitelist.txt");

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl apcrd_Dictionaries {
    pub fn apcrd_load() -> Self {
        let surnames   = zapcrd_load_lines(ZAPCRD_SURNAMES);
        let firstnames = zapcrd_load_lines(ZAPCRD_FIRSTNAMES);
        let cities     = zapcrd_load_lines(ZAPCRD_CITIES);

        let mut all_entries = Vec::new();
        all_entries.extend(surnames);
        all_entries.extend(firstnames);
        all_entries.extend(cities);

        let blacklist_set: HashSet<String> = all_entries.into_iter().collect();
        let blacklist_vec: Vec<String> = blacklist_set.iter().cloned().collect();

        let automaton = AhoCorasick::builder()
            .match_kind(MatchKind::LeftmostLongest)
            .build(&blacklist_vec)
            .expect("aho-corasick build failed");

        let medical = zapcrd_load_lines(ZAPCRD_MEDICAL_WHITELIST);
        let english = zapcrd_load_lines(ZAPCRD_ENGLISH_WHITELIST);
        let mut whitelist = HashSet::new();
        whitelist.extend(medical);
        whitelist.extend(english);

        Self { automaton, blacklist_set, whitelist }
    }

    pub fn apcrd_is_blacklisted(&self, word: &str) -> bool {
        self.blacklist_set.contains(&word.to_lowercase())
    }

    pub fn apcrd_is_whitelisted(&self, word: &str) -> bool {
        self.whitelist.contains(&word.to_lowercase())
    }

    pub fn apcrd_classify_word(&self, word: &str) -> apcrd_WordClass {
        let black = self.apcrd_is_blacklisted(word);
        let white = self.apcrd_is_whitelisted(word);
        match (black, white) {
            (true, true)   => apcrd_WordClass::Collision,
            (true, false)  => apcrd_WordClass::Blacklisted,
            (false, true)  => apcrd_WordClass::Whitelisted,
            (false, false) => apcrd_WordClass::Neutral,
        }
    }

    pub fn apcrd_automaton(&self) -> &AhoCorasick {
        &self.automaton
    }
}

// ---------------------------------------------------------------------------
// Internal
// ---------------------------------------------------------------------------

fn zapcrd_load_lines(text: &str) -> Vec<String> {
    text.lines()
        .map(|l| l.trim().to_lowercase())
        .filter(|l| !l.is_empty())
        .collect()
}
