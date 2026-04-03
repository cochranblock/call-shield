package org.cochranblock.callshield;

import android.telecom.Call;
import android.telecom.CallScreeningService;
import android.telecom.TelecomManager;
import android.util.Log;

/**
 * f7=ShieldScreeningService — Android call screening service.
 * Registered in AndroidManifest.xml. Android routes incoming calls here
 * when the user sets Call Shield as their default screening app.
 *
 * Zero audio leaves the device. No INTERNET permission in manifest.
 */
public class ShieldScreeningService extends CallScreeningService {

    private static final String TAG = "CallShield";

    @Override
    public void onScreenCall(Call.Details details) {
        int presentation = details.getHandlePresentation();
        String number = details.getHandle() != null
            ? details.getHandle().getSchemeSpecificPart()
            : "unknown";

        Log.d(TAG, "Screening call from: " + number);

        CallResponse.Builder response = new CallResponse.Builder();

        if (presentation == TelecomManager.PRESENTATION_RESTRICTED
                || presentation == TelecomManager.PRESENTATION_UNKNOWN) {
            // Restricted/unknown caller ID — classify the number itself
            IntentClassifier.Result result = IntentClassifier.classify(number);
            Log.d(TAG, "Classified restricted caller: " + result.verdict
                + " score=" + result.score);

            if (result.verdict == IntentClassifier.Verdict.SPAM) {
                Log.d(TAG, "Blocked: spam classification (" + result.matched + ")");
                response.setDisallowCall(true)
                        .setRejectCall(true)
                        .setSkipNotification(false)
                        .setSkipCallLog(false);
            } else {
                // Unknown/restricted but not classified as spam — let ring
                Log.d(TAG, "Allowed: restricted but not spam-classified");
                response.setDisallowCall(false)
                        .setRejectCall(false);
            }
        } else {
            Log.d(TAG, "Allowed: known presentation — " + number);
            response.setDisallowCall(false)
                    .setRejectCall(false);
        }

        respondToCall(details, response.build());
    }
}
