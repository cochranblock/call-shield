/// f0=main — entry point, CLI dispatch
fn f0() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.first().map(|s| s.as_str()) {
        Some("--help") | Some("-h") => f1(),
        Some("--version") | Some("-V") => f2(),
        Some("classify") => f3(&args[1..]),
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
    classify <text>    Classify caller speech as spam, legitimate, or unknown
    --help, -h         Show this help
    --version, -V      Show version

EXAMPLES:
    call-shield classify \"We've been trying to reach you about your car's extended warranty\"
    call-shield classify \"Hi, this is Dr. Smith's office confirming your appointment tomorrow\"
    call-shield classify \"Hello?\"

Zero audio leaves the device. Ever.",
        env!("CARGO_PKG_VERSION")
    );
}

/// f2=version — print version
fn f2() {
    println!("call-shield {}", env!("CARGO_PKG_VERSION"));
}

/// f3=classify — intent classification on text input
/// Classifies transcript as: spam, legitimate, or unknown.
fn f3(args: &[String]) {
    if args.is_empty() {
        eprintln!("usage: call-shield classify <text>");
        eprintln!("  provide caller transcript text to classify");
        std::process::exit(1);
    }

    let text = args.join(" ").to_lowercase();
    let (t0, score) = f4(&text);

    println!("input:    {}", args.join(" "));
    println!("verdict:  {t0}");
    println!("score:    {score:.2}");
}

/// t0=Verdict — classification result
type _T0 = &'static str;

/// f4=score — pattern-match classifier
/// Returns (verdict, confidence score 0.0-1.0)
fn f4(text: &str) -> (&'static str, f64) {
    // s0=spam_patterns — phrases that indicate spam
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

    // s1=legit_patterns — phrases that indicate legitimate caller
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

    for (pattern, weight) in s0 {
        if text.contains(pattern) {
            spam_max = spam_max.max(*weight);
        }
    }

    for (pattern, weight) in s1 {
        if text.contains(pattern) {
            legit_max = legit_max.max(*weight);
        }
    }

    if spam_max > legit_max && spam_max > 0.5 {
        ("SPAM", spam_max)
    } else if legit_max > spam_max && legit_max > 0.5 {
        ("LEGITIMATE", legit_max)
    } else {
        ("UNKNOWN", 0.5 - (spam_max - legit_max).abs())
    }
}

fn main() {
    f0();
}
