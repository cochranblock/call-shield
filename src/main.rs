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

// Embedded Cargo.toml for live SBOM
const CARGO_TOML: &str = include_str!("../Cargo.toml");

/// f0=main — entry point, CLI dispatch
fn f0() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.first().map(|s| s.as_str()) {
        Some("--help") | Some("-h") => f1(),
        Some("--version") | Some("-V") => f2(),
        Some("--sbom") => f10(),
        Some("classify") => f3(&args[1..]),
        Some("screen") => f7(),
        Some("govdocs") => f5(&args[1..]),
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
    govdocs [doc]      Print embedded federal compliance docs
    --sbom             Machine-readable SPDX SBOM
    --help, -h         Show this help
    --version, -V      Show version

EXAMPLES:
    call-shield screen
    call-shield classify \"We've been trying to reach you about your car's extended warranty\"
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

/// f4=score — pattern-match classifier
fn f4(text: &str) -> T0 {
    // s0=spam_patterns
    let s0: &[(&str, f64)] = &[
        ("extended warranty", 0.95),
        ("car warranty", 0.95),
        ("been trying to reach you", 0.90),
        ("courtesy call", 0.85),
        ("special offer", 0.85),
        ("selected for", 0.80),
        ("press 1", 0.90),
        ("press one", 0.90),
        ("limited time", 0.80),
        ("act now", 0.80),
        ("free gift", 0.85),
        ("congratulations you", 0.85),
        ("you have won", 0.90),
        ("lower your rate", 0.85),
        ("reduce your debt", 0.85),
        ("irs", 0.75),
        ("social security number", 0.95),
        ("arrest warrant", 0.95),
        ("legal action", 0.80),
        ("final notice", 0.85),
    ];

    // s1=legit_patterns
    let s1: &[(&str, f64)] = &[
        ("appointment", 0.80),
        ("confirming your", 0.85),
        ("returning your call", 0.90),
        ("you called us", 0.85),
        ("this is dr", 0.80),
        ("this is doctor", 0.80),
        ("from your bank", 0.60),
        ("your order", 0.70),
        ("delivery", 0.70),
        ("picking up", 0.75),
        ("schedule", 0.70),
        ("follow up", 0.70),
        ("checking in", 0.65),
        ("your application", 0.65),
        ("interview", 0.80),
    ];

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

    if spam_max > legit_max && spam_max > 0.5 {
        T0::new("SPAM", spam_max, matched)
    } else if legit_max > spam_max && legit_max > 0.5 {
        T0::new("LEGITIMATE", legit_max, matched)
    } else {
        T0::new("UNKNOWN", 0.5 - (spam_max - legit_max).abs(), matched)
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
                f9(&stats);
                break;
            }
            "PASS" => {
                println!("\n✅ LEGITIMATE — ringing through to user.");
                println!("SHIELD: \"Connecting you now.\"");
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

fn main() {
    f0();
}
