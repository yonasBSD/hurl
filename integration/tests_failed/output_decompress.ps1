Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_failed/output_decompress.hurl --compressed
