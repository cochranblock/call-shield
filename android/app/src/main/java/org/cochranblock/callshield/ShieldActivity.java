package org.cochranblock.callshield;

import android.app.Activity;
import android.app.role.RoleManager;
import android.content.Intent;
import android.os.Bundle;
import android.widget.Button;
import android.widget.EditText;
import android.widget.TextView;

/**
 * f0=ShieldActivity — main screen.
 * Shows shield status, lets user test the classifier, and
 * prompts to set Call Shield as default screening app.
 */
public class ShieldActivity extends Activity {

    private static final int REQUEST_SCREENING_ROLE = 1;
    private TextView statusText;
    private TextView resultText;
    private EditText inputText;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_shield);

        statusText = findViewById(R.id.status_text);
        resultText = findViewById(R.id.result_text);
        inputText = findViewById(R.id.input_text);

        Button enableButton = findViewById(R.id.enable_button);
        enableButton.setOnClickListener(v -> requestScreeningRole());

        Button classifyButton = findViewById(R.id.classify_button);
        classifyButton.setOnClickListener(v -> runClassifier());

        updateStatus();
    }

    @Override
    protected void onResume() {
        super.onResume();
        updateStatus();
    }

    private void updateStatus() {
        RoleManager rm = getSystemService(RoleManager.class);
        boolean isDefault = rm.isRoleHeld(RoleManager.ROLE_CALL_SCREENING);

        if (isDefault) {
            statusText.setText("SHIELD ACTIVE\nScreening all incoming calls on-device.\nZero audio leaves this phone.");
        } else {
            statusText.setText("SHIELD INACTIVE\nTap Enable to set Call Shield as your screening app.");
        }
    }

    private void requestScreeningRole() {
        RoleManager rm = getSystemService(RoleManager.class);
        Intent intent = rm.createRequestRoleIntent(RoleManager.ROLE_CALL_SCREENING);
        // Using deprecated startActivityForResult: registerForActivityResult requires androidx.activity
        // dependency. Zero-dep policy takes precedence over API modernity.
        startActivityForResult(intent, REQUEST_SCREENING_ROLE);
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);
        if (requestCode == REQUEST_SCREENING_ROLE) {
            updateStatus();
        }
    }

    private void runClassifier() {
        String input = inputText.getText().toString().trim();
        if (input.isEmpty()) {
            resultText.setText("Enter caller speech to test classification.");
            return;
        }

        IntentClassifier.Result r = IntentClassifier.classify(input);
        String output = String.format(
            "Verdict: %s\nScore: %.0f%%\nMatched: %s",
            r.verdict, r.score * 100, r.matched.isEmpty() ? "(none)" : r.matched
        );
        resultText.setText(output);
    }
}
