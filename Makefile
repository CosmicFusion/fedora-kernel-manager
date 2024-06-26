all:
	true

build:
	cargo fetch
	cargo build --release

build_debug:
	cargo fetch
	cargo build --debug

install_no_build:
	mkdir -p $(DESTDIR)/usr/bin/
	cp -vf target/release/fedora-kernel-manager $(DESTDIR)/usr/bin/
	chmod 755 $(DESTDIR)/usr/bin/fedora-kernel-manager
	mkdir -p $(DESTDIR)/usr/lib/fedora-kernel-manager/
	cp -rvf data/scripts $(DESTDIR)/usr/lib/fedora-kernel-manager/
	chmod 755 $(DESTDIR)/usr/lib/fedora-kernel-manager/scripts/*.sh
	cp -rvf data/locales $(DESTDIR)/usr/lib/fedora-kernel-manager/
	cp -rvf data/kernel_branches $(DESTDIR)/usr/lib/fedora-kernel-manager/
	cp -vf data/scx_scheds.json $(DESTDIR)/usr/lib/fedora-kernel-manager/
	mkdir -p $(DESTDIR)/usr/share/applications
	mkdir -p $(DESTDIR)/usr/share/icons/hicolor/scalable/apps
	cp -vf data/com.github.cosmicfusion.fedora-kernel-manager.svg $(DESTDIR)/usr/share/icons/hicolor/scalable/apps/
	cp -vf data/com.github.cosmicfusion.fedora-kernel-manager.desktop  $(DESTDIR)/usr/share/applications/
	cp -rvf data/polkit-1 $(DESTDIR)/usr/share/

install_no_build_debug:
	mkdir -p $(DESTDIR)/usr/bin/
	cp -vf target/debug/fedora-kernel-manager $(DESTDIR)/usr/bin/
	chmod 755 $(DESTDIR)/usr/bin/fedora-kernel-manager
	mkdir -p $(DESTDIR)/usr/lib/fedora-kernel-manager/
	cp -rvf data/scripts $(DESTDIR)/usr/lib/fedora-kernel-manager/
	chmod 755 $(DESTDIR)/usr/lib/fedora-kernel-manager/scripts/*.sh
	cp -rvf data/locales $(DESTDIR)/usr/lib/fedora-kernel-manager/
	cp -rvf data/kernel_branches $(DESTDIR)/usr/lib/fedora-kernel-manager/
	cp -vf data/scx_scheds.json $(DESTDIR)/usr/lib/fedora-kernel-manager/
	mkdir -p $(DESTDIR)/usr/share/applications
	mkdir -p $(DESTDIR)/usr/share/icons/hicolor/scalable/apps
	cp -vf data/com.github.cosmicfusion.fedora-kernel-manager.svg $(DESTDIR)/usr/share/icons/hicolor/scalable/apps/
	cp -vf data/com.github.cosmicfusion.fedora-kernel-manager.desktop  $(DESTDIR)/usr/share/applications/
	cp -rvf data/polkit-1 $(DESTDIR)/usr/share/

install:
	mkdir -p $(DESTDIR)/usr/bin/
	cargo fetch
	cargo build --release
	cp -vf target/release/fedora-kernel-manager $(DESTDIR)/usr/bin/
	chmod 755 $(DESTDIR)/usr/bin/fedora-kernel-manager
	mkdir -p $(DESTDIR)/usr/lib/fedora-kernel-manager/
	cp -rvf data/scripts $(DESTDIR)/usr/lib/fedora-kernel-manager/
	chmod 755 $(DESTDIR)/usr/lib/fedora-kernel-manager/scripts/*.sh
	cp -rvf data/locales $(DESTDIR)/usr/lib/fedora-kernel-manager/
	cp -rvf data/kernel_branches $(DESTDIR)/usr/lib/fedora-kernel-manager/
	cp -vf data/scx_scheds.json $(DESTDIR)/usr/lib/fedora-kernel-manager/
	mkdir -p $(DESTDIR)/usr/share/applications
	mkdir -p $(DESTDIR)/usr/share/icons/hicolor/scalable/apps
	cp -vf data/com.github.cosmicfusion.fedora-kernel-manager.svg $(DESTDIR)/usr/share/icons/hicolor/scalable/apps/
	cp -vf data/com.github.cosmicfusion.fedora-kernel-manager.desktop  $(DESTDIR)/usr/share/applications/
	cp -rvf data/polkit-1 $(DESTDIR)/usr/share/