#!/bin/bash
set -euo pipefail
redo-ifchange volta.done .daily.done
set -x
cd ..
volta install --verbose node pnpm
date -Iseconds
echo OK
