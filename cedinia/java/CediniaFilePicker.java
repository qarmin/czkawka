// CediniaFilePicker.java
// Launches the Android Storage Access Framework folder picker and calls back into
// Rust once the user makes a selection.
//
// For NativeActivity (no ComponentActivity): shows an AlertDialog with an EditText
// so the user can type/paste a path.  The SAF picker via startActivityForResult is
// also attempted if the manifest activity is CediniaActivity.

import android.app.Activity;
import android.app.AlertDialog;
import android.content.DialogInterface;
import android.content.Intent;
import android.content.pm.PackageManager;
import android.net.Uri;
import android.os.Build;
import android.os.Environment;
import android.provider.DocumentsContract;
import android.provider.Settings;
import android.util.Log;
import android.view.View;
import android.widget.EditText;
import android.widget.LinearLayout;

public class CediniaFilePicker {
    private static final String TAG = "CediniaFilePicker";

    // Request codes used with startActivityForResult (CediniaActivity path)
    static final int REQ_INCLUDE = 0x4345_0001;
    static final int REQ_EXCLUDE = 0x4345_0002;

    // ── permission helpers ────────────────────────────────────────────────

    /**
     * Returns true if the app currently has broad file read access.
     * Android 11+: checks MANAGE_EXTERNAL_STORAGE.
     * Android 6-10: checks READ_EXTERNAL_STORAGE.
     * Below Android 6: always true.
     */
    public static boolean hasStoragePermission(Activity activity) {
        if (Build.VERSION.SDK_INT >= 30) {
            // Android 11+ – MANAGE_EXTERNAL_STORAGE
            return Environment.isExternalStorageManager();
        } else if (Build.VERSION.SDK_INT >= 23) {
            // Android 6-10 – runtime READ_EXTERNAL_STORAGE
            return activity.checkSelfPermission("android.permission.READ_EXTERNAL_STORAGE")
                    == PackageManager.PERMISSION_GRANTED;
        } else {
            return true;
        }
    }

    /**
     * Opens the appropriate system UI to grant storage permission:
     * - Android 11+: Settings > Special app access > All files access
     * - Android 6-10: requestPermissions for READ/WRITE_EXTERNAL_STORAGE
     */
    public static void requestStoragePermission(final Activity activity) {
        Log.i(TAG, "requestStoragePermission: API=" + Build.VERSION.SDK_INT);
        if (Build.VERSION.SDK_INT >= 30) {
            // Must send user to the special settings screen for MANAGE_EXTERNAL_STORAGE
            activity.runOnUiThread(new Runnable() {
                @Override
                public void run() {
                    try {
                        Intent intent = new Intent(
                                Settings.ACTION_MANAGE_APP_ALL_FILES_ACCESS_PERMISSION,
                                Uri.parse("package:" + activity.getPackageName()));
                        activity.startActivity(intent);
                    } catch (Exception e) {
                        // Fallback: open the general "all files" settings page
                        Log.w(TAG, "requestStoragePermission: direct intent failed, opening generic: " + e);
                        try {
                            activity.startActivity(
                                    new Intent(Settings.ACTION_MANAGE_ALL_FILES_ACCESS_PERMISSION));
                        } catch (Exception e2) {
                            Log.e(TAG, "requestStoragePermission: fallback also failed: " + e2);
                        }
                    }
                }
            });
        } else if (Build.VERSION.SDK_INT >= 23) {
            activity.requestPermissions(new String[]{
                    "android.permission.READ_EXTERNAL_STORAGE",
                    "android.permission.WRITE_EXTERNAL_STORAGE"
            }, 0x4345_0010);
        }
        // Below API 23: permissions are granted at install time, nothing to do
    }

    // ── nav bar visibility ────────────────────────────────────────────────

    /**
     * Clears immersive / hide-navigation flags so the system nav bar (Back,
     * Home) stays visible.  Re-applies the clear whenever Slint's renderer
     * would hide it again.  Call once after Slint has been initialised.
     */
    public static void setupNavBar(final Activity activity) {
        activity.runOnUiThread(new Runnable() {
            @Override
            public void run() {
                final View decorView = activity.getWindow().getDecorView();
                decorView.setSystemUiVisibility(0);
                decorView.setOnSystemUiVisibilityChangeListener(
                    new View.OnSystemUiVisibilityChangeListener() {
                        @Override
                        public void onSystemUiVisibilityChange(int visibility) {
                            if ((visibility & View.SYSTEM_UI_FLAG_HIDE_NAVIGATION) != 0) {
                                decorView.setSystemUiVisibility(0);
                            }
                        }
                    }
                );
                Log.i(TAG, "setupNavBar: nav bar listener registered");
            }
        });
    }

    // ── called from Rust ──────────────────────────────────────────────────

    /** Open a URL in the system browser. */
    public static void openUrl(final Activity activity, final String url) {
        activity.runOnUiThread(new Runnable() {
            @Override
            public void run() {
                try {
                    Intent intent = new Intent(Intent.ACTION_VIEW, Uri.parse(url));
                    intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
                    activity.startActivity(intent);
                } catch (Exception e) {
                    Log.e(TAG, "openUrl: failed to open " + url + ": " + e);
                }
            }
        });
    }

    /** Launch the folder picker for an "include" directory. */
    public static void pickIncludeDirectory(Activity activity) {
        Log.i(TAG, "pickIncludeDirectory called, API=" + Build.VERSION.SDK_INT
                + " activity=" + activity.getClass().getName());
        launchPicker(activity, true);
    }

    /** Launch the folder picker for an "exclude" directory. */
    public static void pickExcludeDirectory(Activity activity) {
        Log.i(TAG, "pickExcludeDirectory called, API=" + Build.VERSION.SDK_INT
                + " activity=" + activity.getClass().getName());
        launchPicker(activity, false);
    }

    // ── internal ──────────────────────────────────────────────────────────

    private static void launchPicker(final Activity activity, final boolean isInclude) {
        // Try the SAF picker via startActivityForResult only when running in
        // a CediniaActivity subclass that can intercept onActivityResult.
        String activityClass = activity.getClass().getName();
        if (activityClass.contains("CediniaActivity")) {
            Log.i(TAG, "launchPicker: using SAF startActivityForResult");
            Intent intent = new Intent(Intent.ACTION_OPEN_DOCUMENT_TREE);
            intent.addFlags(
                    Intent.FLAG_GRANT_READ_URI_PERMISSION |
                    Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION);
            activity.startActivityForResult(intent,
                    isInclude ? REQ_INCLUDE : REQ_EXCLUDE);
        } else {
            // NativeActivity does not intercept onActivityResult via Rust,
            // so we fall back to a simple path-entry dialog.
            Log.w(TAG, "launchPicker: NativeActivity detected – using path-entry dialog");
            showPathDialog(activity, isInclude);
        }
    }

    /** Text-entry fallback for NativeActivity where SAF result cannot be received. */
    static void showPathDialog(final Activity activity, final boolean isInclude) {
        Log.i(TAG, "showPathDialog: isInclude=" + isInclude);
        activity.runOnUiThread(new Runnable() {
            @Override
            public void run() {
                final EditText et = new EditText(activity);
                et.setHint(isInclude ? "/sdcard/DCIM" : "/sdcard/Android");
                et.setSingleLine(true);

                LinearLayout layout = new LinearLayout(activity);
                layout.setOrientation(LinearLayout.VERTICAL);
                int pad = (int)(16 * activity.getResources().getDisplayMetrics().density);
                layout.setPadding(pad, pad, pad, pad);
                layout.addView(et);

                new AlertDialog.Builder(activity)
                    .setTitle(isInclude ? "Add scan directory" : "Add exclude directory")
                    .setMessage("Enter a full path (e.g. /sdcard/DCIM):")
                    .setView(layout)
                    .setPositiveButton("Add", new DialogInterface.OnClickListener() {
                        @Override
                        public void onClick(DialogInterface dialog, int which) {
                            String path = et.getText().toString().trim();
                            Log.i(TAG, "showPathDialog confirmed: path='" + path
                                    + "' isInclude=" + isInclude);
                            if (!path.isEmpty()) {
                                onDirectoryPicked(path, isInclude);
                            } else {
                                Log.w(TAG, "showPathDialog: empty path, ignoring");
                            }
                        }
                    })
                    .setNegativeButton("Cancel", new DialogInterface.OnClickListener() {
                        @Override
                        public void onClick(DialogInterface dialog, int which) {
                            Log.i(TAG, "showPathDialog: user cancelled");
                        }
                    })
                    .show();
            }
        });
    }

    /**
     * Called by CediniaActivity.onActivityResult (SAF path).
     */
    public static void handleActivityResult(
            Activity activity,
            int requestCode,
            int resultCode,
            Intent data) {

        Log.i(TAG, "handleActivityResult: requestCode=0x" + Integer.toHexString(requestCode)
                + " resultCode=" + resultCode + " data=" + data);

        if (requestCode != REQ_INCLUDE && requestCode != REQ_EXCLUDE) {
            Log.d(TAG, "handleActivityResult: not our request, ignoring");
            return;
        }
        if (resultCode != Activity.RESULT_OK || data == null) {
            Log.i(TAG, "handleActivityResult: cancelled or null data – resultCode=" + resultCode);
            return;
        }

        Uri treeUri = data.getData();
        if (treeUri == null) {
            Log.w(TAG, "handleActivityResult: null tree URI");
            return;
        }

        persistPermission(activity, treeUri);
        String path = resolveTreeUriToPath(treeUri);
        boolean isInclude = (requestCode == REQ_INCLUDE);
        Log.i(TAG, "handleActivityResult: resolved path='" + path + "' isInclude=" + isInclude);
        onDirectoryPicked(path, isInclude);
    }

    private static void persistPermission(Activity activity, Uri treeUri) {
        try {
            int flags = Intent.FLAG_GRANT_READ_URI_PERMISSION;
            activity.getContentResolver().takePersistableUriPermission(treeUri, flags);
            Log.d(TAG, "persistPermission: ok for " + treeUri);
        } catch (SecurityException e) {
            Log.d(TAG, "persistPermission: provider does not support persistable perms – " + e.getMessage());
        }
    }

    /**
     * Best-effort conversion of a SAF tree URI to a filesystem path.
     */
    private static String resolveTreeUriToPath(Uri treeUri) {
        Log.d(TAG, "resolveTreeUriToPath: input=" + treeUri);
        try {
            Uri docUri = DocumentsContract.buildDocumentUriUsingTree(
                    treeUri,
                    DocumentsContract.getTreeDocumentId(treeUri));
            String docId = DocumentsContract.getDocumentId(docUri);
            Log.d(TAG, "resolveTreeUriToPath: docId=" + docId);

            if (docId == null) {
                return treeUri.toString();
            }

            String[] parts = docId.split(":", 2);
            if (parts.length < 2) {
                return treeUri.toString();
            }

            String volume = parts[0];
            String subPath = parts[1];
            String root;
            if ("primary".equalsIgnoreCase(volume) || "home".equalsIgnoreCase(volume)) {
                root = "/sdcard";
            } else {
                root = "/storage/" + volume;
            }

            String result = subPath.isEmpty() ? root : root + "/" + subPath;
            Log.i(TAG, "resolveTreeUriToPath: result=" + result);
            return result;
        } catch (Exception e) {
            Log.w(TAG, "resolveTreeUriToPath: exception, returning raw URI string: " + e);
            return treeUri.toString();
        }
    }

    // ── native callback registered by Rust via register_native_methods ────

    /**
     * Called after the user picks a directory (either SAF or dialog).
     * Rust registers the native implementation at startup.
     */
    public static native void onDirectoryPicked(String path, boolean isInclude);
}
