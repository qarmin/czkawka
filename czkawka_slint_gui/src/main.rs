fn main() {}

slint::slint! {
    import { Button, VerticalBox , HorizontalBox, TabWidget, ListView, StandardListView, StandardTableView} from "std-widgets.slint";
    export component MainWindow {
        VerticalBox {
            TabWidget {
                width: 400px;
                preferred-height: 300px;
                Tab {
                    title: "Empty Folders";
                    ListView {

                    }
                    StandardTableView {
                    visible: true;
                        columns: [{title: "Selection", Sort: false}, {title: "Name"}, {title: "Path"}];
                        rows: [
                            [{ text: "FG"}, {text: "gasg"}, {text:"asg" }],
                            [{ text: "GS"}, {text: "WW"}, {text:"AGAS" }]
                        ];

                    }

                }
                Tab {
                    title: "Empty Files";
                    Text {
                        text: "Empty files";
                    }
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
