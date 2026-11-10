use std::f32::consts::TAU;

use crate::tools::disk_usage::category::{CATEGORY_COUNT, FileCategory};
use crate::tools::disk_usage::layout::{Layout, LayoutItem, LayoutKind, LayoutOptions, SUNBURST_CENTER_RADIUS, Shape};
use crate::tools::disk_usage::tree::{DiskNode, DiskTree, NodeKind};

fn sample_tree() -> DiskTree {
    let music = DiskNode::directory(
        "music",
        vec![DiskNode::file("a.mp3", 300, FileCategory::Audio), DiskNode::file("b.mp3", 100, FileCategory::Audio)],
    );
    let docs = DiskNode::directory("docs", vec![DiskNode::file("c.pdf", 200, FileCategory::Document)]);
    let root = DiskNode::directory("/root", vec![music, docs, DiskNode::file("d.bin", 400, FileCategory::Binary)]);
    DiskTree::new(vec![root])
}

fn options(kind: LayoutKind) -> LayoutOptions {
    LayoutOptions {
        min_size_fraction: 0.0,
        ..LayoutOptions::new(kind)
    }
}

fn rect_area(item: &LayoutItem) -> f32 {
    match item.shape {
        Shape::Rect { width, height, .. } => width * height,
        Shape::Arc { .. } => panic!("not a rectangle"),
    }
}

fn items_at_depth(layout: &Layout, depth: u16) -> Vec<&LayoutItem> {
    layout.items.iter().filter(|item| item.depth == depth).collect()
}

fn rects_overlap(left: &Shape, right: &Shape) -> bool {
    let (
        Shape::Rect {
            x: lx,
            y: ly,
            width: lw,
            height: lh,
        },
        Shape::Rect {
            x: rx,
            y: ry,
            width: rw,
            height: rh,
        },
    ) = (left, right)
    else {
        panic!("not rectangles");
    };
    const EPSILON: f32 = 1e-5;
    lx + EPSILON < rx + rw && rx + EPSILON < lx + lw && ly + EPSILON < ry + rh && ry + EPSILON < ly + lh
}

#[test]
fn treemap_areas_are_proportional_to_sizes() {
    let layout = Layout::build(&sample_tree(), &options(LayoutKind::Treemap));

    let root = &layout.items[0];
    assert!((rect_area(root) - 1.0).abs() < 1e-4);
    let level_one = items_at_depth(&layout, 1);
    assert_eq!(level_one.len(), 3);
    let covered = level_one.iter().map(|item| rect_area(item)).sum::<f32>();
    let inner_area = (1.0_f32 - 2.0 * 0.002).powi(2);
    assert!((covered - inner_area).abs() < 1e-3, "children cover {covered}, expected {inner_area}");
    let (music, binary) = (
        level_one.iter().find(|i| i.size == 400 && i.category == FileCategory::Audio).unwrap(),
        level_one.iter().find(|i| i.size == 400 && i.category == FileCategory::Binary).unwrap(),
    );
    assert!((rect_area(music) - rect_area(binary)).abs() < 1e-4);
}

#[test]
fn treemap_siblings_do_not_overlap() {
    let layout = Layout::build(&sample_tree(), &options(LayoutKind::Treemap));

    for depth in 0..3 {
        let level = items_at_depth(&layout, depth);
        for (position, left) in level.iter().enumerate() {
            for right in &level[position + 1..] {
                if left.parent == right.parent {
                    assert!(!rects_overlap(&left.shape, &right.shape), "{left:?} overlaps {right:?}");
                }
            }
        }
    }
}

#[test]
fn treemap_respects_max_depth_and_parents_come_first() {
    let mut limited = options(LayoutKind::Treemap);
    limited.max_depth = 2;
    let layout = Layout::build(&sample_tree(), &limited);

    assert!(layout.items.iter().all(|item| item.depth < 2));
    for (index, item) in layout.items.iter().enumerate() {
        assert!(item.parent.is_none_or(|parent| (parent as usize) < index));
    }
}

#[test]
fn min_size_fraction_drops_small_entries() {
    let mut strict = options(LayoutKind::Treemap);
    strict.min_size_fraction = 0.25;
    let layout = Layout::build(&sample_tree(), &strict);

    assert!(
        layout.items.iter().filter(|item| item.depth == 1 && !item.is_omitted_block()).all(|item| item.size >= 250),
        "{:?}",
        layout.items
    );
    assert!(layout.items.iter().any(|item| item.is_omitted_block()), "what was left out is still shown as one block");
}

#[test]
fn disabled_category_is_not_measured() {
    let mut without_audio = options(LayoutKind::Treemap);
    without_audio.enabled_categories[FileCategory::Audio.index()] = false;
    let layout = Layout::build(&sample_tree(), &without_audio);

    assert_eq!(layout.items[0].size, 600);
    assert!(layout.items.iter().all(|item| item.category != FileCategory::Audio));
    assert_eq!(without_audio.enabled_categories.len(), CATEGORY_COUNT);
}

#[test]
fn sunburst_arcs_cover_the_circle_in_proportion() {
    let layout = Layout::build(&sample_tree(), &options(LayoutKind::Sunburst));

    let Shape::Arc {
        start_angle,
        end_angle,
        inner_radius,
        outer_radius,
    } = layout.items[0].shape
    else {
        panic!("sunburst must use arcs")
    };
    assert_eq!((start_angle, end_angle, inner_radius), (0.0, TAU, 0.0));
    assert!((outer_radius - SUNBURST_CENTER_RADIUS).abs() < 1e-6, "the root is the centre");
    let Shape::Arc {
        inner_radius: first_ring_start,
        outer_radius: first_ring_end,
        ..
    } = items_at_depth(&layout, 1)[0].shape
    else {
        panic!()
    };
    assert!((first_ring_start - SUNBURST_CENTER_RADIUS).abs() < 1e-6, "the first ring starts where the centre ends");
    assert!(
        (first_ring_end - first_ring_start - (1.0 - SUNBURST_CENTER_RADIUS) / 3.0).abs() < 1e-6,
        "three rings share the rest"
    );
    let level_one = items_at_depth(&layout, 1);
    let swept = level_one
        .iter()
        .map(|item| match item.shape {
            Shape::Arc { start_angle, end_angle, .. } => end_angle - start_angle,
            Shape::Rect { .. } => panic!(),
        })
        .sum::<f32>();
    assert!((swept - TAU).abs() < 1e-4);
}

#[test]
fn hit_test_finds_deepest_item_and_address_leads_back_to_node() {
    let tree = sample_tree();
    let layout = Layout::build(&tree, &options(LayoutKind::Treemap));

    let music = tree.roots[0].children.iter().position(|child| &*child.name == "music").unwrap() as u32;
    let address = [0, music, 0];
    assert_eq!(&*tree.node(&address).unwrap().name, "a.mp3");
    let item = layout.find_item(&address).unwrap();
    let Shape::Rect { x, y, width, height } = layout.items[item].shape else {
        panic!("the treemap uses rectangles")
    };

    let hit = layout.hit_test(x + width / 2.0, y + height / 2.0).unwrap();
    assert_eq!(hit, item);
    assert_eq!(layout.address(hit).unwrap(), address);
}

#[test]
fn hit_test_works_for_arcs() {
    let layout = Layout::build(&sample_tree(), &options(LayoutKind::Sunburst));

    let centre = layout.hit_test(0.5, 0.5).unwrap();
    assert_eq!(layout.items[centre].depth, 0);
    // Just above the centre (angle ~ 0), in the middle of the first ring: first child, the biggest entry of the root
    let middle_of_first_ring = SUNBURST_CENTER_RADIUS + (1.0 - SUNBURST_CENTER_RADIUS) / 6.0;
    let first_child = layout.hit_test(0.5, 0.5 - middle_of_first_ring / 2.0).unwrap();
    assert_eq!(layout.items[first_child].depth, 1);
    assert_eq!(layout.items[first_child].size, 400);
}

#[test]
fn treemap_leaves_out_rectangles_thinner_than_minimum_size() {
    let big = DiskNode::file("big", 1_000_000, FileCategory::Other);
    let tiny = DiskNode::file("tiny", 1, FileCategory::Other);
    let tree = DiskTree::new(vec![DiskNode::directory("/root", vec![big, tiny])]);
    let mut options = options(LayoutKind::Treemap);
    options.canvas_size_px = (400.0, 300.0);
    options.min_item_size_px = 3.0;

    let layout = Layout::build(&tree, &options);

    assert_eq!(
        layout.items.iter().filter(|item| item.depth == 1).count(),
        1,
        "the tiny file is too small even as a block of its own"
    );
    assert!(layout.items.iter().all(|item| !item.is_omitted_block()));
}

#[test]
fn treemap_rectangles_use_the_canvas_aspect_ratio() {
    let files = ["a", "b", "c", "d"].map(|name| DiskNode::file(name, 10, FileCategory::Other));
    let tree = DiskTree::new(vec![DiskNode::directory("/root", files.to_vec())]);
    let mut options = options(LayoutKind::Treemap);
    options.canvas_size_px = (800.0, 200.0);

    let layout = Layout::build(&tree, &options);

    let Shape::Rect { width, height, .. } = layout.items[1].shape else { panic!() };
    let pixel_ratio = (width * 800.0) / (height * 200.0);
    assert!((0.8..1.25).contains(&pixel_ratio), "files are close to squares in pixels, ratio {pixel_ratio}");
}

#[test]
fn sunburst_leaves_out_arcs_narrower_than_minimum_angle() {
    let big = DiskNode::file("big", 1_000_000, FileCategory::Other);
    let small = DiskNode::file("small", 1_000, FileCategory::Other);
    let tree = DiskTree::new(vec![DiskNode::directory("/root", vec![big, small])]);
    let mut options = options(LayoutKind::Sunburst);
    options.min_arc_angle = 0.03;

    let layout = Layout::build(&tree, &options);

    assert_eq!(layout.items.iter().filter(|item| item.depth == 1).count(), 1, "0.1% of the circle is 0.006 rad");
}

fn many_equal_files(count: usize) -> DiskTree {
    let files = (0..count).map(|index| DiskNode::file(format!("file_{index:03}"), 100, FileCategory::Other)).collect();
    DiskTree::new(vec![DiskNode::directory("/root", files)])
}

#[test]
fn treemap_keeps_many_equal_items_close_to_squares() {
    let mut options = options(LayoutKind::Treemap);
    options.canvas_size_px = (800.0, 500.0);

    let layout = Layout::build(&many_equal_files(60), &options);

    let ratios = layout
        .items
        .iter()
        .filter(|item| item.depth == 1)
        .map(|item| {
            let Shape::Rect { width, height, .. } = item.shape else { panic!() };
            let (width_px, height_px) = (width * 800.0, height * 500.0);
            width_px.max(height_px) / width_px.min(height_px)
        })
        .collect::<Vec<_>>();
    assert_eq!(ratios.len(), 60);
    let worst = ratios.iter().copied().fold(0.0_f32, f32::max);
    assert!(worst < 2.5, "no long strips among equal items, worst ratio {worst}");
}

#[test]
fn entries_too_small_to_see_become_one_block_that_keeps_their_area() {
    let mut files = vec![DiskNode::file("big", 38_000, FileCategory::Other)];
    files.extend((0..2_000).map(|index| DiskNode::file(format!("small_{index:04}"), 1, FileCategory::Other)));
    let tree = DiskTree::new(vec![DiskNode::directory("/root", files)]);
    let mut options = options(LayoutKind::Treemap);
    options.canvas_size_px = (400.0, 300.0);
    options.min_item_size_px = 3.0;

    let layout = Layout::build(&tree, &options);

    let level_one = layout.items.iter().filter(|item| item.depth == 1).collect::<Vec<_>>();
    assert_eq!(level_one.len(), 2, "the big file and one block of the rest");
    let block = level_one.iter().find(|item| item.is_omitted_block()).unwrap();
    assert_eq!((block.omitted_entries, block.size), (2_000, 2_000));
    let block_index = layout.items.iter().position(|item| item.is_omitted_block()).unwrap();
    assert_eq!(layout.address(block_index).unwrap(), [0], "it points at the directory that holds the entries");
    assert_eq!(layout.find_item(&[0, u32::MAX]), None);
}

#[test]
fn directory_whose_content_is_all_too_small_is_a_block_with_its_composition() {
    let dust = (0..10).map(|index| DiskNode::file(format!("dust_{index}"), 1, FileCategory::Audio)).collect();
    let dusty = DiskNode::directory("dusty", dust);
    let big = DiskNode::file("big", 1_000_000, FileCategory::Binary);
    let tree = DiskTree::new(vec![DiskNode::directory("/root", vec![dusty, big])]);
    let mut options = options(LayoutKind::Treemap);
    options.min_size_fraction = 0.0;
    options.canvas_size_px = (400.0, 300.0);
    options.min_item_size_px = 3.0;
    let tree_with_big_dir = DiskTree::new(vec![DiskNode::directory(
        "/root",
        vec![
            DiskNode::directory("dusty", (0..10).map(|index| DiskNode::file(format!("dust_{index}"), 1_000, FileCategory::Audio)).collect()),
            DiskNode::file("big", 1_000_000, FileCategory::Binary),
        ],
    )]);

    let layout = Layout::build(&tree_with_big_dir, &options);

    let directory = layout.items.iter().find(|item| item.kind == NodeKind::Directory && item.depth == 1).unwrap();
    let composition = directory.composition.as_ref().expect("nothing fits inside, so the composition is kept for drawing");
    assert_eq!(composition[FileCategory::Audio.index()], 10_000);
    assert_eq!(layout.items.iter().filter(|item| item.parent == Some(1)).count(), 0);
    drop(tree);
}

#[test]
fn equal_sizes_keep_the_same_order_after_a_rescan() {
    let build = |names: [&str; 3]| {
        let files = names.map(|name| DiskNode::file(name, 10, FileCategory::Other)).to_vec();
        DiskTree::new(vec![DiskNode::directory("/root", files)])
    };

    let first = build(["b", "c", "a"]);
    let second = build(["c", "a", "b"]);

    let names = |tree: &DiskTree| tree.roots[0].children.iter().map(|child| child.name.to_string()).collect::<Vec<_>>();
    assert_eq!(names(&first), ["a", "b", "c"]);
    assert_eq!(names(&first), names(&second));
}

#[test]
fn entries_are_grouped_by_their_share_of_the_folder_not_of_the_whole_scan() {
    let inner = DiskNode::directory(
        "inner",
        vec![DiskNode::file("x", 15_000, FileCategory::Other), DiskNode::file("y", 5_000, FileCategory::Other)],
    );
    let tree = DiskTree::new(vec![DiskNode::directory("/root", vec![DiskNode::file("huge", 10_000_000, FileCategory::Other), inner])]);
    let mut options = options(LayoutKind::Treemap);
    options.min_size_fraction = 0.001;
    options.nested_padding = 0.0;

    let layout = Layout::build(&tree, &options);

    let inside_inner = layout.items.iter().filter(|item| item.depth == 2).collect::<Vec<_>>();
    assert_eq!(inside_inner.len(), 2, "y is only 0.05% of the scan but 25% of its folder");
    assert!(inside_inner.iter().all(|item| !item.is_omitted_block()));
}

#[test]
fn directories_smaller_than_the_limit_are_drawn_as_one_block() {
    let files = (0..4).map(|index| DiskNode::file(format!("f{index}"), 100, FileCategory::Audio)).collect();
    let tree = DiskTree::new(vec![DiskNode::directory(
        "/root",
        vec![DiskNode::directory("small", files), DiskNode::file("big", 100_000, FileCategory::Other)],
    )]);
    let mut options = options(LayoutKind::Treemap);
    options.canvas_size_px = (400.0, 300.0);
    let rendered = |limit: f32| {
        let mut options = options;
        options.min_expanded_directory_px = limit;
        Layout::build(&tree, &options)
    };

    let always_open = rendered(0.0);
    let closed = rendered(60.0);

    let small = |layout: &Layout| layout.items.iter().position(|item| item.depth == 1 && item.kind == NodeKind::Directory).unwrap();
    assert!(always_open.items.iter().any(|item| item.parent == Some(small(&always_open) as u32)));
    assert!(closed.items.iter().all(|item| item.parent != Some(small(&closed) as u32)));
    assert!(closed.items[small(&closed)].composition.is_some());
}

#[test]
fn nested_padding_can_be_turned_off_so_content_fills_its_directory() {
    let tree = sample_tree();
    let mut flat = options(LayoutKind::Treemap);
    flat.nested_padding = 0.0;

    let layout = Layout::build(&tree, &flat);

    let Shape::Rect { x, y, width, height } = layout.items[1].shape else { panic!() };
    let level_one_area = layout
        .items
        .iter()
        .filter(|item| item.depth == 1)
        .map(|item| match item.shape {
            Shape::Rect { width, height, .. } => width * height,
            Shape::Arc { .. } => 0.0,
        })
        .sum::<f32>();
    assert!((level_one_area - 1.0).abs() < 1e-4, "children cover the whole root, got {level_one_area}");
    assert!(x >= 0.0 && y >= 0.0 && width > 0.0 && height > 0.0);
}

#[test]
fn deeply_nested_tree_does_not_overflow_the_stack() {
    const LEVELS: usize = 2000;
    let mut node = DiskNode::file("leaf", 1, FileCategory::Other);
    for level in 0..LEVELS {
        node = DiskNode::directory(format!("d{level}"), vec![DiskNode::file(format!("f{level}"), 1, FileCategory::Other), node]);
    }
    let options = LayoutOptions {
        nested_padding: 0.0,
        min_size_fraction: 0.0,
        ..LayoutOptions::new(LayoutKind::Treemap)
    };

    let layout = Layout::build(&DiskTree::new(vec![node]), &options);

    assert_eq!(layout.items.iter().map(|item| usize::from(item.depth)).max(), Some(LEVELS));
    assert!(
        layout
            .items
            .iter()
            .skip(1)
            .all(|item| item.parent.is_some_and(|parent| (parent as usize) < layout.items.len()))
    );
}

/// Nested directories with entries of very different sizes, so that blocks of omitted entries and empty gaps appear.
fn varied_tree() -> DiskTree {
    let mut seed = 7_u64;
    let mut next = move |limit: u64| {
        seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        (seed >> 33) % limit + 1
    };
    let roots = (0..2)
        .map(|root| {
            let directories = (0..8)
                .map(|directory| {
                    let inner = DiskNode::directory(
                        format!("inner{directory}"),
                        (0..12).map(|file| DiskNode::file(format!("i{file}"), next(2000).pow(2), FileCategory::Video)).collect(),
                    );
                    let files = (0..20).map(|file| DiskNode::file(format!("f{file}"), next(50_000), FileCategory::Audio));
                    DiskNode::directory(format!("d{directory}"), files.chain([inner]).collect())
                })
                .collect();
            DiskNode::directory(format!("/root{root}"), directories)
        })
        .collect();
    DiskTree::new(roots)
}

fn linear_hit_test(layout: &Layout, x: f32, y: f32) -> Option<usize> {
    layout.items.iter().rposition(|item| item.shape.contains(x, y))
}

fn linear_find_item(layout: &Layout, address: &[u32]) -> Option<usize> {
    let (&root, rest) = address.split_first()?;
    let mut current = layout
        .items
        .iter()
        .position(|item| item.parent.is_none() && !item.is_omitted_block() && item.index_in_parent == root)?;
    for &child in rest {
        let parent = u32::try_from(current).ok()?;
        current = layout
            .items
            .iter()
            .position(|item| item.parent == Some(parent) && !item.is_omitted_block() && item.index_in_parent == child)?;
    }
    Some(current)
}

fn varied_layouts() -> Vec<Layout> {
    let tree = varied_tree();
    [LayoutKind::Treemap, LayoutKind::Sunburst]
        .into_iter()
        .flat_map(|kind| {
            [0.0, 0.01].map(|padding| LayoutOptions {
                min_size_fraction: 0.005,
                nested_padding: padding,
                max_depth: 4,
                min_arc_angle: 0.02,
                ..LayoutOptions::new(kind)
            })
        })
        .map(|options| Layout::build(&tree, &options))
        .collect()
}

#[test]
fn hit_test_going_down_the_tree_gives_the_same_item_as_looking_at_every_item() {
    for layout in varied_layouts() {
        assert!(layout.items.iter().any(LayoutItem::is_omitted_block), "the layout should have blocks of omitted entries");
        for row in 0..=240 {
            for column in 0..=240 {
                let (x, y) = (column as f32 / 240.0, row as f32 / 240.0);
                assert_eq!(layout.hit_test(x, y), linear_hit_test(&layout, x, y), "point {x},{y}");
            }
        }
        for item in layout.items.iter().filter(|item| item.parent.is_none()) {
            if let Shape::Rect { x, y, width, height } = item.shape {
                for (px, py) in [(x, y), (x + width, y), (x, y + height), (x + width, y + height), (x + width / 2.0, y)] {
                    assert_eq!(layout.hit_test(px, py), linear_hit_test(&layout, px, py), "edge point {px},{py}");
                }
            }
        }
    }
}

#[test]
fn find_item_going_down_the_tree_gives_the_same_item_as_looking_at_every_item() {
    for layout in varied_layouts() {
        for (index, item) in layout.items.iter().enumerate().filter(|(_, item)| !item.is_omitted_block()) {
            let address = layout.address(index).unwrap();
            assert_eq!(layout.find_item(&address), Some(index));
            assert_eq!(layout.find_item(&address), linear_find_item(&layout, &address), "{:?}", item.shape);
        }
        assert_eq!(layout.find_item(&[]), None);
        assert_eq!(layout.find_item(&[0, u32::MAX]), None);
    }
}
