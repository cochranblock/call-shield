package org.cochranblock.callshield;

import org.junit.Test;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotEquals;
import static org.junit.Assert.assertTrue;
import static org.junit.Assert.assertFalse;

/**
 * IntentClassifierTest — JVM unit tests for the on-device classifier.
 *
 * Mirrors src/main.rs#tests so the Java port stays bit-for-bit equivalent
 * with the Rust reference implementation. Pure JVM — no Android runtime,
 * no emulator, no instrumented test infrastructure.
 *
 * Run: ./gradlew :app:testDebugUnitTest
 */
public class IntentClassifierTest {

    private static final double EPS = 0.01;

    private static IntentClassifier.Result classify(String s) {
        return IntentClassifier.classify(s);
    }

    // =========================================================================
    // SPAM — every pattern individually
    // =========================================================================

    @Test public void spam_extended_warranty() {
        IntentClassifier.Result r = classify("your extended warranty is expiring");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.95, r.score, EPS);
    }

    @Test public void spam_car_warranty() {
        IntentClassifier.Result r = classify("about your car warranty");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.95, r.score, EPS);
    }

    @Test public void spam_been_trying() {
        IntentClassifier.Result r = classify("we have been trying to reach you");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.90, r.score, EPS);
    }

    @Test public void spam_courtesy_call() {
        IntentClassifier.Result r = classify("this is a courtesy call");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.85, r.score, EPS);
    }

    @Test public void spam_special_offer() {
        IntentClassifier.Result r = classify("you qualify for a special offer");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.85, r.score, EPS);
    }

    @Test public void spam_selected_for() {
        IntentClassifier.Result r = classify("you have been selected for");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.80, r.score, EPS);
    }

    @Test public void spam_press_1() {
        IntentClassifier.Result r = classify("press 1 to speak to an agent");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.90, r.score, EPS);
    }

    @Test public void spam_press_one() {
        IntentClassifier.Result r = classify("press one now");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.90, r.score, EPS);
    }

    @Test public void spam_limited_time() {
        IntentClassifier.Result r = classify("this limited time deal");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.80, r.score, EPS);
    }

    @Test public void spam_act_now() {
        IntentClassifier.Result r = classify("you must act now");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.80, r.score, EPS);
    }

    @Test public void spam_free_gift() {
        IntentClassifier.Result r = classify("claim your free gift");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.85, r.score, EPS);
    }

    @Test public void spam_congratulations() {
        IntentClassifier.Result r = classify("congratulations you have been chosen");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.85, r.score, EPS);
    }

    @Test public void spam_you_have_won() {
        IntentClassifier.Result r = classify("you have won a cruise");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.90, r.score, EPS);
    }

    @Test public void spam_lower_rate() {
        IntentClassifier.Result r = classify("we can lower your rate");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.85, r.score, EPS);
    }

    @Test public void spam_reduce_debt() {
        IntentClassifier.Result r = classify("reduce your debt today");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.85, r.score, EPS);
    }

    @Test public void spam_the_irs() {
        IntentClassifier.Result r = classify("this is the irs calling");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        // "the irs" (0.80) and "irs agent" both could match — only "the irs" here.
        assertEquals(0.80, r.score, EPS);
    }

    @Test public void spam_irs_agent() {
        IntentClassifier.Result r = classify("an irs agent will contact you");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.85, r.score, EPS);
    }

    @Test public void spam_ssn() {
        IntentClassifier.Result r = classify("we need your social security number");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.95, r.score, EPS);
    }

    @Test public void spam_arrest_warrant() {
        IntentClassifier.Result r = classify("there is an arrest warrant");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.95, r.score, EPS);
    }

    @Test public void spam_legal_action() {
        IntentClassifier.Result r = classify("we will take legal action");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.80, r.score, EPS);
    }

    @Test public void spam_final_notice() {
        IntentClassifier.Result r = classify("this is your final notice");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.85, r.score, EPS);
    }

    @Test public void spam_from_bank() {
        IntentClassifier.Result r = classify("calling from your bank");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.70, r.score, EPS);
    }

    @Test public void spam_verify_account() {
        IntentClassifier.Result r = classify("please verify your account");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.85, r.score, EPS);
    }

    @Test public void spam_confirm_identity() {
        IntentClassifier.Result r = classify("confirm your identity now");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.80, r.score, EPS);
    }

    // =========================================================================
    // LEGIT — every pattern individually
    // =========================================================================

    @Test public void legit_appointment() {
        IntentClassifier.Result r = classify("your appointment is tomorrow");
        assertEquals(IntentClassifier.Verdict.LEGITIMATE, r.verdict);
        assertEquals(0.80, r.score, EPS);
    }

    @Test public void legit_confirming() {
        IntentClassifier.Result r = classify("confirming your reservation");
        assertEquals(IntentClassifier.Verdict.LEGITIMATE, r.verdict);
        assertEquals(0.85, r.score, EPS);
    }

    @Test public void legit_returning_call() {
        IntentClassifier.Result r = classify("returning your call");
        assertEquals(IntentClassifier.Verdict.LEGITIMATE, r.verdict);
        assertEquals(0.90, r.score, EPS);
    }

    @Test public void legit_you_called_us() {
        IntentClassifier.Result r = classify("you called us earlier");
        assertEquals(IntentClassifier.Verdict.LEGITIMATE, r.verdict);
        assertEquals(0.85, r.score, EPS);
    }

    @Test public void legit_this_is_dr() {
        IntentClassifier.Result r = classify("this is dr smith");
        assertEquals(IntentClassifier.Verdict.LEGITIMATE, r.verdict);
        assertEquals(0.80, r.score, EPS);
    }

    @Test public void legit_this_is_doctor() {
        IntentClassifier.Result r = classify("this is doctor jones");
        assertEquals(IntentClassifier.Verdict.LEGITIMATE, r.verdict);
        assertEquals(0.80, r.score, EPS);
    }

    @Test public void legit_your_order() {
        IntentClassifier.Result r = classify("calling about your order");
        assertEquals(IntentClassifier.Verdict.LEGITIMATE, r.verdict);
        assertEquals(0.70, r.score, EPS);
    }

    @Test public void legit_delivery() {
        IntentClassifier.Result r = classify("your delivery is on the way");
        assertEquals(IntentClassifier.Verdict.LEGITIMATE, r.verdict);
        assertEquals(0.70, r.score, EPS);
    }

    @Test public void legit_picking_up() {
        IntentClassifier.Result r = classify("i am picking up the package");
        assertEquals(IntentClassifier.Verdict.LEGITIMATE, r.verdict);
        assertEquals(0.75, r.score, EPS);
    }

    @Test public void legit_schedule() {
        IntentClassifier.Result r = classify("about your schedule");
        assertEquals(IntentClassifier.Verdict.LEGITIMATE, r.verdict);
        assertEquals(0.70, r.score, EPS);
    }

    @Test public void legit_follow_up() {
        IntentClassifier.Result r = classify("just a follow up call");
        assertEquals(IntentClassifier.Verdict.LEGITIMATE, r.verdict);
        assertEquals(0.70, r.score, EPS);
    }

    @Test public void legit_checking_in() {
        IntentClassifier.Result r = classify("checking in on you");
        assertEquals(IntentClassifier.Verdict.LEGITIMATE, r.verdict);
        assertEquals(0.65, r.score, EPS);
    }

    @Test public void legit_application() {
        IntentClassifier.Result r = classify("regarding your application");
        assertEquals(IntentClassifier.Verdict.LEGITIMATE, r.verdict);
        assertEquals(0.65, r.score, EPS);
    }

    @Test public void legit_interview() {
        IntentClassifier.Result r = classify("about your interview");
        assertEquals(IntentClassifier.Verdict.LEGITIMATE, r.verdict);
        assertEquals(0.80, r.score, EPS);
    }

    // =========================================================================
    // UNKNOWN — no-match
    // =========================================================================

    @Test public void unknown_hello() {
        assertEquals(IntentClassifier.Verdict.UNKNOWN, classify("hello").verdict);
    }

    @Test public void unknown_empty() {
        IntentClassifier.Result r = classify("");
        assertEquals(IntentClassifier.Verdict.UNKNOWN, r.verdict);
        assertEquals(0.50, r.score, EPS);
    }

    @Test public void unknown_gibberish() {
        assertEquals(IntentClassifier.Verdict.UNKNOWN, classify("asdf qwerty zxcv").verdict);
    }

    @Test public void unknown_numbers_only() {
        assertEquals(IntentClassifier.Verdict.UNKNOWN, classify("12345").verdict);
    }

    @Test public void unknown_single_char() {
        assertEquals(IntentClassifier.Verdict.UNKNOWN, classify("x").verdict);
    }

    @Test public void unknown_whitespace() {
        assertEquals(IntentClassifier.Verdict.UNKNOWN, classify("   ").verdict);
    }

    @Test public void unknown_punctuation() {
        assertEquals(IntentClassifier.Verdict.UNKNOWN, classify("...!!!???").verdict);
    }

    // =========================================================================
    // CASE INSENSITIVITY
    // =========================================================================

    @Test public void case_upper_spam() {
        assertEquals(IntentClassifier.Verdict.SPAM, classify("EXTENDED WARRANTY").verdict);
    }

    @Test public void case_mixed_legit() {
        assertEquals(IntentClassifier.Verdict.LEGITIMATE, classify("Returning Your Call").verdict);
    }

    @Test public void case_mixed_spam() {
        assertEquals(IntentClassifier.Verdict.SPAM, classify("Press 1 NOW").verdict);
    }

    // =========================================================================
    // FALSE-POSITIVE REGRESSION — must not classify benign speech as spam
    // =========================================================================

    @Test public void fp_first_not_spam() {
        assertNotEquals(IntentClassifier.Verdict.SPAM, classify("this is your first appointment").verdict);
    }

    @Test public void fp_birthday_not_spam() {
        assertNotEquals(IntentClassifier.Verdict.SPAM, classify("happy birthday to you").verdict);
    }

    @Test public void fp_thirsty_not_spam() {
        assertNotEquals(IntentClassifier.Verdict.SPAM, classify("i am thirsty").verdict);
    }

    @Test public void fp_stairs_not_spam() {
        assertNotEquals(IntentClassifier.Verdict.SPAM, classify("take the stairs").verdict);
    }

    @Test public void fp_pairs_not_spam() {
        assertNotEquals(IntentClassifier.Verdict.SPAM, classify("three pairs of shoes").verdict);
    }

    @Test public void fp_desire_not_spam() {
        assertNotEquals(IntentClassifier.Verdict.SPAM, classify("my desire is to help").verdict);
    }

    @Test public void fp_bird_not_spam() {
        assertNotEquals(IntentClassifier.Verdict.SPAM, classify("a bird in the hand").verdict);
    }

    // =========================================================================
    // MULTI-PATTERN — highest weight wins, ties resolve to UNKNOWN
    // =========================================================================

    @Test public void multi_spam_patterns_highest_wins() {
        IntentClassifier.Result r = classify("press 1 for your extended warranty final notice");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertEquals(0.95, r.score, EPS); // extended warranty = 0.95
    }

    @Test public void multi_legit_patterns_highest_wins() {
        IntentClassifier.Result r = classify("returning your call about your appointment schedule");
        assertEquals(IntentClassifier.Verdict.LEGITIMATE, r.verdict);
        assertEquals(0.90, r.score, EPS); // returning your call = 0.90
    }

    @Test public void spam_beats_legit_when_higher() {
        IntentClassifier.Result r = classify("extended warranty for your appointment");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertTrue("score should exceed 0.80, was " + r.score, r.score > 0.80);
        assertTrue(r.matched.contains("extended warranty"));
        assertTrue(r.matched.contains("appointment"));
    }

    @Test public void legit_beats_spam_when_higher() {
        IntentClassifier.Result r = classify("returning your call about a limited time schedule");
        assertEquals(IntentClassifier.Verdict.LEGITIMATE, r.verdict);
        assertEquals(0.90, r.score, EPS);
    }

    @Test public void tied_scores_are_unknown() {
        // "appointment" (legit 0.80) vs "the irs" (spam 0.80) — tie → UNKNOWN
        assertEquals(IntentClassifier.Verdict.UNKNOWN, classify("the irs appointment").verdict);
    }

    @Test public void matched_list_contains_all_hits() {
        IntentClassifier.Result r = classify("press 1 for your car warranty");
        assertTrue(r.matched.contains("press 1"));
        assertTrue(r.matched.contains("car warranty"));
    }

    // =========================================================================
    // SCORE BOUNDARY CONDITIONS
    // =========================================================================

    @Test public void score_never_negative() {
        String[] inputs = {"", "hello", "random words", "a b c d e f", "xyz 123 !@#"};
        for (String input : inputs) {
            IntentClassifier.Result r = classify(input);
            assertTrue("score for '" + input + "' was " + r.score, r.score >= 0.0);
        }
    }

    @Test public void score_never_above_one() {
        String[] inputs = {
            "extended warranty arrest warrant social security number",
            "returning your call confirming your appointment interview",
        };
        for (String input : inputs) {
            IntentClassifier.Result r = classify(input);
            assertTrue("score for '" + input + "' was " + r.score, r.score <= 1.0);
        }
    }

    @Test public void unknown_empty_score_is_half() {
        // No patterns matched: spamMax=0, legitMax=0 → score = 0.5 - |0-0| = 0.5
        IntentClassifier.Result r = classify("");
        assertEquals(0.50, r.score, EPS);
    }

    @Test public void low_spam_above_threshold_classifies() {
        // "from your bank" weight 0.70 > 0.5 default → SPAM
        assertEquals(IntentClassifier.Verdict.SPAM, classify("from your bank").verdict);
    }

    // =========================================================================
    // VISHING — high-stakes vectors that must always classify as SPAM
    // =========================================================================

    @Test public void vish_irs_arrest() {
        IntentClassifier.Result r = classify("the irs has issued an arrest warrant for you");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertTrue("vishing score must be high, was " + r.score, r.score >= 0.90);
    }

    @Test public void vish_ssn_legal() {
        IntentClassifier.Result r = classify("provide your social security number to avoid legal action");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertTrue(r.score >= 0.90);
    }

    @Test public void vish_bank_confirm() {
        IntentClassifier.Result r = classify("calling from your bank to verify your account");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
    }

    @Test public void vish_verify_now() {
        IntentClassifier.Result r = classify("press 1 to verify your account immediately");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
    }

    @Test public void vish_identity_theft() {
        IntentClassifier.Result r = classify("we need to confirm your identity to stop identity theft");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
    }

    // =========================================================================
    // RESULT STRUCT — basic invariants
    // =========================================================================

    @Test public void result_fields_populated_on_spam() {
        IntentClassifier.Result r = classify("extended warranty");
        assertEquals(IntentClassifier.Verdict.SPAM, r.verdict);
        assertTrue(r.score > 0.0);
        assertFalse(r.matched.isEmpty());
    }

    @Test public void result_matched_empty_on_no_hit() {
        IntentClassifier.Result r = classify("xyz");
        assertEquals(IntentClassifier.Verdict.UNKNOWN, r.verdict);
        assertTrue(r.matched.isEmpty());
    }
}
