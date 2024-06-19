#! /bin/bash

if [[ "$1" == "scx_disabled" ]]; then
  systemctl stop scx & systemctl disable scx
else
  set -e
  sed -i 's/SCX_SCHEDULER=.*/SCX_SCHEDULER="$1"/' /etc/default/scx
  systemctl enable --now scx
  systemctl restart scx
fi