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
import android.os.PowerManager;
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

    // WakeLock held during scanning so Android does not throttle the process
    private static PowerManager.WakeLock sWakeLock = null;

    /**
     * Acquire a PARTIAL_WAKE_LOCK so the CPU keeps running while Czkawka
     * scans files in the background.  Safe to call from any thread.
     * A 60-minute safety timeout is set in case releaseWakeLock is never called.
     */
    public static void acquireWakeLock(Activity activity) {
        if (sWakeLock != null && sWakeLock.isHeld()) {
            Log.d(TAG, "acquireWakeLock: already held, skipping");
            return;
        }
        PowerManager pm = (PowerManager) activity.getSystemService(Activity.POWER_SERVICE);
        if (pm == null) {
            Log.e(TAG, "acquireWakeLock: PowerManager not available");
            return;
        }
        sWakeLock = pm.newWakeLock(PowerManager.PARTIAL_WAKE_LOCK, "Cedinia:ScanWakeLock");
        sWakeLock.acquire(60 * 60 * 1000L /* 60 minutes timeout */);
        Log.i(TAG, "acquireWakeLock: acquired");
    }

    /**
     * Release the WakeLock acquired by {@link #acquireWakeLock}.
     * Silently ignored if no lock is currently held.
     */
    public static void releaseWakeLock(Activity activity) {
        if (sWakeLock != null && sWakeLock.isHeld()) {
            sWakeLock.release();
            sWakeLock = null;
            Log.i(TAG, "releaseWakeLock: released");
        } else {
            Log.d(TAG, "releaseWakeLock: not held, skipping");
        }
    }

    // permission helpers

    /**
     * Returns true if the app currently has broad file read access.
     * Android 11+: checks MANAGE_EXTERNAL_STORAGE.
     * Android 6-10: checks READ_EXTERNAL_STORAGE.
     * Below Android 6: always true.
     */
    public static boolean hasStoragePermission(Activity activity) {
        if (Build.VERSION.SDK_INT >= 30) {
            // Android 11+ - MANAGE_EXTERNAL_STORAGE
            return Environment.isExternalStorageManager();
        } else if (Build.VERSION.SDK_INT >= 23) {
            // Android 6-10 - runtime READ_EXTERNAL_STORAGE
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

    // nav bar visibility

    /**
     * Enables the "swipe from bottom edge to temporarily reveal the system nav bar"
     * (immersive-sticky) behaviour.  When the navigation bar is hidden by Slint's
     * renderer, a swipe from the bottom shows it transiently and it auto-hides again.
     *
     * API 30+: WindowInsetsController.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE.
     * API <30:  adds SYSTEM_UI_FLAG_IMMERSIVE_STICKY whenever HIDE_NAVIGATION is set.
     */
    public static void setupNavBar(final Activity activity) {
        activity.runOnUiThread(new Runnable() {
            @Override
            public void run() {
                if (Build.VERSION.SDK_INT >= 30) {
                    android.view.WindowInsetsController controller =
                            activity.getWindow().getInsetsController();
                    if (controller != null) {
                        controller.setSystemBarsBehavior(
                            android.view.WindowInsetsController.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE);
                        Log.i(TAG, "setupNavBar: BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE set (API 30+)");
                    } else {
                        Log.w(TAG, "setupNavBar: InsetsController is null");
                    }
                } else {
                    final View decorView = activity.getWindow().getDecorView();
                    decorView.setOnSystemUiVisibilityChangeListener(
                        new View.OnSystemUiVisibilityChangeListener() {
                            @Override
                            public void onSystemUiVisibilityChange(int visibility) {
                                if ((visibility & View.SYSTEM_UI_FLAG_HIDE_NAVIGATION) != 0
                                        && (visibility & View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY) == 0) {
                                    decorView.setSystemUiVisibility(
                                        visibility | View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY);
                                }
                            }
                        }
                    );
                    int current = decorView.getSystemUiVisibility();
                    if ((current & View.SYSTEM_UI_FLAG_HIDE_NAVIGATION) != 0) {
                        decorView.setSystemUiVisibility(
                            current | View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY);
                    }
                    Log.i(TAG, "setupNavBar: IMMERSIVE_STICKY listener configured (API < 30)");
                }
            }
        });
    }

    // called from Rust

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

    // MIME type helper

    private static String getMimeType(String path) {
        int dot = path.lastIndexOf('.');
        if (dot < 0) return "*/*";
        switch (path.substring(dot + 1).toLowerCase()) {
            case "jpg": case "jpeg": return "image/jpeg";
            case "png":  return "image/png";
            case "gif":  return "image/gif";
            case "webp": return "image/webp";
            case "bmp":  return "image/bmp";
            case "heic": case "heif": return "image/heic";
            case "mp3":  return "audio/mpeg";
            case "flac": return "audio/flac";
            case "ogg":  return "audio/ogg";
            case "m4a":  return "audio/mp4";
            case "wav":  return "audio/wav";
            case "mp4":  return "video/mp4";
            case "mkv":  return "video/x-matroska";
            case "avi":  return "video/avi";
            case "mov":  return "video/quicktime";
            case "pdf":  return "application/pdf";
            case "zip":  return "application/zip";
            case "txt":  return "text/plain";
            default:     return "*/*";
        }
    }

    /**
     * Query MediaStore for a content:// URI matching the given filesystem path.
     * Returns null if the file is not indexed in MediaStore.
     */
    private static Uri queryMediaStoreUri(Activity activity, String path) {
        String[] projection = { android.provider.MediaStore.MediaColumns._ID };
        String   selection  = android.provider.MediaStore.MediaColumns.DATA + "=?";
        String[] args       = { path };
        Uri[] collections = {
            android.provider.MediaStore.Images.Media.EXTERNAL_CONTENT_URI,
            android.provider.MediaStore.Video.Media.EXTERNAL_CONTENT_URI,
            android.provider.MediaStore.Audio.Media.EXTERNAL_CONTENT_URI,
            android.provider.MediaStore.Files.getContentUri("external"),
        };
        for (Uri col : collections) {
            try {
                android.database.Cursor c = activity.getContentResolver().query(
                    col, projection, selection, args, null);
                if (c != null) {
                    try {
                        if (c.moveToFirst()) {
                            long id = c.getLong(0);
                            return Uri.withAppendedPath(col, String.valueOf(id));
                        }
                    } finally {
                        c.close();
                    }
                }
            } catch (Exception e) {
                Log.d(TAG, "queryMediaStoreUri: " + col + " skipped: " + e);
            }
        }
        return null;
    }

    /**
     * Open a file using the system default handler.
     * On API 26+ (minSdk) uses a MediaStore content:// URI so file:// exposure
     * exceptions are avoided.  Falls back to opening the parent folder when no
     * MediaStore entry exists for the file.
     */
    public static void openFile(final Activity activity, final String path) {
        activity.runOnUiThread(new Runnable() {
            @Override
            public void run() {
                try {
                    Uri contentUri = queryMediaStoreUri(activity, path);
                    if (contentUri == null) {
                        Log.w(TAG, "openFile: no MediaStore entry for " + path + " – opening folder");
                        String parent = new java.io.File(path).getParent();
                        if (parent != null) openFolderInternal(activity, parent);
                        return;
                    }
                    String mime = getMimeType(path);
                    Intent intent = new Intent(Intent.ACTION_VIEW);
                    intent.setDataAndType(contentUri, mime);
                    intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK
                                  | Intent.FLAG_GRANT_READ_URI_PERMISSION);
                    activity.startActivity(intent);
                    Log.i(TAG, "openFile: launched for " + path);
                } catch (Exception e) {
                    Log.e(TAG, "openFile: failed for " + path + ": " + e);
                }
            }
        });
    }

    private static void openFolderInternal(Activity activity, String path) {
        try {
            String docId;
            if (path.startsWith("/storage/emulated/0/")) {
                docId = "primary:" + path.substring("/storage/emulated/0/".length());
            } else if (path.equals("/storage/emulated/0")) {
                docId = "primary:";
            } else if (path.startsWith("/sdcard/")) {
                docId = "primary:" + path.substring("/sdcard/".length());
            } else if (path.equals("/sdcard")) {
                docId = "primary:";
            } else if (path.startsWith("/storage/")) {
                // Non-primary volume, e.g. /storage/FAEB-190A or /storage/FAEB-190A/Music.
                // DocumentsUI uses "<volumeId>:<subpath>" as the document ID.
                String rest = path.substring("/storage/".length());
                int slash = rest.indexOf('/');
                if (slash < 0) {
                    docId = rest + ":";
                } else {
                    docId = rest.substring(0, slash) + ":" + rest.substring(slash + 1);
                }
            } else {
                Log.w(TAG, "openFolder: unrecognised path prefix, cannot open: " + path);
                return;
            }
            // Build the DocumentsUI content URI via the official API so that the document ID
            // is correctly percent-encoded (e.g. slashes inside the ID become %2F).
            Uri folderUri = DocumentsContract.buildDocumentUri(
                    "com.android.externalstorage.documents", docId);
            Intent intent = new Intent(Intent.ACTION_VIEW);
            intent.setDataAndType(folderUri, DocumentsContract.Document.MIME_TYPE_DIR);
            intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
            activity.startActivity(intent);
            Log.i(TAG, "openFolder: launched for " + path + " (docId=" + docId + ")");
        } catch (Exception e) {
            Log.e(TAG, "openFolder: failed for " + path + ": " + e);
        }
    }

    /**
     * Open the folder at {@code path} in the system file manager.
     * Called from Rust via JNI.
     */
    public static void openFolder(final Activity activity, final String path) {
        activity.runOnUiThread(new Runnable() {
            @Override
            public void run() {
                openFolderInternal(activity, path);
            }
        });
    }

    /** Launch the folder picker for an "include" directory (no initial path). */
    public static void pickIncludeDirectory(Activity activity) {
        Log.i(TAG, "pickIncludeDirectory called, API=" + Build.VERSION.SDK_INT
                + " activity=" + activity.getClass().getName());
        launchPicker(activity, true, "");
    }

    /** Launch the folder picker for an "include" directory starting at startPath. */
    public static void pickIncludeDirectory(Activity activity, String startPath) {
        Log.i(TAG, "pickIncludeDirectory called, API=" + Build.VERSION.SDK_INT
                + " startPath='" + startPath + "'");
        launchPicker(activity, true, startPath);
    }

    /** Launch the folder picker for an "exclude" directory (no initial path). */
    public static void pickExcludeDirectory(Activity activity) {
        Log.i(TAG, "pickExcludeDirectory called, API=" + Build.VERSION.SDK_INT
                + " activity=" + activity.getClass().getName());
        launchPicker(activity, false, "");
    }

    /** Launch the folder picker for an "exclude" directory starting at startPath. */
    public static void pickExcludeDirectory(Activity activity, String startPath) {
        Log.i(TAG, "pickExcludeDirectory called, API=" + Build.VERSION.SDK_INT
                + " startPath='" + startPath + "'");
        launchPicker(activity, false, startPath);
    }

    // internal

    private static void launchPicker(final Activity activity, final boolean isInclude,
                                     final String startPath) {
        // Use a headless Fragment so that startActivityForResult / onActivityResult
        // works correctly with a plain NativeActivity (no subclass required).
        try {
            Log.i(TAG, "launchPicker: launching via CediniaPickerFragment"
                    + " isInclude=" + isInclude + " startPath='" + startPath + "'");
            CediniaPickerFragment.launch(activity, isInclude, startPath);
        } catch (Exception e) {
            // Fallback: if the Fragment approach fails for any reason, offer a
            // text-entry dialog so the user can still type a path manually.
            Log.w(TAG, "launchPicker: Fragment approach failed (" + e + "), falling back to dialog");
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
            Log.i(TAG, "handleActivityResult: cancelled or null data - resultCode=" + resultCode);
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

    static void persistPermission(Activity activity, Uri treeUri) {
        try {
            int flags = Intent.FLAG_GRANT_READ_URI_PERMISSION;
            activity.getContentResolver().takePersistableUriPermission(treeUri, flags);
            Log.d(TAG, "persistPermission: ok for " + treeUri);
        } catch (SecurityException e) {
            Log.d(TAG, "persistPermission: provider does not support persistable perms - " + e.getMessage());
        }
    }

    /**
     * Best-effort conversion of a SAF tree URI to a filesystem path.
     */
    static String resolveTreeUriToPath(Uri treeUri) {
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

    // native callback registered by Rust via register_native_methods

    /**
     * Called after the user picks a directory (either SAF or dialog).
     * Rust registers the native implementation at startup.
     */
    public static native void onDirectoryPicked(String path, boolean isInclude);
}
