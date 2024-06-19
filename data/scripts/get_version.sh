#! /bin/bash

dnf info $1 | grep Version | cut -d":" -f2 | head -n1