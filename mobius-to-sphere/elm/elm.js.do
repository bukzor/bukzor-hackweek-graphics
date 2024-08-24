#!/bin/sh

redo-ifchange Main.elm
tmp="$3.js"

set -ex
# https://github.com/rtfeldman/elm-spa-example?tab=readme-ov-file#production-build
elm make Main.elm --output="$tmp" --optimize >&2
< "$tmp" \
  uglifyjs --compress 'pure_funcs="F2,F3,F4,F5,F6,F7,F8,F9,A2,A3,A4,A5,A6,A7,A8,A9",pure_getters=true,keep_fargs=false,unsafe_comps=true,unsafe=true,passes=2' |
  uglifyjs --mangle \
;
rm "$tmp"
