#!/usr/bin/env bash

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

warn() {
	local fmt=$1
	shift
	printf "%b$fmt%b\n" "$YELLOW" "$@" "$NC" >&2
}

err() {
	local fmt=$1
	shift
	printf "%b$fmt%b\n" "$RED" "$@" "$NC" >&1
}

msg() {
	local fmt=$1
	shift
	printf "%b$fmt%b\n" "$GREEN" "$@" "$NC"
}
