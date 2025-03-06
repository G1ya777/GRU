build: 
 cargo build


build_no_output: 
 $(cargo build > /dev/null 2>&1)

publish: 
 ./.release.sh
 cargo publish

help: build_no_output 
 target/debug/rename-gru --help

dev *options: build
 target/debug/rename-gru {{options}}
