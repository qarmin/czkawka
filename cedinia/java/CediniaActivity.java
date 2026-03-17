// CediniaActivity.java
// Thin NativeActivity subclass that intercepts onActivityResult and forwards
// it to the CediniaFilePicker helper, which then calls back into Rust via JNI.
//
// Usage: set android:name=".CediniaActivity" in the manifest activity element.
// cargo-apk injects the package name; we declare the package-less class here
// because the DEX is loaded dynamically (via InMemoryDexClassLoader) and the
// real package name is resolved at load time by the Rust side.

import android.app.NativeActivity;
import android.content.Intent;
import android.os.Bundle;
import android.view.View;
import android.view.WindowManager;

public class CediniaActivity extends NativeActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        // Prevent fullscreen mode from hiding the system navigation bar.
        // NativeActivity (and some GPU renderers) may set FLAG_FULLSCREEN;
        // FLAG_FORCE_NOT_FULLSCREEN overrides it before super.onCreate runs.
        getWindow().clearFlags(WindowManager.LayoutParams.FLAG_FULLSCREEN);
        getWindow().addFlags(WindowManager.LayoutParams.FLAG_FORCE_NOT_FULLSCREEN);
        super.onCreate(savedInstanceState);
    }

    @Override
    public void onWindowFocusChanged(boolean hasFocus) {
        super.onWindowFocusChanged(hasFocus);
        if (hasFocus) {
            // Reset system UI flags so status bar and nav bar stay visible.
            // Passing 0 clears SYSTEM_UI_FLAG_HIDE_NAVIGATION,
            // SYSTEM_UI_FLAG_FULLSCREEN, SYSTEM_UI_FLAG_IMMERSIVE, etc.
            getWindow().getDecorView().setSystemUiVisibility(0);
        }
    }

    @Override
    public void onBackPressed() {
        // Move the task to the background instead of finishing the Activity.
        // This mirrors the Home-button behaviour: tapping the launcher icon
        // will resume the existing instance rather than recreating it from
        // scratch (which is the default NativeActivity behaviour when back
        // is pressed on the root activity).
        moveTaskToBack(true);
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);
        CediniaFilePicker.handleActivityResult(this, requestCode, resultCode, data);
    }
}
