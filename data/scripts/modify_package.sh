#! /bin/bash

set -e

if rpm -q "$1"
then
		dnf remove -y "$1"
else
		dnf install -y "$1"
fi
exit 0