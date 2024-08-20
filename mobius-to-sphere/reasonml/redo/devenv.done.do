#!/bin/bash
set -euo pipefail
HERE="$(cd "$(dirname "$0")"; pwd)"

set -x
redo-ifchange opam-switch.done

