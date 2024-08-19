#!/bin/bash
set -euo pipefail

redo-ifchange .daily.done ./ensure-homebrew
./ensure-homebrew
date -Iseconds
echo OK
