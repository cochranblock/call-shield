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
            Log.d(TAG, "Blocked: restricted/unknown presentation");
            response.setDisallowCall(true)
                    .setRejectCall(true)
                    .setSkipNotification(false)
                    .setSkipCallLog(false);
        } else {
            Log.d(TAG, "Allowed: " + number);
            response.setDisallowCall(false)
                    .setRejectCall(false);
        }

        respondToCall(details, response.build());
    }
}
