SHELL := /bin/sh

DX ?= dx

.PHONY: help check test core core-run web mobile app desktop build-web build-mobile build-app build-desktop bundle-mobile bundle-desktop bundle-android bundle-android-aab bundle-ios bundle-linux bundle-windows bundle-macos release-mobile release-desktop release-all release

help:
	@printf '%s\n' \
		'make web          Run the web app with hot reload' \
		'make mobile       Run the Android/mobile app with hot reload' \
		'make app          Run the desktop app' \
		'make core         Download and check the core crate' \
		'make core-run     Run the core terminal chat' \
		'make check        Check every workspace crate' \
		'make test         Run every workspace test' \
		'make build-web    Build the web app for release' \
		'make build-mobile Build the Android app for release' \
		'make build-app    Bundle the desktop app for release' \
		'make bundle-android Create a release Android APK' \
		'make bundle-ios   Create a release iOS archive' \
		'make bundle-linux Create a Linux AppImage' \
		'make bundle-windows Create a Windows MSI' \
		'make bundle-macos Create a macOS DMG' \
		'make release-mobile Build Android and iOS bundles' \
		'make release-desktop Build Linux, Windows, and macOS bundles' \
		'make release-all  Build every platform bundle (use CI per OS)'

check:
	cargo check --workspace

test:
	cargo test --workspace

core:
	cargo fetch --manifest-path packages/core/Cargo.toml
	cargo check -p hisho-core

core-run:
	cargo run -p hisho-core --bin hisho-core

web:
	$(DX) serve --package web --platform web

mobile:
	$(DX) serve --package mobile --platform android

app: desktop

desktop:
	$(DX) serve --package desktop 

build-web:
	$(DX) build --package web --platform web --release

build-mobile:
	$(DX) build --package mobile --platform android --release

build-app: build-desktop

build-desktop:
	$(MAKE) bundle-desktop

bundle-mobile:
	$(MAKE) bundle-android

bundle-desktop:
	$(DX) bundle --package desktop --platform desktop --release

bundle-android:
	$(DX) bundle --package mobile --platform android --package-types apk --release

bundle-android-aab:
	$(DX) bundle --package mobile --platform android --package-types aab --release

bundle-ios:
	$(DX) bundle --package mobile --platform ios --package-types ipa --release

bundle-linux:
	$(DX) bundle --package desktop --platform linux --package-types appimage --release

bundle-windows:
	$(DX) bundle --package desktop --platform windows --package-types msi --release

bundle-macos:
	$(DX) bundle --package desktop --platform macos --package-types dmg --release

release-mobile: bundle-android bundle-ios

release-desktop: bundle-linux bundle-windows bundle-macos

release-all: release-mobile release-desktop

release: release-all