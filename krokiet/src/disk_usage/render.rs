use std::f32::consts::TAU;

use czkawka_core::tools::disk_usage::category::FileCategory;
use czkawka_core::tools::disk_usage::layout::{Layout, LayoutItem, LayoutKind, SUNBURST_CENTER_RADIUS, Shape};
use czkawka_core::tools::disk_usage::tree::{CategorySizes, NodeKind};
use slint::{Rgba8Pixel, SharedPixelBuffer};

const BACKGROUND_DARK: [u8; 4] = [30, 30, 30, 255];
const BACKGROUND_LIGHT: [u8; 4] = [248, 248, 248, 255];
const NEUTRAL_ITEM: [u8; 3] = [96, 96, 96];
const SELECTION_DARK_THEME: [u8; 4] = [255, 255, 255, 255];
const SELECTION_LIGHT_THEME: [u8; 4] = [20, 20, 20, 255];
const DIRECTORY_BRIGHTNESS: f32 = 0.62;
/// Colour of the one-pixel lines between rectangles; the same for every rectangle, so they show up on any fill.
const LINE_DARK: [u8; 4] = [52, 56, 64, 255];
const LINE_LIGHT: [u8; 4] = [255, 255, 255, 255];
/// Directory frames, when nested borders are on, are this much darker than the lines between rectangles.
const NESTED_FRAME_BRIGHTNESS: f32 = 0.4;
const HATCH_SPACING_PX: usize = 9;
const HATCH_THICKNESS_PX: usize = 2;
const HATCH_BRIGHTNESS: f32 = 0.7;

/// Pixel size of the chart and where inside it the layout is drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ChartGeometry {
    pub width: u32,
    pub height: u32,
    pub kind: LayoutKind,
}

impl ChartGeometry {
    /// The sunburst is a circle, so it gets the biggest centred square; everything else fills the whole chart.
    fn drawing_area(self) -> (u32, u32, u32, u32) {
        if self.kind == LayoutKind::Sunburst {
            let side = self.width.min(self.height);
            ((self.width - side) / 2, (self.height - side) / 2, side, side)
        } else {
            (0, 0, self.width, self.height)
        }
    }

    /// Converts a position given as shares of the chart size into the unit square the layout uses.
    pub(crate) fn to_unit(self, x_share: f32, y_share: f32) -> Option<(f32, f32)> {
        let (left, top, width, height) = self.drawing_area();
        if width == 0 || height == 0 {
            return None;
        }
        let x = (x_share * self.width as f32 - left as f32) / width as f32;
        let y = (y_share * self.height as f32 - top as f32) / height as f32;
        ((0.0..1.0).contains(&x) && (0.0..1.0).contains(&y)).then_some((x, y))
    }
}

pub(crate) struct RenderSettings {
    pub geometry: ChartGeometry,
    /// Number of rings or rows, needed to place the rings of the sunburst.
    pub levels: usize,
    pub selected: Option<usize>,
    pub dark_theme: bool,
    /// A directory keeps its own frame around what is inside, so the lines pile up with every level of nesting;
    /// without it, entries fill their directory and only the faint border of each rectangle is left.
    pub nested_borders: bool,
}

/// Returns `None` when `is_current` reports that the result is no longer wanted.
pub(crate) fn render_chart(layout: &Layout, settings: &RenderSettings, is_current: &dyn Fn() -> bool) -> Option<SharedPixelBuffer<Rgba8Pixel>> {
    let ChartGeometry { width, height, kind } = settings.geometry;
    // Starts fully transparent, so whatever is not drawn shows the background of the window in any theme
    let mut buffer = SharedPixelBuffer::<Rgba8Pixel>::new(width.max(1), height.max(1));

    let drawn = match kind {
        LayoutKind::Sunburst => draw_sunburst(buffer.make_mut_bytes(), layout, settings, is_current),
        LayoutKind::Treemap => draw_rectangles(buffer.make_mut_bytes(), layout, settings, is_current),
    };
    drawn.then_some(buffer)
}

fn draw_rectangles(bytes: &mut [u8], layout: &Layout, settings: &RenderSettings, is_current: &dyn Fn() -> bool) -> bool {
    let (width, height) = (settings.geometry.width, settings.geometry.height);
    for (index, item) in layout.items.iter().enumerate() {
        if index % 512 == 0 && !is_current() {
            return false;
        }
        let Shape::Rect {
            x,
            y,
            width: item_width,
            height: item_height,
        } = item.shape
        else {
            continue;
        };
        let left = (x * width as f32).round() as u32;
        let top = (y * height as f32).round() as u32;
        let right = ((x + item_width) * width as f32).round() as u32;
        let bottom = ((y + item_height) * height as f32).round() as u32;
        if right <= left || bottom <= top {
            continue;
        }
        let rect = (left, top, right, bottom);
        let selection = (settings.selected == Some(index)).then(|| selection_color(settings));
        match (&item.composition, settings.geometry.kind) {
            (Some(composition), LayoutKind::Treemap) => {
                draw_directory_block(bytes, width, rect, composition, usize::from(item.depth), selection, directory_line(settings));
            }
            (None, LayoutKind::Treemap) if item.kind == NodeKind::Directory => {
                let fill = directory_background(settings, usize::from(item.depth));
                draw_framed(bytes, width, rect, fill, directory_line(settings), selection);
            }
            _ => draw_framed(bytes, width, rect, item_color(item), tile_line(settings), selection),
        }
    }
    true
}

/// The line along the right and bottom edge of every rectangle. Neighbours meet in one pixel, whatever their colours,
/// and a rectangle only a pixel wide is that line.
fn tile_line(settings: &RenderSettings) -> [u8; 4] {
    if settings.dark_theme { LINE_DARK } else { LINE_LIGHT }
}

/// What a directory draws along its own edge: the same line, or, with nested borders, a darker one that stands out from its content.
fn directory_line(settings: &RenderSettings) -> [u8; 4] {
    if settings.nested_borders {
        scale_color(tile_line(settings), NESTED_FRAME_BRIGHTNESS)
    } else {
        tile_line(settings)
    }
}

/// Colour of the outline of the selected item; the one that stands out from the background.
fn selection_color(settings: &RenderSettings) -> [u8; 4] {
    if settings.dark_theme { SELECTION_DARK_THEME } else { SELECTION_LIGHT_THEME }
}

/// The fill with a `line` along its right and bottom edge, or a double frame in `selection` colour when the item is selected.
fn draw_framed(bytes: &mut [u8], image_width: u32, rect: (u32, u32, u32, u32), color: [u8; 4], line: [u8; 4], selection: Option<[u8; 4]>) {
    let (left, top, right, bottom) = rect;
    if let Some(selection) = selection {
        fill_rect(bytes, image_width, (left, top, right, bottom), selection);
        fill_rect(bytes, image_width, (left + 2, top + 2, right.saturating_sub(2), bottom.saturating_sub(2)), color);
    } else {
        fill_rect(bytes, image_width, (left, top, right, bottom), line);
        fill_rect(bytes, image_width, (left, top, right.saturating_sub(1), bottom.saturating_sub(1)), color);
    }
}

/// Background of a directory that has its entries drawn inside it; deeper directories are a little lighter.
fn directory_background(settings: &RenderSettings, depth: usize) -> [u8; 4] {
    let background = if settings.dark_theme { BACKGROUND_DARK } else { BACKGROUND_LIGHT };
    let [red, green, blue] = FileCategory::Folder.color();
    blend(background, [red, green, blue, 255], (0.10 + depth as f32 * 0.03).min(0.5))
}

/// A directory with nothing drawn inside: bands show what kinds of files it holds (the biggest on the left),
/// and diagonal lines tell it apart from a file.
fn draw_directory_block(bytes: &mut [u8], image_width: u32, rect: (u32, u32, u32, u32), composition: &CategorySizes, depth: usize, selection: Option<[u8; 4]>, line: [u8; 4]) {
    let (left, top, right, bottom) = rect;
    let mut bands = FileCategory::ALL
        .iter()
        .zip(composition)
        .filter(|(_, size)| **size > 0)
        .map(|(category, size)| (*category, *size))
        .collect::<Vec<_>>();
    bands.sort_by_key(|(_, size)| std::cmp::Reverse(*size));
    let total = bands.iter().map(|(_, size)| *size as f64).sum::<f64>();

    let mut band_start = f64::from(left);
    for (category, size) in &bands {
        let band_end = (band_start + f64::from(right - left) * *size as f64 / total).min(f64::from(right));
        let [red, green, blue] = category.color();
        let shade = (0.78 + depth as f32 * 0.04).min(1.0);
        fill_rect(
            bytes,
            image_width,
            (band_start.round() as u32, top, band_end.round() as u32, bottom),
            scale_color([red, green, blue, 255], shade),
        );
        band_start = band_end;
    }
    draw_hatch(bytes, image_width, rect);

    if let Some(selection) = selection {
        fill_rect(bytes, image_width, (left, top, right, top + 1), selection);
        fill_rect(bytes, image_width, (left, bottom.saturating_sub(1), right, bottom), selection);
        fill_rect(bytes, image_width, (left, top, left + 1, bottom), selection);
        fill_rect(bytes, image_width, (right.saturating_sub(1), top, right, bottom), selection);
    } else {
        fill_rect(bytes, image_width, (right.saturating_sub(1), top, right, bottom), line);
        fill_rect(bytes, image_width, (left, bottom.saturating_sub(1), right, bottom), line);
    }
}

fn draw_hatch(bytes: &mut [u8], image_width: u32, rect: (u32, u32, u32, u32)) {
    let (left, top, right, bottom) = rect;
    let rows = bytes
        .chunks_exact_mut(image_width as usize * 4)
        .enumerate()
        .skip(top as usize)
        .take((bottom - top) as usize);
    for (row_index, row) in rows {
        let columns = row
            .as_chunks_mut::<4>()
            .0
            .iter_mut()
            .enumerate()
            .skip(left as usize)
            .take((right.min(image_width) - left) as usize);
        for (column_index, pixel) in columns {
            let diagonal = (column_index - left as usize + row_index - top as usize) % HATCH_SPACING_PX;
            if diagonal < HATCH_THICKNESS_PX {
                pixel.iter_mut().take(3).for_each(|channel| *channel = (f32::from(*channel) * HATCH_BRIGHTNESS) as u8);
            }
        }
    }
}

fn blend(base: [u8; 4], overlay: [u8; 4], amount: f32) -> [u8; 4] {
    let mix = |from: u8, to: u8| (f32::from(from) * (1.0 - amount) + f32::from(to) * amount) as u8;
    [mix(base[0], overlay[0]), mix(base[1], overlay[1]), mix(base[2], overlay[2]), 255]
}

/// `rect` is `(left, top, right, bottom)`, clipped to the image.
fn fill_rect(bytes: &mut [u8], image_width: u32, rect: (u32, u32, u32, u32), color: [u8; 4]) {
    let (left, top, right, bottom) = rect;
    if right <= left || bottom <= top {
        return;
    }
    let (left, columns) = (left as usize, (right.min(image_width) as usize).saturating_sub(left as usize));
    bytes
        .chunks_exact_mut(image_width as usize * 4)
        .skip(top as usize)
        .take((bottom - top) as usize)
        .flat_map(|row| row.as_chunks_mut::<4>().0.iter_mut().skip(left).take(columns))
        .for_each(|pixel| pixel.copy_from_slice(&color));
}

struct Ring {
    /// `(start_angle, end_angle, item index)`, ordered by start angle.
    arcs: Vec<(f32, f32, usize)>,
}

struct SunburstSampler {
    rings: Vec<Ring>,
    radius_px: f32,
}

impl SunburstSampler {
    fn new(layout: &Layout, levels: usize, radius_px: f32) -> Self {
        let mut rings = (0..levels).map(|_| Ring { arcs: Vec::new() }).collect::<Vec<_>>();
        for (index, item) in layout.items.iter().enumerate() {
            if let (Shape::Arc { start_angle, end_angle, .. }, Some(ring)) = (item.shape, rings.get_mut(usize::from(item.depth))) {
                ring.arcs.push((start_angle, end_angle, index));
            }
        }
        Self { rings, radius_px }
    }

    /// Depth of the ring a point lies in, with where that ring starts and ends as shares of the radius.
    fn ring_at(&self, unit_distance: f32) -> (usize, f32, f32) {
        let levels = self.rings.len();
        if levels < 2 || unit_distance < SUNBURST_CENTER_RADIUS {
            return (0, 0.0, SUNBURST_CENTER_RADIUS);
        }
        let thickness = (1.0 - SUNBURST_CENTER_RADIUS) / (levels - 1) as f32;
        let depth = (((unit_distance - SUNBURST_CENTER_RADIUS) / thickness) as usize + 1).min(levels - 1);
        (
            depth,
            SUNBURST_CENTER_RADIUS + (depth - 1) as f32 * thickness,
            SUNBURST_CENTER_RADIUS + depth as f32 * thickness,
        )
    }

    /// Item under a point given relative to the centre, in pixels; arcs are inset a little so that neighbours get a thin gap.
    /// The centre stays empty, its place is taken by text drawn over the chart.
    fn item_at(&self, dx: f32, dy: f32) -> Option<ArcHit> {
        let distance = dx.hypot(dy);
        let unit_distance = distance / self.radius_px;
        if unit_distance >= 1.0 {
            return None;
        }
        let (depth, ring_start, ring_end) = self.ring_at(unit_distance);
        if depth == 0 {
            return None;
        }
        let from_ring_edge = ((unit_distance - ring_start).min(ring_end - unit_distance)) * self.radius_px;
        if from_ring_edge < GAP_PX / 2.0 {
            return None;
        }
        let angle = dx.atan2(-dy).rem_euclid(TAU);
        let ring = self.rings.get(depth)?;
        let candidate = ring.arcs.partition_point(|(start, _, _)| *start <= angle).checked_sub(1)?;
        let &(start, end, item_index) = ring.arcs.get(candidate)?;
        let (from_start, to_end) = ((angle - start) * distance, (end - angle) * distance);
        let inside = angle < end && from_start >= GAP_PX / 2.0 && to_end >= GAP_PX / 2.0;
        let near_edge = from_ring_edge.min(from_start).min(to_end) < OUTLINE_PX;
        inside.then_some(ArcHit { index: item_index, near_edge })
    }
}

/// An item of the sunburst under a point, and whether the point is close to the edge of its arc.
#[derive(Clone, Copy, PartialEq)]
struct ArcHit {
    index: usize,
    near_edge: bool,
}

/// Offsets of the samples used to estimate how much of a pixel an item covers, in pixels from its centre.
const CORNER_OFFSET: f32 = 0.35;
const SUPERSAMPLE_STEPS: u32 = 4;
const GAP_PX: f32 = 1.0;
/// Width of the white outline of the selected arc.
const OUTLINE_PX: f32 = 2.0;

fn draw_sunburst(bytes: &mut [u8], layout: &Layout, settings: &RenderSettings, is_current: &dyn Fn() -> bool) -> bool {
    let geometry = settings.geometry;
    let (left, top, side, _) = geometry.drawing_area();
    let radius_px = side as f32 / 2.0;
    let sampler = SunburstSampler::new(layout, settings.levels.max(1), radius_px);
    let center = (left as f32 + radius_px, top as f32 + radius_px);
    let image_width = geometry.width as usize;
    let colors = |hit: ArcHit| {
        let item = layout.items.get(hit.index)?;
        Some(if hit.near_edge && settings.selected == Some(hit.index) {
            selection_color(settings)
        } else {
            item_color(item)
        })
    };

    let rows = bytes.chunks_exact_mut(image_width * 4).enumerate().skip(top as usize).take(side as usize);
    for (row_index, row) in rows {
        if row_index % 16 == 0 && !is_current() {
            return false;
        }
        let dy = row_index as f32 + 0.5 - center.1;
        for (column_index, pixel) in row.as_chunks_mut::<4>().0.iter_mut().enumerate().skip(left as usize).take(side as usize) {
            let dx = column_index as f32 + 0.5 - center.0;
            if let Some(color) = antialiased_pixel(&sampler, (dx, dy), &colors) {
                pixel.copy_from_slice(&color);
            }
        }
    }
    true
}

/// `None` means nothing is drawn in the pixel. On an edge the pixel keeps the colour of what covers it and gets transparent
/// in proportion to how much of it is empty, so it blends with the background of the window in any theme.
fn antialiased_pixel(sampler: &SunburstSampler, (dx, dy): (f32, f32), colors: &dyn Fn(ArcHit) -> Option<[u8; 4]>) -> Option<[u8; 4]> {
    let center = sampler.item_at(dx, dy);
    let corners_agree = [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .iter()
        .all(|(x, y)| sampler.item_at(dx + x * CORNER_OFFSET, dy + y * CORNER_OFFSET) == center);
    if corners_agree {
        return center.and_then(colors);
    }

    let samples = SUPERSAMPLE_STEPS * SUPERSAMPLE_STEPS;
    let mut sum = [0_u32; 3];
    let mut covered = 0_u32;
    for step_y in 0..SUPERSAMPLE_STEPS {
        for step_x in 0..SUPERSAMPLE_STEPS {
            let offset = |step: u32| (step as f32 + 0.5) / SUPERSAMPLE_STEPS as f32 - 0.5;
            if let Some(color) = sampler.item_at(dx + offset(step_x), dy + offset(step_y)).and_then(colors) {
                sum.iter_mut().zip(color).for_each(|(total, channel)| *total += u32::from(channel));
                covered += 1;
            }
        }
    }
    if covered == 0 {
        return None;
    }
    let [red, green, blue] = sum.map(|total| (total / covered) as u8);
    Some([red, green, blue, (covered * 255 / samples) as u8])
}

fn item_color(item: &LayoutItem) -> [u8; 4] {
    let [red, green, blue] = match item.kind {
        NodeKind::File => item.category.color(),
        NodeKind::Directory => item.category.color().map(|channel| (f32::from(channel) * DIRECTORY_BRIGHTNESS) as u8),
        NodeKind::Symlink | NodeKind::Inaccessible => NEUTRAL_ITEM,
    };
    [red, green, blue, 255]
}

fn scale_color(color: [u8; 4], factor: f32) -> [u8; 4] {
    let [red, green, blue, alpha] = color;
    [(f32::from(red) * factor) as u8, (f32::from(green) * factor) as u8, (f32::from(blue) * factor) as u8, alpha]
}

#[cfg(test)]
mod tests {
    use czkawka_core::tools::disk_usage::category::FileCategory;
    use czkawka_core::tools::disk_usage::layout::{Layout, LayoutKind, LayoutOptions, Shape};
    use czkawka_core::tools::disk_usage::tree::{DiskNode, DiskTree};

    use super::{ChartGeometry, RenderSettings, render_chart};

    fn sample_layout(kind: LayoutKind) -> Layout {
        let root = DiskNode::directory(
            "/root",
            vec![
                DiskNode::file("song.mp3", 300, FileCategory::Audio),
                DiskNode::file("program.bin", 100, FileCategory::Binary),
            ],
        );
        let options = LayoutOptions {
            min_size_fraction: 0.0,
            ..LayoutOptions::new(kind)
        };
        Layout::build(&DiskTree::new(vec![root]), &options)
    }

    fn settings(width: u32, height: u32, kind: LayoutKind, levels: usize) -> RenderSettings {
        RenderSettings {
            geometry: ChartGeometry { width, height, kind },
            levels,
            selected: None,
            dark_theme: true,
            nested_borders: false,
        }
    }

    fn pixel(bytes: &[u8], width: u32, x: u32, y: u32) -> [u8; 4] {
        let offset = ((y * width + x) * 4) as usize;
        [bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]
    }

    #[test]
    fn image_has_requested_size() {
        let layout = sample_layout(LayoutKind::Treemap);
        let buffer = render_chart(&layout, &settings(120, 80, LayoutKind::Treemap, usize::MAX), &|| true).unwrap();

        assert_eq!((buffer.width(), buffer.height()), (120, 80));
    }

    #[test]
    fn middle_of_a_file_has_the_colour_of_its_category() {
        let layout = sample_layout(LayoutKind::Treemap);
        let buffer = render_chart(&layout, &settings(100, 100, LayoutKind::Treemap, usize::MAX), &|| true).unwrap();

        let audio = layout.items.iter().find(|item| item.category == FileCategory::Audio && item.depth == 1).unwrap();
        let Shape::Rect { x, y, width, height } = audio.shape else {
            panic!("treemap uses rectangles")
        };
        let [red, green, blue] = FileCategory::Audio.color();
        let middle = pixel(buffer.as_bytes(), 100, ((x + width / 2.0) * 100.0) as u32, ((y + height / 2.0) * 100.0) as u32);
        assert_eq!(middle, [red, green, blue, 255]);
    }

    #[test]
    fn rectangles_are_separated_by_a_one_pixel_line_of_one_colour() {
        let layout = sample_layout(LayoutKind::Treemap);
        let audio = layout.items.iter().find(|item| item.category == FileCategory::Audio && item.depth == 1).unwrap();
        let Shape::Rect { x, y, width, height } = audio.shape else {
            panic!("treemap uses rectangles")
        };
        let (right, middle) = (((x + width) * 100.0).round() as u32, ((y + height / 2.0) * 100.0) as u32);
        let [red, green, blue] = FileCategory::Audio.color();

        let buffer = render_chart(&layout, &settings(100, 100, LayoutKind::Treemap, usize::MAX), &|| true).unwrap();

        assert_eq!(
            pixel(buffer.as_bytes(), 100, right - 1, middle),
            super::LINE_DARK,
            "the last column of the rectangle is the line"
        );
        assert_eq!(pixel(buffer.as_bytes(), 100, right - 2, middle), [red, green, blue, 255], "the line is a single pixel");
        assert_ne!(pixel(buffer.as_bytes(), 100, right, middle), super::LINE_DARK, "the next rectangle starts right after it");
    }

    #[test]
    fn nested_borders_make_the_line_around_a_directory_darker() {
        let root = DiskNode::directory(
            "/root",
            vec![
                DiskNode::file("song.mp3", 300, FileCategory::Audio),
                DiskNode::file("program.bin", 100, FileCategory::Binary),
            ],
        );
        let options = LayoutOptions {
            min_size_fraction: 0.0,
            nested_padding: 0.05,
            ..LayoutOptions::new(LayoutKind::Treemap)
        };
        let layout = Layout::build(&DiskTree::new(vec![root]), &options);
        let edge = |nested_borders: bool| {
            let mut settings = settings(100, 100, LayoutKind::Treemap, usize::MAX);
            settings.nested_borders = nested_borders;
            let buffer = render_chart(&layout, &settings, &|| true).unwrap();
            pixel(buffer.as_bytes(), 100, 99, 50)
        };

        assert_eq!(edge(false), super::LINE_DARK);
        assert!(
            edge(true)[0] < super::LINE_DARK[0],
            "the frame of the directory is darker than the lines between rectangles"
        );
    }

    #[test]
    fn rectangle_one_pixel_wide_is_only_the_line() {
        let files = (0..6)
            .map(|index| DiskNode::file(format!("f{index}"), if index == 0 { 10_000 } else { 1 }, FileCategory::Audio))
            .collect();
        let tree = DiskTree::new(vec![DiskNode::directory("/root", files)]);
        let options = LayoutOptions {
            min_size_fraction: 0.0,
            nested_padding: 0.0,
            ..LayoutOptions::new(LayoutKind::Treemap)
        };
        let layout = Layout::build(&tree, &options);

        let buffer = render_chart(&layout, &settings(60, 60, LayoutKind::Treemap, usize::MAX), &|| true).unwrap();

        let mut line_pixels = 0;
        for x in 0..60 {
            line_pixels += usize::from(pixel(buffer.as_bytes(), 60, x, 59) == super::LINE_DARK);
        }
        assert!(line_pixels > 0, "tiny rectangles still leave their line");
    }

    #[test]
    fn stale_request_is_dropped() {
        let layout = sample_layout(LayoutKind::Treemap);

        assert!(render_chart(&layout, &settings(100, 100, LayoutKind::Treemap, 4), &|| false).is_none());
    }

    #[test]
    fn sunburst_is_a_centred_circle_with_background_in_the_corners() {
        let layout = sample_layout(LayoutKind::Sunburst);
        let buffer = render_chart(&layout, &settings(200, 100, LayoutKind::Sunburst, 4), &|| true).unwrap();

        let corner = pixel(buffer.as_bytes(), 200, 2, 2);
        assert_eq!(corner[3], 0, "nothing is drawn outside the circle, so the window shows through");
        let ring_pixel = pixel(buffer.as_bytes(), 200, 110, 50 - 26);
        assert_eq!(ring_pixel[3], 255, "a pixel in the middle of the first ring is fully covered");
    }

    #[test]
    fn positions_outside_the_sunburst_square_are_not_on_the_chart() {
        let geometry = ChartGeometry {
            width: 200,
            height: 100,
            kind: LayoutKind::Sunburst,
        };

        assert_eq!(geometry.to_unit(0.5, 0.5), Some((0.5, 0.5)));
        assert_eq!(geometry.to_unit(0.05, 0.5), None);
        let flat = ChartGeometry {
            kind: LayoutKind::Treemap,
            ..geometry
        };
        assert_eq!(flat.to_unit(0.05, 0.5), Some((0.05, 0.5)));
    }
}
