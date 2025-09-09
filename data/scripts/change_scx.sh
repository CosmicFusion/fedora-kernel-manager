#! /bin/bash

if [[ "$1" == "scx_disabled" ]]; then
  systemctl disable --now scx
else
  set -e
  sed -i "s/SCX_SCHEDULER=.*/SCX_SCHEDULER="$1"/" /etc/default/scx
  sed -i '/^[^#]*SCX_FLAGS/d' /etc/default/scx
  if [ -n "$2" ]
  then
    echo "SCX_FLAGS=${@:2}" >> /etc/default/scx
  fi
  systemctl enable --now scx
  systemctl restart scx
fi