#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   bash scripts/compare_criterion_baseline.sh <group_name> <baseline_json>
# Example:
#   bash scripts/compare_criterion_baseline.sh watch_timer_hot perf_baselines/watch_timer_hot.json

GROUP_NAME="${1:-}"
BASELINE_JSON="${2:-}"

if [[ -z "${GROUP_NAME}" || -z "${BASELINE_JSON}" ]]; then
  echo "usage: $0 <group_name> <baseline_json>" >&2
  exit 2
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "error: jq is required" >&2
  exit 2
fi

if [[ ! -f "${BASELINE_JSON}" ]]; then
  echo "error: baseline file not found: ${BASELINE_JSON}" >&2
  exit 2
fi

# Determine base directory: support both nested and flat criterion layouts
# 1) Nested:   target/criterion/<GROUP_NAME>/<KEY>/new/estimates.json
# 2) Flat:     target/criterion/<KEY>/new/estimates.json
if [[ -d "target/criterion/${GROUP_NAME}" ]]; then
  BASE_DIR="target/criterion/${GROUP_NAME}"
elif [[ -d "target/criterion" ]]; then
  BASE_DIR="target/criterion"
else
  echo "error: criterion directory not found: target/criterion" >&2
  exit 2
fi

# Optional: show which layout is used (useful in CI logs)
echo "Using Criterion base directory: ${BASE_DIR}" >&2

fail_count=0

# Iterate baseline entries: key, median_s, tolerance
while IFS=$'\t' read -r KEY MEDIAN_S TOL; do
  # Build path to criterion estimates for this key
  EST_PATH="${BASE_DIR}/${KEY}/new/estimates.json"
  if [[ ! -f "${EST_PATH}" ]]; then
    echo "missing estimates: ${EST_PATH}" >&2
    fail_count=$((fail_count + 1))
    continue
  fi

  # Criterion stores values in seconds; prefer median, fallback to mean
  ACTUAL_S=$(jq -r '.median.point_estimate // .mean.point_estimate' "${EST_PATH}")
  if [[ -z "${ACTUAL_S}" || "${ACTUAL_S}" == "null" ]]; then
    echo "invalid estimates json: ${EST_PATH}" >&2
    fail_count=$((fail_count + 1))
    continue
  fi

  # allowed upper bound = baseline * (1 + tol)
  ALLOWED=$(awk -v b="${MEDIAN_S}" -v t="${TOL}" 'BEGIN{ printf "%.12f", b*(1.0+t) }')
  OK=$(awk -v a="${ACTUAL_S}" -v m="${ALLOWED}" 'BEGIN{ print (a<=m)?"1":"0" }')

  # Pretty print values in µs/ms when helpful
  pretty() {
    python3 - "$1" <<'PY'
import sys
v=float(sys.argv[1])
if v>=1.0:
    print(f"{v:.3f}s")
elif v>=1e-3:
    print(f"{v*1e3:.3f}ms")
else:
    print(f"{v*1e6:.3f}µs")
PY
  }

  ACT_P=$(pretty "${ACTUAL_S}")
  BASE_P=$(pretty "${MEDIAN_S}")
  ALWD_P=$(pretty "${ALLOWED}")

  if [[ "${OK}" == "1" ]]; then
    echo "OK   ${KEY} actual=${ACT_P} baseline=${BASE_P} tol=${TOL} allowed<=${ALWD_P}"
  else
    echo "FAIL ${KEY} actual=${ACT_P} baseline=${BASE_P} tol=${TOL} allowed<=${ALWD_P}" >&2
    fail_count=$((fail_count + 1))
  fi

done < <(jq -r 'to_entries[] | [ .key, .value.median_s, .value.tolerance ] | @tsv' "${BASELINE_JSON}")

if [[ ${fail_count} -gt 0 ]]; then
  echo "Baseline comparison failed: ${fail_count} regression(s) detected." >&2
  exit 1
fi

echo "Baseline comparison passed."
