// CediniaPickerFragment.java
// Headless Fragment used to launch the SAF folder picker (ACTION_OPEN_DOCUMENT_TREE)
// from a plain NativeActivity context.
//
// A Fragment's startActivityForResult + onActivityResult lifecycle is correctly
// dispatched by the Activity framework even when the host Activity does NOT override
// onActivityResult — so this avoids requiring CediniaActivity as the manifest activity.

import android.app.Activity;
import android.app.Fragment;
import android.content.Intent;
import android.net.Uri;
import android.os.Build;
import android.os.Bundle;
import android.provider.DocumentsContract;
import android.util.Log;

public class CediniaPickerFragment extends Fragment {
    private static final String TAG = "CediniaPickerFrag";

    // Request codes distinct from those in CediniaFilePicker to avoid collisions.
    private static final int REQ_INCLUDE = 0x4345_0006;
    private static final int REQ_EXCLUDE = 0x4345_0007;

    /**
     * Attach a headless picker fragment to the activity.
     * The SAF UI is launched in the fragment's onStart().
     * @param startPath  filesystem path to open the picker at (e.g. "/storage/XXXX-XXXX"),
     *                   or empty string / null to use the picker's default location.
     */
    static void launch(final Activity activity, final boolean isInclude, final String startPath) {
        Log.i(TAG, "launch: isInclude=" + isInclude + " startPath='" + startPath + "'");
        final CediniaPickerFragment frag = new CediniaPickerFragment();
        frag.mIsInclude = isInclude;
        frag.mStartPath = (startPath != null) ? startPath : "";
        final String tag = "cedinia_picker_" + (isInclude ? "inc" : "exc");
        // Run on the UI thread in case we are called from a background thread.
        activity.runOnUiThread(new Runnable() {
            @Override
            public void run() {
                try {
                    android.app.FragmentManager fm = activity.getFragmentManager();
                    android.app.FragmentTransaction tx = fm.beginTransaction();
                    // Remove any stale picker fragment left over from a previous attempt
                    // (e.g. after the activity was recreated while the picker was open).
                    Fragment existing = fm.findFragmentByTag(tag);
                    if (existing != null) {
                        Log.i(TAG, "launch: removing stale fragment with tag " + tag);
                        tx.remove(existing);
                    }
                    tx.add(frag, tag).commitAllowingStateLoss();
                } catch (Exception e) {
                    Log.e(TAG, "launch: fragment commit failed: " + e);
                }
            }
        });
    }

    private boolean mIsInclude;
    private String  mStartPath = "";
    /** Guard so the SAF intent is only fired once even if onStart() is re-entered. */
    private boolean mPickerLaunched;

    @Override
    public void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        if (savedInstanceState != null) {
            // This fragment is being restored after the activity was recreated
            // (e.g. the user backgrounded the app while the picker was open).
            // Do not relaunch the picker automatically – the user will tap the
            // button again to start a fresh pick.
            Log.i(TAG, "onCreate: restored from saved state, detaching immediately");
            detachSelf();
        }
    }

    @Override
    public void onStart() {
        super.onStart();
        if (!mPickerLaunched) {
            mPickerLaunched = true;
            Log.i(TAG, "onStart: starting ACTION_OPEN_DOCUMENT_TREE, isInclude=" + mIsInclude);
            try {
                Intent intent = new Intent(Intent.ACTION_OPEN_DOCUMENT_TREE);
                intent.addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION
                        | Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION);
                // Open the picker at the requested volume path (API 26+).
                if (Build.VERSION.SDK_INT >= 26 && mStartPath != null && !mStartPath.isEmpty()) {
                    Uri initialUri = buildInitialUri(mStartPath);
                    if (initialUri != null) {
                        Log.d(TAG, "onStart: setting EXTRA_INITIAL_URI=" + initialUri);
                        intent.putExtra(DocumentsContract.EXTRA_INITIAL_URI, initialUri);
                    }
                }
                startActivityForResult(intent, mIsInclude ? REQ_INCLUDE : REQ_EXCLUDE);
            } catch (Exception e) {
                Log.e(TAG, "onStart: startActivityForResult failed: " + e);
                detachSelf();
            }
        }
    }

    @Override
    public void onActivityResult(int requestCode, int resultCode, Intent data) {
        Log.i(TAG, "onActivityResult: req=0x" + Integer.toHexString(requestCode)
                + " result=" + resultCode);
        if (requestCode == REQ_INCLUDE || requestCode == REQ_EXCLUDE) {
            if (resultCode == Activity.RESULT_OK && data != null) {
                Uri treeUri = data.getData();
                if (treeUri != null) {
                    Activity act = getActivity();
                    if (act != null) {
                        CediniaFilePicker.persistPermission(act, treeUri);
                    }
                    String path = CediniaFilePicker.resolveTreeUriToPath(treeUri);
                    boolean isInclude = (requestCode == REQ_INCLUDE);
                    Log.i(TAG, "onActivityResult: resolved path='" + path
                            + "' isInclude=" + isInclude);
                    CediniaFilePicker.onDirectoryPicked(path, isInclude);
                } else {
                    Log.w(TAG, "onActivityResult: treeUri is null");
                }
            } else {
                Log.i(TAG, "onActivityResult: user cancelled (resultCode=" + resultCode + ")");
            }
            detachSelf();
        }
    }

    /**
     * Convert a filesystem path (e.g. "/storage/emulated/0/DCIM" or "/storage/XXXX-XXXX")
     * to a SAF tree URI suitable for DocumentsContract.EXTRA_INITIAL_URI.
     * Returns null if the path cannot be mapped.
     */
    private static Uri buildInitialUri(String path) {
        try {
            String docId;
            if (path.startsWith("/storage/emulated/0")) {
                String sub = path.substring("/storage/emulated/0".length());
                if (sub.startsWith("/")) sub = sub.substring(1);
                docId = sub.isEmpty() ? "primary:" : "primary:" + sub;
            } else if (path.equals("/sdcard") || path.startsWith("/sdcard/")) {
                String sub = path.equals("/sdcard") ? "" : path.substring("/sdcard/".length());
                docId = sub.isEmpty() ? "primary:" : "primary:" + sub;
            } else if (path.startsWith("/storage/")) {
                String rest = path.substring("/storage/".length());
                int slash = rest.indexOf('/');
                if (slash < 0) {
                    docId = rest + ":";
                } else {
                    docId = rest.substring(0, slash) + ":" + rest.substring(slash + 1);
                }
            } else {
                Log.d(TAG, "buildInitialUri: unrecognised path, skipping: " + path);
                return null;
            }
            Uri uri = DocumentsContract.buildDocumentUri(
                    "com.android.externalstorage.documents", docId);
            Log.d(TAG, "buildInitialUri: path='" + path + "' → docId='" + docId + "' uri=" + uri);
            return uri;
        } catch (Exception e) {
            Log.w(TAG, "buildInitialUri: exception for path='" + path + "': " + e);
            return null;
        }
    }

    private void detachSelf() {
        try {
            getFragmentManager()
                    .beginTransaction()
                    .remove(this)
                    .commitAllowingStateLoss();
        } catch (Exception e) {
            Log.e(TAG, "detachSelf: failed: " + e);
        }
    }
}
