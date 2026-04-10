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

//! Tests for dictionary loading.

#[cfg(test)]
mod tests {
    use crate::apcrd_dictionaries::*;

    #[test]
    fn test_dictionaries_load() {
        let d = apcrd_Dictionaries::apcrd_load();
        // Verify some known entries are present
        assert!(d.apcrd_is_blacklisted("smith"));
        assert!(d.apcrd_is_blacklisted("johnson"));
        assert!(d.apcrd_is_whitelisted("the"));
        assert!(d.apcrd_is_whitelisted("aspirin"));
    }

    #[test]
    fn test_blacklist_case_insensitive() {
        let d = apcrd_Dictionaries::apcrd_load();
        assert!(d.apcrd_is_blacklisted("Smith"));
        assert!(d.apcrd_is_blacklisted("SMITH"));
        assert!(d.apcrd_is_blacklisted("smith"));
        assert!(d.apcrd_is_blacklisted("sMiTh"));
    }

    #[test]
    fn test_whitelist_case_insensitive() {
        let d = apcrd_Dictionaries::apcrd_load();
        assert!(d.apcrd_is_whitelisted("Aspirin"));
        assert!(d.apcrd_is_whitelisted("ASPIRIN"));
        assert!(d.apcrd_is_whitelisted("aspirin"));
    }

    #[test]
    fn test_classify_blacklisted_only() {
        let d = apcrd_Dictionaries::apcrd_load();
        // "thornton" is a surname, unlikely in whitelist
        assert_eq!(d.apcrd_classify_word("thornton"), apcrd_WordClass::Blacklisted);
    }

    #[test]
    fn test_classify_whitelisted_only() {
        let d = apcrd_Dictionaries::apcrd_load();
        // "hypertension" is a medical term, not a name
        assert_eq!(d.apcrd_classify_word("hypertension"), apcrd_WordClass::Whitelisted);
    }

    #[test]
    fn test_classify_neutral() {
        let d = apcrd_Dictionaries::apcrd_load();
        assert_eq!(d.apcrd_classify_word("xyzzyplugh99"), apcrd_WordClass::Neutral);
    }

    #[test]
    fn test_classify_collision_exists() {
        let d = apcrd_Dictionaries::apcrd_load();
        // Common surnames that are also English words: young, long, white, king, brown
        let known_dual = ["young", "long", "white", "king", "brown"];
        let found = known_dual.iter().any(|w| {
            d.apcrd_classify_word(w) == apcrd_WordClass::Collision
        });
        assert!(found, "Expected at least one collision among known dual-use words");
    }
}
