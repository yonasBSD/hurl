#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/http_connection.hurl
