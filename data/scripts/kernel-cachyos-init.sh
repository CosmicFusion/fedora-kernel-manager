#!/bin/bash

YUM_CHANGED=false

if [ ! -f /etc/yum.repos.d/bieszczaders-kernel-cachyos-fedora-$(rpm -E %fedora).repo ]
then
  wget https://copr.fedorainfracloud.org/coprs/bieszczaders/kernel-cachyos/repo/fedora-$(rpm -E %fedora)/bieszczaders-kernel-cachyos-fedora-$(rpm -E %fedora).repo -O /etc/yum.repos.d/bieszczaders-kernel-cachyos-fedora-$(rpm -E %fedora).repo
  YUM_CHANGED=true
fi

if [ ! -f /etc/yum.repos.d/bieszczaders-kernel-cachyos-addons-fedora-$(rpm -E %fedora).repo ]
then
  wget https://copr.fedorainfracloud.org/coprs/bieszczaders/kernel-cachyos-addons/repo/fedora-$(rpm -E %fedora)/bieszczaders-kernel-cachyos-addons-fedora-$(rpm -E %fedora).repo -O /etc/yum.repos.d/bieszczaders-kernel-cachyos-addons-fedora-$(rpm -E %fedora).repo
  YUM_CHANGED=true
fi

if [ YUM_CHANGED == true ]
then
     dnf repoquery
fi