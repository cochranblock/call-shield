package org.cochranblock.callshield;

import android.telecom.Call;
import android.telecom.CallScreeningService;
import android.util.Log;

/**
 * f7=ShieldScreeningService — Android call screening service.
 * Registered in AndroidManifest.xml. Android routes incoming calls here
 * when the user sets Call Shield as their default screening app.
 *
 * Flow:
 * 1. Android delivers call details (number, presentation)
 * 2. We check against known contacts (future: Whisper STT on audio)
 * 3. Classify intent on-device
 * 4. Respond: allow, reject, or silence
 *
 * Zero audio leaves the device. No INTERNET permission in manifest.
 */
public class ShieldScreeningService extends CallScreeningService {

    private static final String TAG = "CallShield";

    @Override
    public void onScreenCall(Call.Details details) {
        // Get caller info from call details
        int presentation = details.getCallerNumberPresentation();
        String number = details.getHandle() != null
            ? details.getHandle().getSchemeSpecificPart()
            : "unknown";

        Log.d(TAG, "Screening call from: " + number);

        // Decision logic
        CallResponse.Builder response = new CallResponse.Builder();

        if (presentation == Call.Details.PRESENTATION_RESTRICTED
                || presentation == Call.Details.PRESENTATION_UNKNOWN) {
            // Restricted/unknown caller ID — high spam signal
            Log.d(TAG, "Blocked: restricted/unknown presentation");
            response.setDisallowCall(true)
                    .setRejectCall(true)
                    .setSkipNotification(false)
                    .setSkipCallLog(false);
        } else {
            // For now: allow all calls with valid caller ID.
            // Future: pipe audio through Whisper STT → IntentClassifier
            // on-device, then decide based on transcript classification.
            //
            // The IntentClassifier is ready — it needs audio input from
            // either CallScreeningService audio access (API 33+) or
            // AccessibilityService as a fallback.
            Log.d(TAG, "Allowed: " + number);
            response.setDisallowCall(false)
                    .setRejectCall(false);
        }

        respondToCall(details, response.build());
    }
}
