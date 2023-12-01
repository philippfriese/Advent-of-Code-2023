#!/usr/bin/bash

CURDAY=$(date +%d)
DIR="task_$CURDAY"
mkdir "src/$DIR"
cp src/template.rs "src/$DIR/$DIR.rs"
touch "src/$DIR/mod.rs"
echo "pub mod $DIR;" >> "src/$DIR/mod.rs"

touch "src/$DIR/test.dat"
touch "src/$DIR/data.dat"
