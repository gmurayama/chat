#!/bin/bash

case $TARGETPLATFORM in
"linux/amd64")
  echo "x86_64-unknown-linux-gnu"
  ;;
"linux/arm64")
  echo "aarch64-unknown-linux-gnu"
  ;;
*)
  echo "ERROR: unsupported platform \"${TARGETPLATFORM}\""
  exit 1
  ;;
esac
