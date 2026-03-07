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

public class CediniaActivity extends NativeActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);
        CediniaFilePicker.handleActivityResult(this, requestCode, resultCode, data);
    }
}

