#!/bin/bash
set -Eeuo pipefail

hurl --http1.1 tests_ok/http_version/http_version_11.hurl
