use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=patterns.csv");

    let csv = BufReader::new(fs::File::open("patterns.csv").expect("patterns.csv not found"));
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("patterns.rs");
    let mut out = fs::File::create(&dest).unwrap();

    let mut spam = Vec::new();
    let mut legit = Vec::new();

    for line in csv.lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() || line.starts_with("category") {
            continue;
        }
        let mut parts = line.splitn(3, ',');
        let category = parts.next().unwrap().trim();
        let pattern = parts.next().unwrap().trim();
        let weight = parts.next().unwrap().trim();

        match category {
            "spam" => spam.push((pattern.to_string(), weight.to_string())),
            "legit" => legit.push((pattern.to_string(), weight.to_string())),
            other => panic!("unknown category in patterns.csv: {other}"),
        }
    }

    writeln!(out, "// Generated from patterns.csv — do not edit").unwrap();
    writeln!(out, "const SPAM_PATTERNS: &[(&str, f64)] = &[").unwrap();
    for (p, w) in &spam {
        writeln!(out, "    (\"{p}\", {w}),").unwrap();
    }
    writeln!(out, "];").unwrap();

    writeln!(out, "const LEGIT_PATTERNS: &[(&str, f64)] = &[").unwrap();
    for (p, w) in &legit {
        writeln!(out, "    (\"{p}\", {w}),").unwrap();
    }
    writeln!(out, "];").unwrap();
}
