//! The Democracy Index published by the Economist Group is an index measuring the quality of democracy across the world.

#![no_std]

use core::fmt;

/// The year the data is based on
pub const REPORT_YEAR: u16 = 2024;

/// Category based on score
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegimeType {
    /// Political pluralism is nonexistent or severely limited
    Authoritarian,
    /// Regular electoral fraud, preventing them from being fair and free democracies
    HybridRegime,
    /// Elections are fair and free and basic civil liberties are honoured, but may have issues
    FlawedDemocracy,
    /// Civil liberties and fundamental political freedoms are not only respected but also reinforced by a political culture conducive to the thriving of democratic principles
    FullDemocracy,
}

impl RegimeType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Authoritarian => "Authoritarian regime",
            Self::HybridRegime => "Hybrid regime",
            Self::FlawedDemocracy => "Flawed democracy",
            Self::FullDemocracy => "Full democracy",
        }
    }
}

impl fmt::Display for RegimeType {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "{}", self.as_str())
    }
}

/// The various scores for a country
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemocracyQualities {
    pub overall_score: u16,
    pub regime_type: RegimeType,
    pub electoral_process_and_pluralism: u16,
    pub functioning_of_government: u16,
    pub political_participation: u16,
    pub political_culture: u16,
    pub civil_liberties: u16,
}

include!(concat!(env!("OUT_DIR"), "/gen.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_entries() {
        assert_eq!(RANKING.len(), 167);
    }

    #[test]
    fn test_sort_regime_types() {
        let list = &mut [
            RegimeType::HybridRegime,
            RegimeType::FlawedDemocracy,
            RegimeType::Authoritarian,
            RegimeType::FullDemocracy,
        ];
        list.sort();
        assert_eq!(
            list,
            &[
                RegimeType::Authoritarian,
                RegimeType::HybridRegime,
                RegimeType::FlawedDemocracy,
                RegimeType::FullDemocracy,
            ]
        );
        assert!(RegimeType::FullDemocracy > RegimeType::Authoritarian);
    }

    #[test]
    fn test_sweden() {
        assert_eq!(
            countries::SWE,
            DemocracyQualities {
                overall_score: 939,
                regime_type: RegimeType::FullDemocracy,
                electoral_process_and_pluralism: 958,
                functioning_of_government: 964,
                political_participation: 833,
                political_culture: 1000,
                civil_liberties: 941,
            }
        );
        assert_eq!(get("SWE"), Some(&countries::SWE));
    }

    #[test]
    fn test_nepal() {
        assert_eq!(
            countries::NPL,
            DemocracyQualities {
                overall_score: 460,
                regime_type: RegimeType::HybridRegime,
                electoral_process_and_pluralism: 483,
                functioning_of_government: 536,
                political_participation: 500,
                political_culture: 250,
                civil_liberties: 529,
            }
        );
        assert_eq!(get("NPL"), Some(&countries::NPL));
    }

    #[test]
    fn test_cuba() {
        assert_eq!(
            countries::CUB,
            DemocracyQualities {
                overall_score: 258,
                regime_type: RegimeType::Authoritarian,
                electoral_process_and_pluralism: 0,
                functioning_of_government: 286,
                political_participation: 333,
                political_culture: 375,
                civil_liberties: 294,
            }
        );
        assert_eq!(get("CUB"), Some(&countries::CUB));
    }

    #[test]
    fn test_north_korea() {
        assert_eq!(
            countries::PRK,
            DemocracyQualities {
                overall_score: 108,
                regime_type: RegimeType::Authoritarian,
                electoral_process_and_pluralism: 0,
                functioning_of_government: 250,
                political_participation: 167,
                political_culture: 125,
                civil_liberties: 0,
            }
        );
        assert_eq!(get("PRK"), Some(&countries::PRK));
    }

    #[test]
    fn test_get_invalid() {
        assert_eq!(get("X"), None);
        assert_eq!(get(""), None);
    }
}
