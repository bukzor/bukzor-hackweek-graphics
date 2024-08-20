#!/bin/bash
set -euo pipefail
redo-ifchange brew-bundle.done .daily.done
set -x
cd ..
opam init --no-setup --yes
date -Iseconds
echo OK
