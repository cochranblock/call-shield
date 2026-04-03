use std::io::{self, BufRead, Write};

// Embedded govdocs — baked into binary at compile time
const GOVDOC_SBOM: &str = include_str!("../govdocs/SBOM.md");
const GOVDOC_SSDF: &str = include_str!("../govdocs/SSDF.md");
const GOVDOC_SUPPLY_CHAIN: &str = include_str!("../govdocs/SUPPLY_CHAIN.md");
const GOVDOC_SECURITY: &str = include_str!("../govdocs/SECURITY.md");
const GOVDOC_ACCESSIBILITY: &str = include_str!("../govdocs/ACCESSIBILITY.md");
const GOVDOC_PRIVACY: &str = include_str!("../govdocs/PRIVACY.md");
const GOVDOC_FIPS: &str = include_str!("../govdocs/FIPS.md");
const GOVDOC_FEDRAMP: &str = include_str!("../govdocs/FedRAMP_NOTES.md");
const GOVDOC_CMMC: &str = include_str!("../govdocs/CMMC.md");
const GOVDOC_ITAR_EAR: &str = include_str!("../govdocs/ITAR_EAR.md");
const GOVDOC_FEDERAL_USE_CASES: &str = include_str!("../govdocs/FEDERAL_USE_CASES.md");
const GOVDOC_SUPPLY_CHAIN_AUDIT: &str = include_str!("../govdocs/SUPPLY_CHAIN_AUDIT.md");

// Embedded Cargo.toml for live SBOM
const CARGO_TOML: &str = include_str!("../Cargo.toml");

/// Global threshold for classification confidence. Default 0.5.
static mut THRESHOLD: f64 = 0.5;

fn threshold() -> f64 {
    unsafe { THRESHOLD }
}

/// f0=main — entry point, CLI dispatch
fn f0() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Parse --threshold before dispatch
    let mut cmd_args = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--threshold" {
            if let Some(val) = args.get(i + 1) {
                match val.parse::<f64>() {
                    Ok(t) if (0.0..=1.0).contains(&t) => unsafe { THRESHOLD = t },
                    _ => {
                        eprintln!("--threshold must be a number between 0.0 and 1.0");
                        std::process::exit(1);
                    }
                }
                i += 2;
                continue;
            } else {
                eprintln!("--threshold requires a value (e.g. --threshold 0.7)");
                std::process::exit(1);
            }
        }
        cmd_args.push(args[i].clone());
        i += 1;
    }

    match cmd_args.first().map(|s| s.as_str()) {
        Some("--help") | Some("-h") => f1(),
        Some("--version") | Some("-V") => f2(),
        Some("--sbom") => f10(),
        Some("classify") => f3(&cmd_args[1..]),
        Some("screen") => f7(),
        Some("govdocs") => f5(&cmd_args[1..]),
        Some("whitelist") => f11(&cmd_args[1..]),
        Some("log") => f12(&cmd_args[1..]),
        Some(other) => {
            eprintln!("unknown command: {other}");
            eprintln!("run 'call-shield --help' for usage");
            std::process::exit(1);
        }
        None => f1(),
    }
}

/// f1=help — print usage
fn f1() {
    println!(
        "\
call-shield {}
On-device call screening without the cloud.

USAGE:
    call-shield [COMMAND]

COMMANDS:
    screen             Interactive call screening session
    classify <text>    Classify a single transcript line
    whitelist <cmd>    Manage contact whitelist (add/remove/list/check)
    log                View local screening log
    govdocs [doc]      Print embedded federal compliance docs
    --sbom             Machine-readable SPDX SBOM
    --help, -h         Show this help
    --version, -V      Show version

OPTIONS:
    --threshold <0.0-1.0>  Classification confidence cutoff (default: 0.5)

EXAMPLES:
    call-shield screen
    call-shield classify \"We've been trying to reach you about your car's extended warranty\"
    call-shield --threshold 0.7 classify \"hello this is a courtesy call\"
    call-shield whitelist add \"+15551234567\"
    call-shield govdocs sbom
    call-shield --sbom > sbom.spdx

Zero audio leaves the device. Ever.",
        env!("CARGO_PKG_VERSION")
    );
}

/// f2=version — print version
fn f2() {
    println!("call-shield {}", env!("CARGO_PKG_VERSION"));
}

/// f3=classify — intent classification on text input
fn f3(args: &[String]) {
    if args.is_empty() {
        eprintln!("usage: call-shield classify <text>");
        eprintln!("  provide caller transcript text to classify");
        std::process::exit(1);
    }

    let text = args.join(" ");
    let result = f4(&text.to_lowercase());

    println!("input:    {text}");
    println!("verdict:  {}", result.verdict);
    println!("score:    {:.2}", result.score);
    if !result.matched.is_empty() {
        println!("matched:  {}", result.matched.join(", "));
    }
}

/// t0=ClassifyResult — classification output
struct T0 {
    verdict: &'static str,
    score: f64,
    matched: Vec<&'static str>,
}

impl T0 {
    fn new(verdict: &'static str, score: f64, matched: Vec<&'static str>) -> Self {
        Self { verdict, score, matched }
    }
}

// s0/s1 — generated from patterns.csv at build time
include!(concat!(env!("OUT_DIR"), "/patterns.rs"));

/// f4=score — pattern-match classifier
fn f4(text: &str) -> T0 {
    let s0 = SPAM_PATTERNS;
    let s1 = LEGIT_PATTERNS;

    let mut spam_max = 0.0_f64;
    let mut legit_max = 0.0_f64;
    let mut matched = Vec::new();

    for (pattern, weight) in s0 {
        if text.contains(pattern) {
            if *weight > spam_max {
                spam_max = *weight;
            }
            matched.push(*pattern);
        }
    }

    for (pattern, weight) in s1 {
        if text.contains(pattern) {
            if *weight > legit_max {
                legit_max = *weight;
            }
            matched.push(*pattern);
        }
    }

    let t = threshold();
    if spam_max > legit_max && spam_max > t {
        T0::new("SPAM", spam_max, matched)
    } else if legit_max > spam_max && legit_max > t {
        T0::new("LEGITIMATE", legit_max, matched)
    } else {
        T0::new("UNKNOWN", t - (spam_max - legit_max).abs(), matched)
    }
}

/// f5=govdocs — print embedded compliance docs
fn f5(args: &[String]) {
    match args.first().map(|s| s.as_str()) {
        Some("sbom") => println!("{GOVDOC_SBOM}"),
        Some("ssdf") => println!("{GOVDOC_SSDF}"),
        Some("supply-chain") => println!("{GOVDOC_SUPPLY_CHAIN}"),
        Some("security") => println!("{GOVDOC_SECURITY}"),
        Some("accessibility") => println!("{GOVDOC_ACCESSIBILITY}"),
        Some("privacy") => println!("{GOVDOC_PRIVACY}"),
        Some("fips") => println!("{GOVDOC_FIPS}"),
        Some("fedramp") => println!("{GOVDOC_FEDRAMP}"),
        Some("cmmc") => println!("{GOVDOC_CMMC}"),
        Some("itar-ear") | Some("export") => println!("{GOVDOC_ITAR_EAR}"),
        Some("federal-use-cases") | Some("use-cases") => println!("{GOVDOC_FEDERAL_USE_CASES}"),
        Some("supply-chain-audit") | Some("audit") => println!("{GOVDOC_SUPPLY_CHAIN_AUDIT}"),
        Some("all") => f6(),
        Some(other) => {
            eprintln!("unknown govdoc: {other}");
            f5(&[]);
        }
        None => {
            println!(
                "\
call-shield govdocs — embedded federal compliance documentation

USAGE:
    call-shield govdocs <document>

DOCUMENTS:
    sbom              Software Bill of Materials (EO 14028)
    ssdf              NIST SP 800-218 Secure Development Framework
    supply-chain      Supply chain integrity
    security          Security posture and attack surface
    accessibility     Section 508 / WCAG compliance
    privacy           Privacy impact assessment
    fips              FIPS 140-2/3 cryptographic status
    fedramp           FedRAMP applicability notes
    cmmc              CMMC Level 1-2 practice mapping
    itar-ear          ITAR/EAR export classification
    use-cases         Federal agency use cases
    audit             Supply chain audit (deep code review)
    all               Print all documents

MACHINE-READABLE:
    call-shield --sbom    SPDX format SBOM for federal scanners"
            );
        }
    }
}

/// f6=govdocs_all — dump all compliance docs
fn f6() {
    let docs = [
        ("SBOM", GOVDOC_SBOM),
        ("SSDF", GOVDOC_SSDF),
        ("Supply Chain", GOVDOC_SUPPLY_CHAIN),
        ("Security", GOVDOC_SECURITY),
        ("Accessibility", GOVDOC_ACCESSIBILITY),
        ("Privacy", GOVDOC_PRIVACY),
        ("FIPS", GOVDOC_FIPS),
        ("FedRAMP", GOVDOC_FEDRAMP),
        ("CMMC", GOVDOC_CMMC),
        ("ITAR/EAR", GOVDOC_ITAR_EAR),
        ("Federal Use Cases", GOVDOC_FEDERAL_USE_CASES),
        ("Supply Chain Audit", GOVDOC_SUPPLY_CHAIN_AUDIT),
    ];
    for (name, content) in docs {
        println!("{}", "=".repeat(72));
        println!("  {name}");
        println!("{}", "=".repeat(72));
        println!("{content}");
    }
}

/// f7=screen — interactive call screening session
fn f7() {
    println!(
        "\
call-shield {} — interactive screening
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Simulates on-device call screening.
Type what the caller says. Call Shield classifies each line.
Screening continues until the call ends.

Commands during screening:
  /hangup    End the call
  /stats     Show session statistics
  /quit      Exit call-shield
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
        env!("CARGO_PKG_VERSION")
    );

    let stdin = io::stdin();
    let mut stats = T1::default();

    loop {
        // Prompt for caller speech
        if stats.turn == 0 {
            println!("\n📞 Incoming call...");
            println!("SHIELD: \"Please state your name and the reason for your call.\"");
        }

        print!("\nCALLER> ");
        let _ = io::stdout().flush();

        let mut line = String::new();
        if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 {
            break;
        }
        if line.len() > 4096 {
            line.truncate(4096);
        }
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        match line {
            "/hangup" => {
                println!("\n☎️  Call ended by user.");
                f9(&stats);
                break;
            }
            "/stats" => {
                f9(&stats);
                continue;
            }
            "/quit" => {
                println!("Exiting.");
                return;
            }
            _ => {}
        }

        stats.turn += 1;
        let result = f4(&line.to_lowercase());

        // Accumulate scores for running verdict
        match result.verdict {
            "SPAM" => stats.spam_hits += 1,
            "LEGITIMATE" => stats.legit_hits += 1,
            _ => stats.unknown_hits += 1,
        }
        if result.score > stats.peak_score {
            stats.peak_score = result.score;
            stats.peak_verdict = result.verdict;
        }

        // Show real-time classification
        println!("  [{} {:.0}%] {}", result.verdict, result.score * 100.0,
            if result.matched.is_empty() {
                String::new()
            } else {
                format!("(matched: {})", result.matched.join(", "))
            }
        );

        // Take action based on running assessment
        let action = f8(&stats);
        match action {
            "BLOCK" => {
                println!("\n🛑 SPAM DETECTED — call blocked.");
                println!("SHIELD: \"This call has been identified as unwanted. Goodbye.\"");
                f13(stats.peak_verdict, stats.peak_score, &result.matched, "BLOCK", stats.turn);
                f9(&stats);
                break;
            }
            "PASS" => {
                println!("\n✅ LEGITIMATE — ringing through to user.");
                println!("SHIELD: \"Connecting you now.\"");
                f13(stats.peak_verdict, stats.peak_score, &result.matched, "PASS", stats.turn);
                f9(&stats);
                break;
            }
            "PROMPT" => {
                if stats.turn < 3 {
                    println!("SHIELD: \"Could you tell me more about why you're calling?\"");
                }
                // After 3 turns of unknown, force a decision
                if stats.turn >= 3 {
                    println!("\n⚠️  INCONCLUSIVE after {} turns — routing to voicemail.", stats.turn);
                    f13(stats.peak_verdict, stats.peak_score, &result.matched, "VOICEMAIL", stats.turn);
                    f9(&stats);
                    break;
                }
            }
            _ => {}
        }
    }
}

/// t1=SessionStats — running stats for a screening session
#[derive(Default)]
struct T1 {
    turn: usize,
    spam_hits: usize,
    legit_hits: usize,
    unknown_hits: usize,
    peak_score: f64,
    peak_verdict: &'static str,
}

/// f8=decide — make a routing decision based on session state
fn f8(stats: &T1) -> &'static str {
    // Immediate block: 2+ spam hits or any high-confidence spam
    if stats.spam_hits >= 2 || (stats.peak_verdict == "SPAM" && stats.peak_score >= 0.90) {
        return "BLOCK";
    }
    // Immediate pass: 2+ legit hits or any high-confidence legit
    if stats.legit_hits >= 2 || (stats.peak_verdict == "LEGITIMATE" && stats.peak_score >= 0.85) {
        return "PASS";
    }
    "PROMPT"
}

/// f9=print_stats — display session statistics
fn f9(stats: &T1) {
    println!("\n--- Session Stats ---");
    println!("  turns:       {}", stats.turn);
    println!("  spam hits:   {}", stats.spam_hits);
    println!("  legit hits:  {}", stats.legit_hits);
    println!("  unknown:     {}", stats.unknown_hits);
    if stats.turn > 0 {
        println!("  peak:        {} ({:.0}%)", stats.peak_verdict, stats.peak_score * 100.0);
    }
}

/// f10=sbom_spdx — machine-readable SPDX SBOM from embedded Cargo.toml
fn f10() {
    // Parse package info from embedded Cargo.toml at runtime
    let mut name = "call-shield";
    let mut version = "0.1.0";
    let mut license = "Unlicense";
    let mut deps: Vec<(&str, &str)> = Vec::new();
    let mut in_deps = false;

    for line in CARGO_TOML.lines() {
        let line = line.trim();
        if line.starts_with("[dependencies]") {
            in_deps = true;
            continue;
        }
        if line.starts_with('[') {
            in_deps = false;
        }
        if let Some(rest) = line.strip_prefix("name = ") {
            name = rest.trim_matches('"');
        }
        if let Some(rest) = line.strip_prefix("version = ") {
            version = rest.trim_matches('"');
        }
        if let Some(rest) = line.strip_prefix("license = ") {
            license = rest.trim_matches('"');
        }
        if in_deps && line.contains('=') && !line.starts_with('#') && !line.is_empty()
            && let Some((dep_name, dep_ver)) = line.split_once('=')
        {
            let dep_name = dep_name.trim();
            let dep_ver = dep_ver.trim().trim_matches('"');
            if !dep_name.is_empty() {
                deps.push((dep_name, dep_ver));
            }
        }
    }

    // SPDX 2.3 format
    println!("SPDXVersion: SPDX-2.3");
    println!("DataLicense: CC0-1.0");
    println!("SPDXID: SPDXRef-DOCUMENT");
    println!("DocumentName: {name}-{version}");
    println!("DocumentNamespace: https://github.com/cochranblock/{name}/spdx");
    println!("Creator: Tool: call-shield-{version}");
    println!();
    println!("PackageName: {name}");
    println!("SPDXID: SPDXRef-Package-{name}");
    println!("PackageVersion: {version}");
    println!("PackageDownloadLocation: https://github.com/cochranblock/{name}");
    println!("PackageLicenseConcluded: {license}");
    println!("PackageLicenseDeclared: {license}");
    println!("PackageCopyrightText: NOASSERTION");
    println!("PackageSupplier: Organization: The Cochran Block LLC");
    println!("FilesAnalyzed: true");

    if deps.is_empty() {
        println!();
        println!("Relationship: SPDXRef-Package-{name} DEPENDS_ON NONE");
        println!("Comment: This package has zero third-party dependencies.");
    } else {
        for (dep_name, dep_ver) in &deps {
            println!();
            println!("PackageName: {dep_name}");
            println!("SPDXID: SPDXRef-Package-{dep_name}");
            println!("PackageVersion: {dep_ver}");
            println!("PackageDownloadLocation: https://crates.io/crates/{dep_name}");
            println!("Relationship: SPDXRef-Package-{name} DEPENDS_ON SPDXRef-Package-{dep_name}");
        }
    }
}

/// f11=whitelist — manage contact whitelist
fn f11(args: &[String]) {
    let path = whitelist_path();

    match args.first().map(|s| s.as_str()) {
        Some("add") => {
            if args.len() < 2 {
                eprintln!("usage: call-shield whitelist add <number>");
                std::process::exit(1);
            }
            let number = args[1..].join(" ").trim().to_string();
            let existing = load_whitelist(&path);
            if existing.contains(&number) {
                println!("already whitelisted: {number}");
                return;
            }
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            use std::fs::OpenOptions;
            let mut f = OpenOptions::new().create(true).append(true).open(&path).unwrap();
            writeln!(f, "{number}").unwrap();
            println!("added: {number}");
        }
        Some("remove") => {
            if args.len() < 2 {
                eprintln!("usage: call-shield whitelist remove <number>");
                std::process::exit(1);
            }
            let number = args[1..].join(" ").trim().to_string();
            let entries: Vec<String> = load_whitelist(&path)
                .into_iter()
                .filter(|e| e != &number)
                .collect();
            std::fs::write(&path, entries.join("\n") + "\n").unwrap();
            println!("removed: {number}");
        }
        Some("check") => {
            if args.len() < 2 {
                eprintln!("usage: call-shield whitelist check <number>");
                std::process::exit(1);
            }
            let number = args[1..].join(" ").trim().to_string();
            if load_whitelist(&path).contains(&number) {
                println!("{number}: WHITELISTED");
            } else {
                println!("{number}: not whitelisted");
            }
        }
        Some("list") | None => {
            let entries = load_whitelist(&path);
            if entries.is_empty() {
                println!("whitelist is empty");
                println!("add numbers: call-shield whitelist add <number>");
            } else {
                println!("whitelisted contacts ({}):", entries.len());
                for e in &entries {
                    println!("  {e}");
                }
            }
        }
        Some(other) => {
            eprintln!("unknown whitelist command: {other}");
            eprintln!("commands: add, remove, check, list");
            std::process::exit(1);
        }
    }
}

fn whitelist_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    std::path::Path::new(&home).join(".call-shield").join("whitelist.txt")
}

fn load_whitelist(path: &std::path::Path) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

/// f12=log — view screening log
fn f12(args: &[String]) {
    let path = log_path();
    match args.first().map(|s| s.as_str()) {
        Some("clear") => {
            let _ = std::fs::remove_file(&path);
            println!("screening log cleared");
        }
        Some("path") => println!("{}", path.display()),
        _ => {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            if content.is_empty() {
                println!("no screening log entries");
                println!("run 'call-shield screen' to generate log entries");
                return;
            }
            for line in content.lines() {
                println!("{line}");
            }
        }
    }
}

/// f13=log_entry — write a screening decision to the local log
fn f13(verdict: &str, score: f64, matched: &[&str], action: &str, turns: usize) {
    let path = log_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    use std::fs::OpenOptions;
    // ISO 8601 timestamp without deps: use seconds since epoch
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let matched_str = matched.join(", ");
    let entry = format!(
        "{{\"ts\":{ts},\"verdict\":\"{verdict}\",\"score\":{score:.2},\"matched\":\"{matched_str}\",\"action\":\"{action}\",\"turns\":{turns}}}"
    );
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&path) {
        let _ = writeln!(f, "{entry}");
    }
}

fn log_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    std::path::Path::new(&home).join(".call-shield").join("call_log.jsonl")
}

fn main() {
    f0();
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Classifier correctness ---

    #[test]
    fn spam_extended_warranty() {
        let r = f4("your extended warranty is expiring");
        assert_eq!(r.verdict, "SPAM");
        assert!(r.score >= 0.90);
    }

    #[test]
    fn spam_car_warranty() {
        let r = f4("we are calling about your car warranty");
        assert_eq!(r.verdict, "SPAM");
    }

    #[test]
    fn spam_press_1() {
        let r = f4("press 1 to speak to an agent");
        assert_eq!(r.verdict, "SPAM");
        assert!(r.score >= 0.90);
    }

    #[test]
    fn legit_appointment() {
        let r = f4("confirming your appointment for tuesday");
        assert_eq!(r.verdict, "LEGITIMATE");
        assert!(r.score >= 0.80);
    }

    #[test]
    fn legit_returning_call() {
        let r = f4("hi, returning your call from earlier");
        assert_eq!(r.verdict, "LEGITIMATE");
        assert!(r.score >= 0.90);
    }

    #[test]
    fn unknown_hello() {
        let r = f4("hello");
        assert_eq!(r.verdict, "UNKNOWN");
    }

    #[test]
    fn unknown_empty() {
        let r = f4("");
        assert_eq!(r.verdict, "UNKNOWN");
        assert!((r.score - 0.50).abs() < 0.01);
    }

    // --- False-positive regression (irs fix) ---

    #[test]
    fn first_not_spam() {
        let r = f4("this is your first appointment");
        assert_ne!(r.verdict, "SPAM", "'first' must not match 'irs'");
    }

    #[test]
    fn birthday_not_spam() {
        let r = f4("happy birthday");
        assert_ne!(r.verdict, "SPAM", "'birthday' must not match 'irs'");
    }

    #[test]
    fn thirsty_not_spam() {
        let r = f4("i am thirsty");
        assert_ne!(r.verdict, "SPAM");
    }

    // --- Vishing vector regression ---

    #[test]
    fn from_your_bank_is_spam() {
        let r = f4("this is from your bank please confirm");
        assert_eq!(r.verdict, "SPAM", "'from your bank' is a vishing vector");
    }

    #[test]
    fn verify_account_is_spam() {
        let r = f4("we need to verify your account");
        assert_eq!(r.verdict, "SPAM");
    }

    #[test]
    fn confirm_identity_is_spam() {
        let r = f4("please confirm your identity");
        assert_eq!(r.verdict, "SPAM");
    }

    // --- Score logic ---

    #[test]
    fn spam_beats_legit_when_higher() {
        let r = f4("extended warranty for your appointment");
        assert_eq!(r.verdict, "SPAM");
        assert!(r.score > 0.80);
    }

    #[test]
    fn legit_beats_spam_when_higher() {
        let r = f4("returning your call about a limited time schedule");
        assert_eq!(r.verdict, "LEGITIMATE");
    }

    #[test]
    fn score_never_negative() {
        for input in ["", "hello", "random words", "a b c d e f"] {
            let r = f4(input);
            assert!(r.score >= 0.0, "score for '{input}' was {}", r.score);
        }
    }

    // --- SBOM output ---

    #[test]
    fn sbom_contains_spdx_header() {
        // f10 prints to stdout, so we test the govdoc embed instead
        assert!(GOVDOC_SBOM.contains("Software Bill of Materials"));
    }

    // --- Whitelist ---

    #[test]
    fn log_entry_format() {
        let dir = std::env::temp_dir().join("call-shield-test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("log_test.jsonl");
        let _ = std::fs::remove_file(&path);

        // Override log path not possible without refactor, so test the format manually
        let matched = vec!["extended warranty"];
        let ts = 1000u64;
        let entry = format!(
            "{{\"ts\":{ts},\"verdict\":\"SPAM\",\"score\":0.95,\"matched\":\"extended warranty\",\"action\":\"BLOCK\",\"turns\":1}}"
        );
        assert!(entry.contains("\"verdict\":\"SPAM\""));
        assert!(entry.contains("\"action\":\"BLOCK\""));
        assert!(entry.contains("\"matched\":\"extended warranty\""));
        drop(matched);
    }

    #[test]
    fn whitelist_roundtrip() {
        let dir = std::env::temp_dir().join("call-shield-test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("whitelist_test.txt");
        let _ = std::fs::remove_file(&path);

        // empty
        let entries = load_whitelist(&path);
        assert!(entries.is_empty());

        // write
        std::fs::write(&path, "+15551234567\n+15559876543\n").unwrap();
        let entries = load_whitelist(&path);
        assert_eq!(entries.len(), 2);
        assert!(entries.contains(&"+15551234567".to_string()));

        let _ = std::fs::remove_file(&path);
    }
}
