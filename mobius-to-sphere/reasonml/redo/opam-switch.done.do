#!/bin/bash
set -euo pipefail
redo-ifchange opam.done ../mobius_to_sphere__reasonml.opam ../dune-project
redo-always  # always check dep consistency

exec 3>&1 1>&2  # opam prints logs to stdout...


set -x
cd ..
if [[ "$(opam switch show)" -ef . ]]; then
  : switch exists already
else
  tty-attach opam switch create --yes . \
    --with-test --with-doc --with-dev-setup
fi
#opam switch show
#tty-attach opam switch show
#opam install --verbose --yes dune
#dune build

exec 1>&3  # restore stdout
date -Iseconds
echo OK
