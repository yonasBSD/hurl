#!/bin/bash
set -Eeuo pipefail

hurl --json tests_ok/captures/captures.hurl
