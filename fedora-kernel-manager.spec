%define pkg_release 0.1.0


Name:          fedora-kernel-manager
Version:       %{pkg_release}
Release:       1%{?dist}
License:       GPLv2
Group:         System Environment/Libraries
Summary:       A Libadwaita rust based application for managing and installing kernels.


URL:            https://github.com/CosmicFusion/fedora-kernel-manager
Source0:        %{URL}/archive/refs/tags/%{pkg_release}.tar.gz

BuildRequires:	wget
BuildRequires:	cargo
BuildRequires:	gdk-pixbuf2-devel
BuildRequires:	gtk4-devel
BuildRequires:	gtk3-devel
BuildRequires:	libadwaita-devel
BuildRequires:	openssl-devel

Requires:   /usr/bin/bash
Requires:	gtk4
Requires:	gtk3
Requires:	libadwaita
Requires: 	glib2
Requires: 	util-linux
Requires: 	polkit
Requires:   iputils
Requires:   fedora-kernel-manager-cachyos-config

Recommends: sched-ext-scx

%description
A Libadwaita rust based application for managing and installing kernels.

%package cachyos-config
Summary:        Config files to enable coprs/bieszczaders/kernel-cachyos in fedora-kernel-manager
Requires:       fedora-kernel-manager

%description cachyos-config
Config files to enable coprs/bieszczaders/kernel-cachyos in fedora-kernel-manager

%files cachyos-config
%{_prefix}/lib/fedora-kernel-manager/kernel_branches/kernel-cachyos.json
%{_datadir}/polkit-1/actions/fkm.kernel.cachyos.init.policy
%{_datadir}/polkit-1/rules.d/99-fkm.kernel.cachyos.init.rules

%prep
%autosetup -p1 -n nobara-drivers-gtk4

%build
DESTDIR=%{buildroot} make install

%files
%{_prefix}/lib/fedora-kernel-manager/*
%{_bindir}/*
%{_datadir}/applications/*
%{_datadir}/icons/hicolor/scalable/apps/*.svg
%{_datadir}/polkit-1/actions/fkm.change.scx.policy
%{_datadir}/polkit-1/actions/fkm.modify.package.policy
