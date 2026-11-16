use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use ntfs_mft::{FileNameNamespace, MftError, RawMftLeanEntry, RawMftLeanScanOptions, Volume};

use crate::tools::disk_usage::scan::mft::assemble::{MftEntry, MftEntryKind};

pub(super) fn read_volume(volume: &Volume, stop_flag: &AtomicBool, entries_counter: &AtomicUsize) -> Result<Vec<MftEntry>, MftError> {
    let raw_mft = volume.raw_mft()?;
    let mut entries = Vec::new();
    let options = RawMftLeanScanOptions::builder().collect_dos_file_name_links(false).build();
    raw_mft.parallel().lean().scan_options(options).for_each_batch(|batch| {
        if stop_flag.load(Ordering::Relaxed) {
            return;
        }
        entries_counter.fetch_add(batch.entries.len(), Ordering::Relaxed);
        batch.entries.iter().for_each(|entry| push_entries(entry, &mut entries));
    })?;
    Ok(entries)
}

fn push_entries(entry: &RawMftLeanEntry, entries: &mut Vec<MftEntry>) {
    if !entry.is_used || entry.base_reference().is_some() {
        return;
    }
    let kind = if entry.reparse_tag.is_some() {
        MftEntryKind::Symlink
    } else if entry.is_directory {
        MftEntryKind::Directory
    } else {
        MftEntryKind::File
    };
    let size = if kind == MftEntryKind::File { entry.real_size } else { 0 };

    let mut links = entry.links.iter().filter(|link| link.namespace != FileNameNamespace::Dos);
    if let Some(first) = links.next() {
        entries.push(entry_for(entry, first.parent_reference.record_number(), first.file_name.to_string_lossy(), kind, size));
        if kind == MftEntryKind::File {
            entries.extend(links.map(|link| entry_for(entry, link.parent_reference.record_number(), link.file_name.to_string_lossy(), kind, 0)));
        }
    } else {
        entries.push(entry_for(entry, entry.parent_reference.record_number(), entry.file_name.to_string_lossy(), kind, size));
    }
}

fn entry_for(entry: &RawMftLeanEntry, parent_record: u64, name: String, kind: MftEntryKind, size: u64) -> MftEntry {
    MftEntry {
        record: entry.record_number,
        parent_record,
        name: name.into_boxed_str(),
        kind,
        size,
    }
}
