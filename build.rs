use std::env;
use std::fs;
use std::num::ParseIntError;
use std::path::PathBuf;

fn parse_score(txt: &str) -> Result<u16, ParseIntError> {
    let (full, sub) = txt.split_once(".").unwrap_or((txt, "0"));
    let full = full.parse::<u16>()?;
    let sub = sub.parse::<u16>()?;
    Ok((full * 100) + sub)
}

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let table = fs::read_to_string("table.wikitext").unwrap();

    let mut src = String::new();
    let mut ranking = Vec::new();

    let mut line_no = 0;
    let mut country = None;
    let mut regime_type = None;
    let mut scores = Vec::new();

    src += "/// Score data for each country\n";
    src += "pub mod countries {\n";
    src += "use super::{DemocracyQualities, RegimeType};\n";
    for line in table.lines() {
        let line = match line.strip_prefix("|") {
            Some("-" | "}") | None => {
                if let Some(country) = country.take() {
                    ranking.push(country);
                    let regime_type: &str = regime_type.expect("Missing regime type");

                    let mut scores = scores.iter();

                    src += "/// Democracy Index data for country code `";
                    src += country;
                    src += "`\n";

                    src += "pub const ";
                    src += country;
                    src += ": DemocracyQualities = DemocracyQualities {\n";

                    src += "    regime_type: RegimeType::";
                    src += regime_type;
                    src += ",\n";

                    for key in [
                        "overall_score",
                        "electoral_process_and_pluralism",
                        "functioning_of_government",
                        "political_participation",
                        "political_culture",
                        "civil_liberties",
                    ] {
                        src += &format!("    {key}: {},\n", scores.next().expect("overall_score"));
                    }
                    src += "};\n";
                }
                line_no = 0;
                regime_type = None;
                scores.clear();
                continue;
            }
            Some(line) => line,
        };

        match line_no {
            // This line we don't need
            0 => (),
            // Sometimes this line includes the next column
            1 => {
                // We sometimes care about this
                if let Some((_, line)) = line.split_once(r#"|| style="text-align:left;" |{{"#) {
                    country = Some(
                        line.strip_suffix("}}")
                            .expect("Missing expected country code prefix"),
                    );
                    line_no += 1;
                }
            }
            // This line contains the country code
            2 => {
                country = Some(
                    line.strip_prefix(r#" style="text-align:left;" |{{"#)
                        .expect("Missing expected country code prefix")
                        .strip_suffix("}}")
                        .expect("Missing country code suffix"),
                );
            }
            // This line contains rating and score
            3 => {
                let (rating, line) = line.split_once("||").expect("Rating line malformed");
                regime_type = Some(match rating {
                    "Full democracy" => "FullDemocracy",
                    "Flawed democracy" => "FlawedDemocracy",
                    "Hybrid regime" => "HybridRegime",
                    "Authoritarian regime" => "Authoritarian",
                    _ => panic!("Unknown regime type: {rating:?}"),
                });
                let (_line, score) = line
                    .split_once("|'''")
                    .expect("Missing overall score prefix");
                let score = score
                    .strip_suffix("}}")
                    .unwrap_or(score)
                    .strip_suffix("'''")
                    .expect("Missing overall score suffix");
                scores.push(parse_score(score).unwrap());
            }
            4 => {
                let chunks = line.split("||").skip(1);
                for chunk in chunks {
                    scores.push(parse_score(chunk).unwrap());
                }
            }
            _ => panic!("Unexpected line number in entry: {line_no}"),
        }

        line_no += 1;
    }
    src += "}\n";

    if let Some(country) = country {
        panic!("Table wasn't properly terminated: {country:?}");
    }

    src += "/// List of countries sorted by score\n";
    src += "pub const RANKING: &[(&str, DemocracyQualities)] = &[\n";
    for country in ranking {
        src += "(\"";
        src += country;
        src += "\", countries::";
        src += country;
        src += "),\n";
    }
    src += "];\n";

    fs::write(out.join("gen.rs"), src).unwrap();
    println!("cargo:rerun-if-changed=table.wikitext");
}
