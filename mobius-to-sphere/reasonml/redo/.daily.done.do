#!/bin/bash
set -euo pipefail
exec > >(tee >(redo-stamp))
date -I
