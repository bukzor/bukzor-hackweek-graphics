#!/bin/bash
set -euo pipefail

redo-ifchange brew.done ../Brewfile

set -x
cd ..
brew bundle
