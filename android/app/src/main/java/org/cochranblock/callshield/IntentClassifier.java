package org.cochranblock.callshield;

/**
 * f4=IntentClassifier — on-device pattern-match caller intent classifier.
 * Mirrors the Rust CLI classifier. Zero cloud. Zero network.
 *
 * Future: replace pattern matching with embedded Whisper + ML model.
 */
public final class IntentClassifier {

    public enum Verdict { SPAM, LEGITIMATE, UNKNOWN }

    public static final class Result {
        public final Verdict verdict;
        public final double score;
        public final String matched;

        Result(Verdict verdict, double score, String matched) {
            this.verdict = verdict;
            this.score = score;
            this.matched = matched;
        }
    }

    // s0=spam patterns
    private static final String[][] SPAM = {
        {"extended warranty", "0.95"},
        {"car warranty", "0.95"},
        {"been trying to reach you", "0.90"},
        {"courtesy call", "0.85"},
        {"special offer", "0.85"},
        {"selected for", "0.80"},
        {"press 1", "0.90"},
        {"press one", "0.90"},
        {"limited time", "0.80"},
        {"act now", "0.80"},
        {"free gift", "0.85"},
        {"congratulations you", "0.85"},
        {"you have won", "0.90"},
        {"lower your rate", "0.85"},
        {"reduce your debt", "0.85"},
        {"the irs", "0.80"},
        {"irs agent", "0.85"},
        {"social security number", "0.95"},
        {"arrest warrant", "0.95"},
        {"legal action", "0.80"},
        {"final notice", "0.85"},
        {"from your bank", "0.70"},
        {"verify your account", "0.85"},
        {"confirm your identity", "0.80"},
    };

    // s1=legit patterns
    private static final String[][] LEGIT = {
        {"appointment", "0.80"},
        {"confirming your", "0.85"},
        {"returning your call", "0.90"},
        {"you called us", "0.85"},
        {"this is dr", "0.80"},
        {"this is doctor", "0.80"},
        {"your order", "0.70"},
        {"delivery", "0.70"},
        {"picking up", "0.75"},
        {"schedule", "0.70"},
        {"follow up", "0.70"},
        {"checking in", "0.65"},
        {"your application", "0.65"},
        {"interview", "0.80"},
    };

    /** Classify a transcript string. All processing on-device. */
    public static Result classify(String transcript) {
        String text = transcript.toLowerCase();
        double spamMax = 0.0;
        double legitMax = 0.0;
        StringBuilder matched = new StringBuilder();

        for (String[] entry : SPAM) {
            if (text.contains(entry[0])) {
                double w = Double.parseDouble(entry[1]);
                if (w > spamMax) spamMax = w;
                if (matched.length() > 0) matched.append(", ");
                matched.append(entry[0]);
            }
        }

        for (String[] entry : LEGIT) {
            if (text.contains(entry[0])) {
                double w = Double.parseDouble(entry[1]);
                if (w > legitMax) legitMax = w;
                if (matched.length() > 0) matched.append(", ");
                matched.append(entry[0]);
            }
        }

        if (spamMax > legitMax && spamMax > 0.5) {
            return new Result(Verdict.SPAM, spamMax, matched.toString());
        } else if (legitMax > spamMax && legitMax > 0.5) {
            return new Result(Verdict.LEGITIMATE, legitMax, matched.toString());
        } else {
            return new Result(Verdict.UNKNOWN,
                0.5 - Math.abs(spamMax - legitMax), matched.toString());
        }
    }
}
