#!/bin/bash
NUMBER="8.0.0"
CZKAWKA_PATH="/home/rafal"

cd "$CZKAWKA_PATH"
CZKAWKA_PATH="$CZKAWKA_PATH/czkawka"
rm -rf $CZKAWKA_PATH
git clone https://github.com/qarmin/czkawka.git "$CZKAWKA_PATH"
cd $CZKAWKA_PATH
git checkout "$NUMBER"


cd "$CZKAWKA_PATH/czkawka_cli"
cargo package
if [ $(echo $?) != "0"  ]
then
  echo "Cargo package failed CLI"
  exit 1
fi
git reset --hard


cd "$CZKAWKA_PATH/czkawka_gui"
cargo package
if [ $(echo $?) != "0"  ]
then
  echo "Cargo package failed GUI"
  exit 1
fi
git reset --hard

cd "$CZKAWKA_PATH/krokiet"
cargo package
if [ $(echo $?) != "0"  ]
then
  echo "Cargo package failed krokiet"
  exit 1
fi
git reset --hard




cd "$CZKAWKA_PATH/czkawka_cli"
# sed -i "s/{ path = \"..\/czkawka_core\" }/\"=$NUMBER\"/g" "$CZKAWKA_PATH/czkawka_cli/Cargo.toml"
cargo publish # --allow-dirty
git reset --hard

cd "$CZKAWKA_PATH/czkawka_gui"
# sed -i "s/{ path = \"..\/czkawka_core\" }/\"=$NUMBER\"/g" "$CZKAWKA_PATH/czkawka_gui/Cargo.toml"
cargo publish # --allow-dirty
git reset --hard

cd "$CZKAWKA_PATH/krokiet"
# sed -i "s/{ path = \"..\/czkawka_core\" }/\"=$NUMBER\"/g" "$CZKAWKA_PATH/czkawka_gui/Cargo.toml"
cargo publish # --allow-dirty
git reset --hard
