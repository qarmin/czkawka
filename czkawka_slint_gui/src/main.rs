fn main() {}

slint::slint! {
import { Button, VerticalBox , HorizontalBox, TabWidget, ListView, StandardListView, StandardTableView, CheckBox} from "std-widgets.slint";

component SelectableTableView inherits Rectangle {
    in property <[string]> columns;
    in property <[[string]]> values;

    private property <[length]> column_sizes: [30px, 100px, 100px, 100px];
    private property <int> column_number: 4;

    VerticalBox {
        padding: 5px;
        // Widgets
        HorizontalLayout {
            padding: 5px; spacing: 5px;
            vertical-stretch: 0;
            for title[idx] in root.columns : HorizontalLayout {
                width: root.column_sizes[idx];
                Text { overflow: elide; text: title; }
                Rectangle {
                    width: 1px;
                    background: gray;

                    TouchArea {
                        width: 5px;
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
        list_view:= ListView {
            for r[idx] in root.values : Rectangle {
                private property <bool> selected: false;
                background: touch-area.has-hover ? (selected ? #333333 : #222222) : (selected ? #333333: #222222);

                touch_area:= TouchArea {
                    clicked => {
                        parent.selected = !parent.selected
                    }
                }

                HorizontalLayout {
                    padding: 5px;
                    spacing: 5px;
                    //width: root.column_sizes[idx];

                    CheckBox {
                        //min-width: 200px;
                        width: root.column-sizes[0];
                    }

                    HorizontalLayout {
                        padding: 5px;
                        spacing: 5px;
                        for f[idx] in r : Text {
                            width: root.column-sizes[idx + 1];
                            text: f;
                            vertical-alignment: center;

                            overflow: elide;
                        }
                    }
                }
            }
        }
    }
}

export component MainWindow {
    in-out property <int> active-tab;
    VerticalBox {
        HorizontalBox {
            width: 600px;
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

            // TODO - using root.active-tab in visible property will not
            if root.active-tab == 0: SelectableTableView {
                columns: ["Selection", "Folder Name", "Path", "Modification Date"];
                values: [
                        ["kropkarz", "/Xd1", "24.10.2023"] ,
                        ["witasphere", "/Xd1/Imagerren2", "25.11.1991"] ,
                        ["lokkaler", "/Xd1/Vide2", "01.23.1911"] ,
                ];
            }
        }
        HorizontalBox {
            scan_button:= Button {
                text: "Scan";
            }
            delete_button:= Button {
                text: "Delete";
            }
        }
    }
}




}
