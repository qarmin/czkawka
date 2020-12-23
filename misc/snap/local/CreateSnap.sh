#!/bin/bash
NUMBER="2.0.0"
CZKAWKA_PATH="/home/rafal"

cd "$CZKAWKA_PATH"
CZKAWKA_PATH="$CZKAWKA_PATH/czkawka"
rm -rf $CZKAWKA_PATH
git clone https://github.com/qarmin/czkawka.git "$CZKAWKA_PATH"
cd $CZKAWKA_PATH
git checkout "$NUMBER"

cd "$CZKAWKA_PATH/misc/snap"
snapcraft

snapcraft login

snapcraft upload --release=stable "czkawka_${NUMBER}_amd64.snap"
