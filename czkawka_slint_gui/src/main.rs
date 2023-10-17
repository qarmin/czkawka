fn main() {}

slint::slint! {
import { Button, VerticalBox , HorizontalBox, TabWidget, ListView, StandardListView, StandardTableView} from "std-widgets.slint";

component CzkawkaTableView inherits Rectangle {
    in property <[string]> columns;
    in property <[[string]]> values;

    private property <[length]> column_sizes: [20px, 100px, 50px, 200px];
    private property <int> column_number: 3;

    VerticalBox {
        padding: 5px;
        HorizontalLayout {
            padding: 5px; spacing: 5px;
            vertical-stretch: 0;
            for title[idx] in root.columns : HorizontalLayout {
                width: root.column_sizes[idx];
                Text { overflow: elide; text: idx; }
                Rectangle {
                    width: 1px;
                    background: gray;
                    TouchArea {
                        width: 10px;
                        x: (parent.width - self.width) / 2;
                        property <length> cached;
                        pointer-event(event) => {
                            if (event.button == PointerEventButton.left && event.kind == PointerEventKind.down) {
                                self.cached = root.column_sizes[idx];
                            }
                        }
                        moved => {
                            if (self.pressed) {
                                root.column_sizes[idx] += (self.mouse-x - self.pressed-x);
                                if (root.column_sizes[idx] < 0) {
                                    root.column_sizes[idx] = 0;
                                }
                            }
                        }
                        mouse-cursor: ew-resize;
                    }
                }
            }
        }
        ListView {
            for r in root.values : HorizontalLayout {
                padding: 5px;
                spacing: 5px;
                for t[idx] in r : HorizontalLayout {
                    width: root.column_sizes[idx];
                    Text { overflow: elide; text: t; }
                }
            }
        }
    }
}

export component MainWindow {
    in-out property <int> active-tab;
    VerticalBox {
        HorizontalBox {
            width: 400px;
            preferred-height: 300px;

            tab_bar := VerticalLayout {
                width: 120px;
                spacing: 3px;
                Button {
                    text: "Empty Folders";
                    clicked => { root.active-tab = 0; }
                }
                Button {
                    text: "Similar Images";
                    clicked => { root.active-tab = 1; }
                }
            }

            CzkawkaTableView {
                columns: ["Device", "Mount Point", "Total", "Free"];
                values: [
                        ["/dev/sda1", "/", "255GB", "82.2GB"] ,
                        ["/dev/sda2", "/tmp", "60.5GB", "44.5GB"] ,
                        ["/dev/sdb1", "/home", "255GB", "32.2GB"] ,
                ];
            }
        }
        HorizontalBox {
            Button {
                text: "Scan";
            }
            Button {
                text: "Delete";
            }
        }
    }
}

}
