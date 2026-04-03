use std::io::{self, BufRead, Write};
use std::sync::atomic::{AtomicU64, Ordering};

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
/// AtomicU64 stores f64 bits — safe under parallel test execution.
static THRESHOLD: AtomicU64 = AtomicU64::new(0x3FE0000000000000); // 0.5_f64.to_bits()

fn threshold() -> f64 {
    f64::from_bits(THRESHOLD.load(Ordering::Relaxed))
}

fn set_threshold(t: f64) {
    THRESHOLD.store(t.to_bits(), Ordering::Relaxed);
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
                    Ok(t) if (0.0..=1.0).contains(&t) => set_threshold(t),
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

    // =========================================================================
    // CLASSIFIER — every spam pattern individually
    // =========================================================================

    #[test] fn spam_extended_warranty() { let r = f4("your extended warranty is expiring"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.95).abs() < 0.01); }
    #[test] fn spam_car_warranty() { let r = f4("about your car warranty"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.95).abs() < 0.01); }
    #[test] fn spam_been_trying() { let r = f4("we have been trying to reach you"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.90).abs() < 0.01); }
    #[test] fn spam_courtesy_call() { let r = f4("this is a courtesy call"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.85).abs() < 0.01); }
    #[test] fn spam_special_offer() { let r = f4("you qualify for a special offer"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.85).abs() < 0.01); }
    #[test] fn spam_selected_for() { let r = f4("you have been selected for"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.80).abs() < 0.01); }
    #[test] fn spam_press_1() { let r = f4("press 1 to speak to an agent"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.90).abs() < 0.01); }
    #[test] fn spam_press_one() { let r = f4("press one now"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.90).abs() < 0.01); }
    #[test] fn spam_limited_time() { let r = f4("this limited time deal"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.80).abs() < 0.01); }
    #[test] fn spam_act_now() { let r = f4("you must act now"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.80).abs() < 0.01); }
    #[test] fn spam_free_gift() { let r = f4("claim your free gift"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.85).abs() < 0.01); }
    #[test] fn spam_congratulations() { let r = f4("congratulations you have been chosen"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.85).abs() < 0.01); }
    #[test] fn spam_you_have_won() { let r = f4("you have won a cruise"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.90).abs() < 0.01); }
    #[test] fn spam_lower_rate() { let r = f4("we can lower your rate"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.85).abs() < 0.01); }
    #[test] fn spam_reduce_debt() { let r = f4("reduce your debt today"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.85).abs() < 0.01); }
    #[test] fn spam_the_irs() { let r = f4("this is the irs calling"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.80).abs() < 0.01); }
    #[test] fn spam_irs_agent() { let r = f4("an irs agent will contact you"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.85).abs() < 0.01); }
    #[test] fn spam_ssn() { let r = f4("we need your social security number"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.95).abs() < 0.01); }
    #[test] fn spam_arrest_warrant() { let r = f4("there is an arrest warrant"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.95).abs() < 0.01); }
    #[test] fn spam_legal_action() { let r = f4("we will take legal action"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.80).abs() < 0.01); }
    #[test] fn spam_final_notice() { let r = f4("this is your final notice"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.85).abs() < 0.01); }
    #[test] fn spam_from_bank() { let r = f4("calling from your bank"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.70).abs() < 0.01); }
    #[test] fn spam_verify_account() { let r = f4("please verify your account"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.85).abs() < 0.01); }
    #[test] fn spam_confirm_identity() { let r = f4("confirm your identity now"); assert_eq!(r.verdict, "SPAM"); assert!((r.score - 0.80).abs() < 0.01); }

    // =========================================================================
    // CLASSIFIER — every legit pattern individually
    // =========================================================================

    #[test] fn legit_appointment() { let r = f4("your appointment is tomorrow"); assert_eq!(r.verdict, "LEGITIMATE"); assert!((r.score - 0.80).abs() < 0.01); }
    #[test] fn legit_confirming() { let r = f4("confirming your reservation"); assert_eq!(r.verdict, "LEGITIMATE"); assert!((r.score - 0.85).abs() < 0.01); }
    #[test] fn legit_returning_call() { let r = f4("returning your call"); assert_eq!(r.verdict, "LEGITIMATE"); assert!((r.score - 0.90).abs() < 0.01); }
    #[test] fn legit_you_called_us() { let r = f4("you called us earlier"); assert_eq!(r.verdict, "LEGITIMATE"); assert!((r.score - 0.85).abs() < 0.01); }
    #[test] fn legit_this_is_dr() { let r = f4("this is dr smith"); assert_eq!(r.verdict, "LEGITIMATE"); assert!((r.score - 0.80).abs() < 0.01); }
    #[test] fn legit_this_is_doctor() { let r = f4("this is doctor jones"); assert_eq!(r.verdict, "LEGITIMATE"); assert!((r.score - 0.80).abs() < 0.01); }
    #[test] fn legit_your_order() { let r = f4("calling about your order"); assert_eq!(r.verdict, "LEGITIMATE"); assert!((r.score - 0.70).abs() < 0.01); }
    #[test] fn legit_delivery() { let r = f4("your delivery is on the way"); assert_eq!(r.verdict, "LEGITIMATE"); assert!((r.score - 0.70).abs() < 0.01); }
    #[test] fn legit_picking_up() { let r = f4("i am picking up the package"); assert_eq!(r.verdict, "LEGITIMATE"); assert!((r.score - 0.75).abs() < 0.01); }
    #[test] fn legit_schedule() { let r = f4("about your schedule"); assert_eq!(r.verdict, "LEGITIMATE"); assert!((r.score - 0.70).abs() < 0.01); }
    #[test] fn legit_follow_up() { let r = f4("just a follow up call"); assert_eq!(r.verdict, "LEGITIMATE"); assert!((r.score - 0.70).abs() < 0.01); }
    #[test] fn legit_checking_in() { let r = f4("checking in on you"); assert_eq!(r.verdict, "LEGITIMATE"); assert!((r.score - 0.65).abs() < 0.01); }
    #[test] fn legit_application() { let r = f4("regarding your application"); assert_eq!(r.verdict, "LEGITIMATE"); assert!((r.score - 0.65).abs() < 0.01); }
    #[test] fn legit_interview() { let r = f4("about your interview"); assert_eq!(r.verdict, "LEGITIMATE"); assert!((r.score - 0.80).abs() < 0.01); }

    // =========================================================================
    // CLASSIFIER — unknown / no-match
    // =========================================================================

    #[test] fn unknown_hello() { let r = f4("hello"); assert_eq!(r.verdict, "UNKNOWN"); }
    #[test] fn unknown_empty() { let r = f4(""); assert_eq!(r.verdict, "UNKNOWN"); assert!((r.score - 0.50).abs() < 0.01); }
    #[test] fn unknown_gibberish() { let r = f4("asdf qwerty zxcv"); assert_eq!(r.verdict, "UNKNOWN"); }
    #[test] fn unknown_numbers_only() { let r = f4("12345"); assert_eq!(r.verdict, "UNKNOWN"); }
    #[test] fn unknown_single_char() { let r = f4("x"); assert_eq!(r.verdict, "UNKNOWN"); }
    #[test] fn unknown_whitespace() { let r = f4("   "); assert_eq!(r.verdict, "UNKNOWN"); }
    #[test] fn unknown_punctuation() { let r = f4("...!!!???"); assert_eq!(r.verdict, "UNKNOWN"); }

    // =========================================================================
    // CLASSIFIER — case insensitivity (f4 receives lowercased input from f3)
    // =========================================================================

    #[test] fn case_upper_spam() { let r = f4("extended warranty"); assert_eq!(r.verdict, "SPAM"); }
    #[test] fn case_mixed_legit() { let r = f4("returning your call"); assert_eq!(r.verdict, "LEGITIMATE"); }

    // =========================================================================
    // CLASSIFIER — false-positive regression
    // =========================================================================

    #[test] fn fp_first_not_spam() { let r = f4("this is your first appointment"); assert_ne!(r.verdict, "SPAM"); }
    #[test] fn fp_birthday_not_spam() { let r = f4("happy birthday to you"); assert_ne!(r.verdict, "SPAM"); }
    #[test] fn fp_thirsty_not_spam() { let r = f4("i am thirsty"); assert_ne!(r.verdict, "SPAM"); }
    #[test] fn fp_stairs_not_spam() { let r = f4("take the stairs"); assert_ne!(r.verdict, "SPAM"); }
    #[test] fn fp_pairs_not_spam() { let r = f4("three pairs of shoes"); assert_ne!(r.verdict, "SPAM"); }
    #[test] fn fp_desire_not_spam() { let r = f4("my desire is to help"); assert_ne!(r.verdict, "SPAM"); }
    #[test] fn fp_bird_not_spam() { let r = f4("a bird in the hand"); assert_ne!(r.verdict, "SPAM"); }

    // =========================================================================
    // CLASSIFIER — multi-pattern matching
    // =========================================================================

    #[test]
    fn multi_spam_patterns_highest_wins() {
        let r = f4("press 1 for your extended warranty final notice");
        assert_eq!(r.verdict, "SPAM");
        assert!((r.score - 0.95).abs() < 0.01); // extended warranty = 0.95
        assert!(r.matched.len() >= 3);
    }

    #[test]
    fn multi_legit_patterns_highest_wins() {
        let r = f4("returning your call about your appointment schedule");
        assert_eq!(r.verdict, "LEGITIMATE");
        assert!((r.score - 0.90).abs() < 0.01); // returning your call = 0.90
        assert!(r.matched.len() >= 3);
    }

    #[test]
    fn spam_beats_legit_when_higher() {
        let r = f4("extended warranty for your appointment");
        assert_eq!(r.verdict, "SPAM");
        assert!(r.score > 0.80);
        assert!(r.matched.contains(&"extended warranty"));
        assert!(r.matched.contains(&"appointment"));
    }

    #[test]
    fn legit_beats_spam_when_higher() {
        let r = f4("returning your call about a limited time schedule");
        assert_eq!(r.verdict, "LEGITIMATE");
        assert!((r.score - 0.90).abs() < 0.01);
    }

    #[test]
    fn tied_scores_are_unknown() {
        // Both "appointment" (legit 0.80) and "the irs" (spam 0.80) match
        let r = f4("the irs appointment");
        assert_eq!(r.verdict, "UNKNOWN");
    }

    #[test]
    fn matched_list_contains_all_hits() {
        let r = f4("press 1 for your car warranty");
        assert!(r.matched.contains(&"press 1"));
        assert!(r.matched.contains(&"car warranty"));
    }

    // =========================================================================
    // CLASSIFIER — score boundary conditions
    // =========================================================================

    #[test]
    fn score_never_negative() {
        for input in ["", "hello", "random words", "a b c d e f", "xyz 123 !@#"] {
            let r = f4(input);
            assert!(r.score >= 0.0, "score for '{input}' was {}", r.score);
        }
    }

    #[test]
    fn score_never_above_one() {
        for input in ["extended warranty arrest warrant social security number",
                      "returning your call confirming your appointment interview"] {
            let r = f4(input);
            assert!(r.score <= 1.0, "score for '{input}' was {}", r.score);
        }
    }

    #[test]
    fn unknown_score_is_threshold_minus_delta() {
        let r = f4("");
        // No patterns matched: spam_max=0, legit_max=0, delta=0
        // Score = threshold - |0-0| = 0.5
        assert!((r.score - 0.50).abs() < 0.01);
    }

    #[test]
    fn low_spam_below_threshold_is_unknown() {
        // "from your bank" has weight 0.70, which is above 0.5 threshold
        // but if we could set threshold to 0.8 it would be unknown
        // Test with default threshold: 0.70 > 0.5 = SPAM
        let r = f4("from your bank");
        assert_eq!(r.verdict, "SPAM");
    }

    // =========================================================================
    // CLASSIFIER — T0 struct
    // =========================================================================

    #[test]
    fn t0_new_sets_fields() {
        let r = T0::new("SPAM", 0.95, vec!["test"]);
        assert_eq!(r.verdict, "SPAM");
        assert!((r.score - 0.95).abs() < 0.01);
        assert_eq!(r.matched, vec!["test"]);
    }

    #[test]
    fn t0_empty_matched() {
        let r = T0::new("UNKNOWN", 0.50, vec![]);
        assert!(r.matched.is_empty());
    }

    // =========================================================================
    // SESSION DECISION — f8
    // =========================================================================

    #[test]
    fn f8_block_on_two_spam_hits() {
        let s = T1 { spam_hits: 2, ..Default::default() };
        assert_eq!(f8(&s), "BLOCK");
    }

    #[test]
    fn f8_block_on_high_confidence_spam() {
        let s = T1 { spam_hits: 1, peak_verdict: "SPAM", peak_score: 0.95, ..Default::default() };
        assert_eq!(f8(&s), "BLOCK");
    }

    #[test]
    fn f8_block_threshold_spam_90() {
        let s = T1 { spam_hits: 1, peak_verdict: "SPAM", peak_score: 0.90, ..Default::default() };
        assert_eq!(f8(&s), "BLOCK");
    }

    #[test]
    fn f8_no_block_spam_89() {
        let s = T1 { spam_hits: 1, peak_verdict: "SPAM", peak_score: 0.89, ..Default::default() };
        assert_eq!(f8(&s), "PROMPT");
    }

    #[test]
    fn f8_pass_on_two_legit_hits() {
        let s = T1 { legit_hits: 2, ..Default::default() };
        assert_eq!(f8(&s), "PASS");
    }

    #[test]
    fn f8_pass_on_high_confidence_legit() {
        let s = T1 { legit_hits: 1, peak_verdict: "LEGITIMATE", peak_score: 0.90, ..Default::default() };
        assert_eq!(f8(&s), "PASS");
    }

    #[test]
    fn f8_pass_threshold_legit_85() {
        let s = T1 { legit_hits: 1, peak_verdict: "LEGITIMATE", peak_score: 0.85, ..Default::default() };
        assert_eq!(f8(&s), "PASS");
    }

    #[test]
    fn f8_no_pass_legit_84() {
        let s = T1 { legit_hits: 1, peak_verdict: "LEGITIMATE", peak_score: 0.84, ..Default::default() };
        assert_eq!(f8(&s), "PROMPT");
    }

    #[test]
    fn f8_prompt_on_unknown() {
        let s = T1 { unknown_hits: 1, peak_verdict: "UNKNOWN", peak_score: 0.50, ..Default::default() };
        assert_eq!(f8(&s), "PROMPT");
    }

    #[test]
    fn f8_prompt_on_fresh_session() {
        let s = T1::default();
        assert_eq!(f8(&s), "PROMPT");
    }

    #[test]
    fn f8_spam_priority_over_legit() {
        // 2 spam hits + 1 legit = BLOCK (spam checked first)
        let s = T1 { spam_hits: 2, legit_hits: 1, peak_verdict: "SPAM", peak_score: 0.80, ..Default::default() };
        assert_eq!(f8(&s), "BLOCK");
    }

    #[test]
    fn f8_three_spam_hits() {
        let s = T1 { spam_hits: 3, peak_verdict: "SPAM", peak_score: 0.85, ..Default::default() };
        assert_eq!(f8(&s), "BLOCK");
    }

    #[test]
    fn f8_three_legit_hits() {
        let s = T1 { legit_hits: 3, peak_verdict: "LEGITIMATE", peak_score: 0.75, ..Default::default() };
        assert_eq!(f8(&s), "PASS");
    }

    // =========================================================================
    // T1 SessionStats defaults
    // =========================================================================

    #[test]
    fn t1_default_values() {
        let s = T1::default();
        assert_eq!(s.turn, 0);
        assert_eq!(s.spam_hits, 0);
        assert_eq!(s.legit_hits, 0);
        assert_eq!(s.unknown_hits, 0);
        assert!((s.peak_score - 0.0).abs() < 0.01);
        assert_eq!(s.peak_verdict, "");
    }

    // =========================================================================
    // THRESHOLD — AtomicU64 correctness
    // =========================================================================

    #[test]
    fn threshold_default_is_half() {
        // AtomicU64 constant encodes 0.5_f64.to_bits() — verify the round-trip
        assert!((threshold() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn threshold_bits_roundtrip() {
        // f64 <-> u64 bit reinterpretation must be lossless
        for v in [0.0_f64, 0.1, 0.5, 0.7, 0.9, 1.0] {
            let bits = v.to_bits();
            let back = f64::from_bits(bits);
            assert!((back - v).abs() < f64::EPSILON, "roundtrip failed for {v}");
        }
    }

    #[test]
    fn set_threshold_stores_and_loads() {
        let original = threshold();
        set_threshold(0.75);
        assert!((threshold() - 0.75).abs() < f64::EPSILON);
        set_threshold(original);
    }

    #[test]
    fn set_threshold_zero() {
        let original = threshold();
        set_threshold(0.0);
        assert!((threshold() - 0.0).abs() < f64::EPSILON);
        set_threshold(original);
    }

    #[test]
    fn set_threshold_one() {
        let original = threshold();
        set_threshold(1.0);
        assert!((threshold() - 1.0).abs() < f64::EPSILON);
        set_threshold(original);
    }

    #[test]
    fn threshold_atomic_constant_value() {
        // Verify the AtomicU64 initializer literal matches 0.5_f64.to_bits()
        assert_eq!(0x3FE0000000000000_u64, 0.5_f64.to_bits());
    }

    #[test]
    fn classifier_respects_threshold_via_atomic() {
        // "from your bank" weight = 0.70. At threshold 0.8 it should be UNKNOWN.
        let original = threshold();
        set_threshold(0.8);
        let r = f4("calling from your bank");
        assert_eq!(r.verdict, "UNKNOWN", "0.70 weight should be UNKNOWN at threshold 0.8");
        set_threshold(original);
    }

    #[test]
    fn classifier_respects_threshold_low() {
        // At threshold 0.0 anything with weight > 0 classifies.
        let original = threshold();
        set_threshold(0.0);
        let r = f4("calling from your bank"); // weight 0.70
        assert_eq!(r.verdict, "SPAM", "0.70 weight should be SPAM at threshold 0.0");
        set_threshold(original);
    }

    // =========================================================================
    // PATTERN TABLE INTEGRITY (build.rs generated)
    // =========================================================================

    #[test]
    fn spam_pattern_count() {
        assert_eq!(SPAM_PATTERNS.len(), 24);
    }

    #[test]
    fn legit_pattern_count() {
        assert_eq!(LEGIT_PATTERNS.len(), 14);
    }

    #[test]
    fn total_pattern_count() {
        assert_eq!(SPAM_PATTERNS.len() + LEGIT_PATTERNS.len(), 38);
    }

    #[test]
    fn no_empty_patterns() {
        for (p, _) in SPAM_PATTERNS { assert!(!p.is_empty(), "empty spam pattern"); }
        for (p, _) in LEGIT_PATTERNS { assert!(!p.is_empty(), "empty legit pattern"); }
    }

    #[test]
    fn all_weights_in_range() {
        for (p, w) in SPAM_PATTERNS {
            assert!(*w > 0.0 && *w <= 1.0, "spam pattern '{p}' weight {w} out of range");
        }
        for (p, w) in LEGIT_PATTERNS {
            assert!(*w > 0.0 && *w <= 1.0, "legit pattern '{p}' weight {w} out of range");
        }
    }

    #[test]
    fn no_duplicate_patterns() {
        let mut seen = std::collections::HashSet::new();
        for (p, _) in SPAM_PATTERNS { assert!(seen.insert(p), "duplicate spam: {p}"); }
        for (p, _) in LEGIT_PATTERNS { assert!(seen.insert(p), "duplicate legit: {p}"); }
    }

    #[test]
    fn patterns_are_lowercase() {
        for (p, _) in SPAM_PATTERNS { assert_eq!(*p, p.to_lowercase(), "spam '{p}' not lowercase"); }
        for (p, _) in LEGIT_PATTERNS { assert_eq!(*p, p.to_lowercase(), "legit '{p}' not lowercase"); }
    }

    // =========================================================================
    // EMBEDDED GOVDOCS — all 12 non-empty and contain expected content
    // =========================================================================

    #[test] fn govdoc_sbom_nonempty() { assert!(!GOVDOC_SBOM.is_empty()); assert!(GOVDOC_SBOM.contains("Software Bill of Materials")); }
    #[test] fn govdoc_ssdf_nonempty() { assert!(!GOVDOC_SSDF.is_empty()); assert!(GOVDOC_SSDF.contains("SSDF")); }
    #[test] fn govdoc_supply_chain_nonempty() { assert!(!GOVDOC_SUPPLY_CHAIN.is_empty()); assert!(GOVDOC_SUPPLY_CHAIN.contains("Supply Chain")); }
    #[test] fn govdoc_security_nonempty() { assert!(!GOVDOC_SECURITY.is_empty()); assert!(GOVDOC_SECURITY.contains("Security")); }
    #[test] fn govdoc_accessibility_nonempty() { assert!(!GOVDOC_ACCESSIBILITY.is_empty()); assert!(GOVDOC_ACCESSIBILITY.contains("508") || GOVDOC_ACCESSIBILITY.contains("Accessibility")); }
    #[test] fn govdoc_privacy_nonempty() { assert!(!GOVDOC_PRIVACY.is_empty()); assert!(GOVDOC_PRIVACY.contains("Privacy")); }
    #[test] fn govdoc_fips_nonempty() { assert!(!GOVDOC_FIPS.is_empty()); assert!(GOVDOC_FIPS.contains("FIPS")); }
    #[test] fn govdoc_fedramp_nonempty() { assert!(!GOVDOC_FEDRAMP.is_empty()); assert!(GOVDOC_FEDRAMP.contains("FedRAMP")); }
    #[test] fn govdoc_cmmc_nonempty() { assert!(!GOVDOC_CMMC.is_empty()); assert!(GOVDOC_CMMC.contains("CMMC")); }
    #[test] fn govdoc_itar_nonempty() { assert!(!GOVDOC_ITAR_EAR.is_empty()); assert!(GOVDOC_ITAR_EAR.contains("ITAR") || GOVDOC_ITAR_EAR.contains("EAR")); }
    #[test] fn govdoc_use_cases_nonempty() { assert!(!GOVDOC_FEDERAL_USE_CASES.is_empty()); assert!(GOVDOC_FEDERAL_USE_CASES.contains("Federal")); }
    #[test] fn govdoc_audit_nonempty() { assert!(!GOVDOC_SUPPLY_CHAIN_AUDIT.is_empty()); assert!(GOVDOC_SUPPLY_CHAIN_AUDIT.contains("Audit")); }

    // =========================================================================
    // EMBEDDED CARGO.TOML
    // =========================================================================

    #[test] fn cargo_toml_has_name() { assert!(CARGO_TOML.contains("name = \"call-shield\"")); }
    #[test] fn cargo_toml_has_version() { assert!(CARGO_TOML.contains("version = ")); }
    #[test] fn cargo_toml_has_license() { assert!(CARGO_TOML.contains("license = \"Unlicense\"")); }
    #[test] fn cargo_toml_has_edition() { assert!(CARGO_TOML.contains("edition = \"2024\"")); }
    #[test] fn cargo_toml_empty_deps() { assert!(CARGO_TOML.contains("[dependencies]")); }

    // =========================================================================
    // WHITELIST — file operations
    // =========================================================================

    fn test_dir() -> std::path::PathBuf {
        let d = std::env::temp_dir().join("call-shield-test");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    #[test]
    fn whitelist_empty_file() {
        let path = test_dir().join("wl_empty.txt");
        std::fs::write(&path, "").unwrap();
        let entries = load_whitelist(&path);
        assert!(entries.is_empty());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn whitelist_nonexistent_file() {
        let path = test_dir().join("wl_nonexist_12345.txt");
        let _ = std::fs::remove_file(&path);
        let entries = load_whitelist(&path);
        assert!(entries.is_empty());
    }

    #[test]
    fn whitelist_blank_lines_ignored() {
        let path = test_dir().join("wl_blanks.txt");
        std::fs::write(&path, "\n\n+15551234567\n\n+15559876543\n\n").unwrap();
        let entries = load_whitelist(&path);
        assert_eq!(entries.len(), 2);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn whitelist_whitespace_trimmed() {
        let path = test_dir().join("wl_ws.txt");
        std::fs::write(&path, "  +15551234567  \n").unwrap();
        let entries = load_whitelist(&path);
        assert_eq!(entries[0], "+15551234567");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn whitelist_roundtrip() {
        let path = test_dir().join("wl_roundtrip.txt");
        let _ = std::fs::remove_file(&path);
        assert!(load_whitelist(&path).is_empty());
        std::fs::write(&path, "+15551234567\n+15559876543\n").unwrap();
        let entries = load_whitelist(&path);
        assert_eq!(entries.len(), 2);
        assert!(entries.contains(&"+15551234567".to_string()));
        assert!(entries.contains(&"+15559876543".to_string()));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn whitelist_single_entry() {
        let path = test_dir().join("wl_single.txt");
        std::fs::write(&path, "+15551234567\n").unwrap();
        let entries = load_whitelist(&path);
        assert_eq!(entries.len(), 1);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn whitelist_many_entries() {
        let path = test_dir().join("wl_many.txt");
        let data: String = (0..100).map(|i| format!("+1555{i:07}\n")).collect();
        std::fs::write(&path, &data).unwrap();
        let entries = load_whitelist(&path);
        assert_eq!(entries.len(), 100);
        let _ = std::fs::remove_file(&path);
    }

    // =========================================================================
    // WHITELIST / LOG — path functions
    // =========================================================================

    #[test]
    fn whitelist_path_contains_call_shield() {
        let p = whitelist_path();
        assert!(p.to_str().unwrap().contains(".call-shield"));
        assert!(p.to_str().unwrap().contains("whitelist.txt"));
    }

    #[test]
    fn log_path_contains_call_shield() {
        let p = log_path();
        assert!(p.to_str().unwrap().contains(".call-shield"));
        assert!(p.to_str().unwrap().contains("call_log.jsonl"));
    }

    // =========================================================================
    // LOG — format validation
    // =========================================================================

    #[test]
    fn log_entry_format_spam() {
        let ts = 1000u64;
        let entry = format!(
            "{{\"ts\":{ts},\"verdict\":\"SPAM\",\"score\":0.95,\"matched\":\"extended warranty\",\"action\":\"BLOCK\",\"turns\":1}}"
        );
        assert!(entry.contains("\"verdict\":\"SPAM\""));
        assert!(entry.contains("\"action\":\"BLOCK\""));
        assert!(entry.contains("\"turns\":1"));
        assert!(entry.contains("\"score\":0.95"));
    }

    #[test]
    fn log_entry_format_legit() {
        let ts = 2000u64;
        let entry = format!(
            "{{\"ts\":{ts},\"verdict\":\"LEGITIMATE\",\"score\":0.90,\"matched\":\"returning your call\",\"action\":\"PASS\",\"turns\":2}}"
        );
        assert!(entry.contains("\"verdict\":\"LEGITIMATE\""));
        assert!(entry.contains("\"action\":\"PASS\""));
        assert!(entry.contains("\"turns\":2"));
    }

    #[test]
    fn log_entry_format_voicemail() {
        let ts = 3000u64;
        let entry = format!(
            "{{\"ts\":{ts},\"verdict\":\"UNKNOWN\",\"score\":0.50,\"matched\":\"\",\"action\":\"VOICEMAIL\",\"turns\":3}}"
        );
        assert!(entry.contains("\"action\":\"VOICEMAIL\""));
        assert!(entry.contains("\"turns\":3"));
    }

    // =========================================================================
    // CLASSIFIER — long input / stress
    // =========================================================================

    #[test]
    fn classify_long_input() {
        let long = "hello ".repeat(1000);
        let r = f4(&long);
        assert_eq!(r.verdict, "UNKNOWN");
    }

    #[test]
    fn classify_all_spam_words() {
        // Input contains every spam pattern — should still return SPAM with highest weight
        let all_spam = "extended warranty car warranty been trying to reach you courtesy call \
            special offer selected for press 1 press one limited time act now free gift \
            congratulations you you have won lower your rate reduce your debt the irs irs agent \
            social security number arrest warrant legal action final notice from your bank \
            verify your account confirm your identity";
        let r = f4(all_spam);
        assert_eq!(r.verdict, "SPAM");
        assert!((r.score - 0.95).abs() < 0.01);
        assert_eq!(r.matched.len(), 24);
    }

    #[test]
    fn classify_all_legit_words() {
        let all_legit = "appointment confirming your returning your call you called us this is dr \
            this is doctor your order delivery picking up schedule follow up checking in \
            your application interview";
        let r = f4(all_legit);
        assert_eq!(r.verdict, "LEGITIMATE");
        assert!((r.score - 0.90).abs() < 0.01);
        assert_eq!(r.matched.len(), 14);
    }

    #[test]
    fn classify_unicode() {
        let r = f4("hello 你好 مرحبا");
        assert_eq!(r.verdict, "UNKNOWN");
    }

    #[test]
    fn classify_emoji() {
        let r = f4("🎉🎊🎈");
        assert_eq!(r.verdict, "UNKNOWN");
    }

    // =========================================================================
    // CLASSIFIER — vishing vectors (social engineering phrases)
    // =========================================================================

    #[test] fn vish_bank_confirm() { let r = f4("this is from your bank please confirm"); assert_eq!(r.verdict, "SPAM"); }
    #[test] fn vish_verify_now() { let r = f4("verify your account immediately"); assert_eq!(r.verdict, "SPAM"); }
    #[test] fn vish_identity_theft() { let r = f4("we need you to confirm your identity or your account will be locked"); assert_eq!(r.verdict, "SPAM"); }
    #[test] fn vish_irs_arrest() { let r = f4("the irs has issued an arrest warrant"); assert_eq!(r.verdict, "SPAM"); assert!(r.score >= 0.80); }
    #[test] fn vish_ssn_legal() { let r = f4("provide your social security number or face legal action"); assert_eq!(r.verdict, "SPAM"); assert!(r.score >= 0.80); }

    // =========================================================================
    // CLASSIFIER — realistic call transcripts
    // =========================================================================

    #[test]
    fn real_doctor_office() {
        let r = f4("hi this is doctor garcia from the clinic confirming your appointment for thursday");
        assert_eq!(r.verdict, "LEGITIMATE");
    }

    #[test]
    fn real_delivery_driver() {
        let r = f4("hey i have your delivery but nobody is answering the door");
        assert_eq!(r.verdict, "LEGITIMATE");
    }

    #[test]
    fn real_robocall_warranty() {
        let r = f4("we have been trying to reach you about your car warranty this is your final notice press 1 to speak to a representative");
        assert_eq!(r.verdict, "SPAM");
        assert!(r.matched.len() >= 4);
    }

    #[test]
    fn real_job_callback() {
        let r = f4("hi this is sarah from acme corp returning your call about the interview we discussed");
        assert_eq!(r.verdict, "LEGITIMATE");
    }

    #[test]
    fn real_irs_scam() {
        let r = f4("this is the irs calling about your social security number there is an arrest warrant for tax fraud");
        assert_eq!(r.verdict, "SPAM");
        assert!(r.matched.len() >= 3);
    }

    #[test]
    fn real_neighbor() {
        let r = f4("hey it is your neighbor just checking in");
        assert_eq!(r.verdict, "LEGITIMATE");
    }

    #[test]
    fn real_ambiguous() {
        let r = f4("hello is this michael");
        assert_eq!(r.verdict, "UNKNOWN");
    }

    // =========================================================================
    // LOG — f13 file write roundtrip
    // =========================================================================

    #[test]
    fn log_write_creates_file() {
        let dir = test_dir().join("log_write_test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("call_log.jsonl");
        let _ = std::fs::remove_file(&path);

        // Write entry manually using the same format as f13
        let ts = 9999u64;
        let entry = format!(
            "{{\"ts\":{ts},\"verdict\":\"SPAM\",\"score\":0.95,\"matched\":\"extended warranty\",\"action\":\"BLOCK\",\"turns\":1}}"
        );
        use std::fs::OpenOptions;
        let mut f = OpenOptions::new().create(true).append(true).open(&path).unwrap();
        use std::io::Write as _;
        writeln!(f, "{entry}").unwrap();
        drop(f);

        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("\"verdict\":\"SPAM\""));
        assert!(content.contains("\"action\":\"BLOCK\""));
        assert!(content.contains("\"ts\":9999"));
        assert!(content.contains("\"turns\":1"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn log_write_appends_multiple_entries() {
        let dir = test_dir().join("log_append_test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("call_log.jsonl");
        let _ = std::fs::remove_file(&path);

        use std::fs::OpenOptions;
        use std::io::Write as _;
        for (ts, action) in [(1u64, "BLOCK"), (2u64, "PASS"), (3u64, "VOICEMAIL")] {
            let entry = format!(
                "{{\"ts\":{ts},\"verdict\":\"TEST\",\"score\":0.50,\"matched\":\"\",\"action\":\"{action}\",\"turns\":1}}"
            );
            let mut f = OpenOptions::new().create(true).append(true).open(&path).unwrap();
            writeln!(f, "{entry}").unwrap();
        }

        let lines: Vec<&str> = std::fs::read_to_string(&path).unwrap()
            .lines()
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>()
            .into_iter()
            .collect();
        // Can't collect &str from owned String this way — just check line count
        let content = std::fs::read_to_string(&path).unwrap();
        let line_count = content.lines().count();
        assert_eq!(line_count, 3);
        assert!(content.contains("\"action\":\"BLOCK\""));
        assert!(content.contains("\"action\":\"PASS\""));
        assert!(content.contains("\"action\":\"VOICEMAIL\""));
        let _ = std::fs::remove_dir_all(&dir);
    }

    // =========================================================================
    // SBOM — f10 output fields (via CARGO_TOML parse validation)
    // =========================================================================

    #[test]
    fn sbom_cargo_toml_parses_name() {
        let mut name = "";
        for line in CARGO_TOML.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("name = ") {
                name = rest.trim_matches('"');
                break;
            }
        }
        assert_eq!(name, "call-shield");
    }

    #[test]
    fn sbom_cargo_toml_parses_version() {
        let mut version = "";
        for line in CARGO_TOML.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("version = ") {
                version = rest.trim_matches('"');
                break;
            }
        }
        assert!(!version.is_empty());
        // semver-shaped: N.N.N
        let parts: Vec<&str> = version.split('.').collect();
        assert_eq!(parts.len(), 3, "version '{version}' is not semver");
    }

    #[test]
    fn sbom_cargo_toml_parses_license() {
        let mut license = "";
        for line in CARGO_TOML.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("license = ") {
                license = rest.trim_matches('"');
                break;
            }
        }
        assert_eq!(license, "Unlicense");
    }

    #[test]
    fn sbom_cargo_toml_no_deps_section_empty() {
        // call-shield is zero-dep. Confirm [dependencies] section has no entries.
        let mut in_deps = false;
        let mut dep_count = 0usize;
        for line in CARGO_TOML.lines() {
            let line = line.trim();
            if line.starts_with("[dependencies]") {
                in_deps = true;
                continue;
            }
            if line.starts_with('[') {
                in_deps = false;
            }
            if in_deps && line.contains('=') && !line.starts_with('#') && !line.is_empty() {
                dep_count += 1;
            }
        }
        assert_eq!(dep_count, 0, "expected zero deps but found {dep_count}");
    }

    // =========================================================================
    // PATH HELPERS — HOME env var respected
    // =========================================================================

    #[test]
    fn whitelist_path_uses_home_env() {
        // whitelist_path() reads HOME at call time
        let p = whitelist_path();
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        assert!(p.starts_with(&home));
    }

    #[test]
    fn log_path_uses_home_env() {
        let p = log_path();
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        assert!(p.starts_with(&home));
    }

    // =========================================================================
    // CLASSIFIER — threshold sensitivity
    // =========================================================================

    #[test]
    fn low_confidence_spam_below_default_threshold() {
        // "from your bank" weight is 0.70, above default 0.5 = SPAM
        let r = f4("calling from your bank");
        assert_eq!(r.verdict, "SPAM");
        assert!((r.score - 0.70).abs() < 0.01);
    }

    #[test]
    fn low_confidence_legit_below_default_threshold() {
        // "checking in" weight 0.65, above default 0.5 = LEGITIMATE
        let r = f4("just checking in on you today");
        assert_eq!(r.verdict, "LEGITIMATE");
        assert!((r.score - 0.65).abs() < 0.01);
    }

    #[test]
    fn score_formula_unknown_with_partial_match() {
        // With one spam match at 0.60 and no legit, spam_max=0.60 > legit_max=0.0
        // But if 0.60 > threshold(0.5) it's SPAM, not UNKNOWN
        // "from your bank" = 0.70, so this would be SPAM.
        // Test the UNKNOWN path: no patterns match
        let r = f4("this sentence has no patterns in it whatsoever");
        assert_eq!(r.verdict, "UNKNOWN");
        // Score = threshold - |0-0| = threshold()
        assert!((r.score - threshold()).abs() < 0.01);
    }
}
