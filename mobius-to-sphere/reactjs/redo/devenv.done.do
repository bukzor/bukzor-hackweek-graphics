#!/bin/bash
set -euo pipefail
HERE="$(cd "$(dirname "$0")"; pwd)"
project_root="$(cd "$HERE/.."; pwd)"


redo-ifchange pnpm.done ../package.json ../pnpm-lock.yaml
set -x
cd "$project_root"
pnpm install
