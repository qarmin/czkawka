#!/bin/bash
NUMBER="8.0.0"
CZKAWKA_PATH="/home/rafal"

cd "$CZKAWKA_PATH"
CZKAWKA_PATH="$CZKAWKA_PATH/czkawka"
rm -rf $CZKAWKA_PATH
git clone https://github.com/qarmin/czkawka.git "$CZKAWKA_PATH"
cd $CZKAWKA_PATH
git checkout "$NUMBER"

cd "$CZKAWKA_PATH/czkawka_core"
cargo package
if [ $(echo $?) != "0"  ]
then
  echo "Cargo package failed CORE"
  exit 1
fi
git reset --hard

cd "$CZKAWKA_PATH/czkawka_core"
cargo publish
git reset --hard

