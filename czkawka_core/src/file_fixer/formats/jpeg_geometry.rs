//! How a JPEG scan's coded units are laid out.
//!
//! Split out because getting this wrong is silent: the scan still decodes, it just counts the
//! wrong number of units, and the error only shows up as a crop of the wrong height.

use super::jpeg_markers::Layout;

/// How a scan's coded units are laid out.
///
/// A scan covering several components interleaves them into minimum coded units sized by the
/// largest sampling factors. A scan covering *one* component does not interleave at all: its
/// unit is a single 8x8 block, and the component's own dimensions decide how many there are.
/// Treating the second case like the first miscounts every file where a dimension is not a
/// multiple of the MCU size, which is most of them.
#[derive(Debug, Clone)]
pub struct ScanGeometry {
    /// Coded units in one row of the image.
    pub units_per_row: u32,
    pub unit_rows: u32,
    /// Blocks each unit contributes per scan component, in scan order.
    pub blocks_per_unit: Vec<u32>,
    /// Image rows one row of units covers, which is what a crop needs.
    pub pixel_rows_per_unit_row: u32,
}

impl ScanGeometry {
    pub fn total_units(&self) -> u32 {
        self.units_per_row.saturating_mul(self.unit_rows)
    }
}

/// Layout of the first scan's coded units.
pub fn scan_geometry(layout: &Layout) -> Option<ScanGeometry> {
    let frame = layout.frame.as_ref()?;
    let scan = layout.scans.first()?;
    let (h_max, v_max) = frame.max_sampling();

    if scan.components.len() == 1 {
        let id = scan.components.first()?.id;
        let component = frame.components.iter().find(|c| c.id == id)?;
        // The component's own resolution, which is the full image only when it is sampled
        // at the maximum rate.
        let width = u32::from(frame.width) * u32::from(component.h) / u32::from(h_max).max(1);
        let height = u32::from(frame.height) * u32::from(component.v) / u32::from(v_max).max(1);

        return Some(ScanGeometry {
            units_per_row: width.div_ceil(8).max(1),
            unit_rows: height.div_ceil(8).max(1),
            blocks_per_unit: vec![1],
            pixel_rows_per_unit_row: 8 * u32::from(v_max) / u32::from(component.v).max(1),
        });
    }

    let blocks_per_unit = scan
        .components
        .iter()
        .map(|component| frame.components.iter().find(|c| c.id == component.id).map(|c| u32::from(c.h) * u32::from(c.v)))
        .collect::<Option<Vec<u32>>>()?;

    Some(ScanGeometry {
        units_per_row: frame.mcus_per_row(),
        unit_rows: frame.mcu_rows(),
        blocks_per_unit,
        pixel_rows_per_unit_row: 8 * u32::from(v_max),
    })
}

#[cfg(test)]
mod tests {
    use super::super::jpeg_markers::{Frame, FrameComponent, FrameKind, Scan, ScanComponent};
    use super::*;

    fn layout_with(components: Vec<FrameComponent>, scan_ids: &[u8], width: u16, height: u16) -> Layout {
        Layout {
            frame: Some(Frame {
                kind: FrameKind::Baseline,
                precision: 8,
                width,
                height,
                height_offset: 0,
                components,
            }),
            scans: vec![Scan {
                data_offset: 0,
                components: scan_ids.iter().map(|&id| ScanComponent { id, dc_table: 0, ac_table: 0 }).collect(),
            }],
            ..Layout::default()
        }
    }

    /// Found by running the tool over a few hundred real photographs: a grayscale JPEG whose
    /// single component is sampled 2x2 was counted as if its units were 16x16 MCUs of four
    /// blocks. A one-component scan is not interleaved at all - its unit is one 8x8 block.
    #[test]
    fn a_single_component_scan_uses_plain_blocks_not_mcus() {
        let layout = layout_with(vec![FrameComponent { id: 1, h: 2, v: 2 }], &[1], 100, 50);
        let geometry = scan_geometry(&layout).expect("a scannable layout");

        assert_eq!(geometry.units_per_row, 100u32.div_ceil(8));
        assert_eq!(geometry.unit_rows, 50u32.div_ceil(8));
        assert_eq!(geometry.blocks_per_unit, vec![1]);
        assert_eq!(geometry.pixel_rows_per_unit_row, 8);
    }

    #[test]
    fn an_interleaved_scan_uses_the_largest_sampling_factors() {
        let layout = layout_with(
            vec![
                FrameComponent { id: 1, h: 2, v: 2 },
                FrameComponent { id: 2, h: 1, v: 1 },
                FrameComponent { id: 3, h: 1, v: 1 },
            ],
            &[1, 2, 3],
            201,
            203,
        );
        let geometry = scan_geometry(&layout).expect("a scannable layout");

        assert_eq!(geometry.units_per_row, 13);
        assert_eq!(geometry.unit_rows, 13);
        assert_eq!(geometry.blocks_per_unit, vec![4, 1, 1]);
        assert_eq!(geometry.pixel_rows_per_unit_row, 16);
    }

    /// A chroma component scanned on its own covers a quarter of the image, so its block grid
    /// is derived from its own resolution rather than the frame's.
    #[test]
    fn a_subsampled_component_scanned_alone_uses_its_own_resolution() {
        let layout = layout_with(vec![FrameComponent { id: 1, h: 2, v: 2 }, FrameComponent { id: 2, h: 1, v: 1 }], &[2], 160, 80);
        let geometry = scan_geometry(&layout).expect("a scannable layout");

        assert_eq!(geometry.units_per_row, 10);
        assert_eq!(geometry.unit_rows, 5);
        assert_eq!(geometry.pixel_rows_per_unit_row, 16);
    }
}
