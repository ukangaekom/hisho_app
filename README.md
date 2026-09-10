# Development

Your new workspace contains a member crate for each of the web, desktop and mobile platforms, and a `ui` crate for components that are shared between multiple platforms:

```
your_project/
├─ README.md
├─ Cargo.toml
└─ packages/
   ├─ core/
   │  └─ ... # Platform-independent application and CLI logic
   ├─ web/
   │  └─ ... # Web specific UI/logic
   ├─ desktop/
   │  └─ ... # Desktop specific UI/logic
   ├─ mobile/
   │  └─ ... # Mobile specific UI/logic
   └─ ui/
      └─ ... # Component shared between multiple platforms
```

## Platform crates

Each platform crate contains the entry point for the platform, and any assets, components and dependencies that are specific to that platform. For example, the desktop crate in the workspace looks something like this:

```
desktop/ # The desktop crate contains all platform specific UI, logic and dependencies for the desktop app
├─ assets/ # Assets used by the desktop app - Any platform specific assets should go in this folder
├─ src/
│  ├─ main.rs # The entrypoint for the desktop app. It also defines the routes for the desktop platform
│  ├─ views/ # The views each route will render in the desktop version of the app
│  │  ├─ mod.rs # Defines the module for the views route and re-exports the components for each route
│  │  ├─ blog.rs # The component that will render at the /blog/:id route
│  │  ├─ home.rs # The component that will render at the / route
├─ Cargo.toml # The desktop crate's Cargo.toml - This should include all desktop specific dependencies
```

When you start developing with the workspace setup each of the platform crates will look almost identical. The UI starts out exactly the same on all platforms. However, as you continue developing your application, this setup makes it easy to let the views for each platform change independently.

## Shared UI crate

The workspace contains a `ui` crate with components that are shared between multiple platforms. You should put any UI elements you want to use in multiple platforms in this crate. You can also put some shared client side logic in this crate, but be careful to not pull in platform specific dependencies. The `ui` crate starts out something like this:

```
ui/
├─ src/
│  ├─ lib.rs # The entrypoint for the ui crate
│  ├─ hero.rs # The Hero component that will be used in every platform
│  ├─ navbar.rs # The Navbar component that will be used in the layout of every platform's router
```

## Running the workspace

This repository is a Cargo workspace. Run commands from the repository root:

```bash
cd hisho_app
```

The workspace contains these crates:

| Crate | Purpose |
| --- | --- |
| `hisho-core` | Platform-independent application and CLI logic |
| `ui` | Components shared by the platform applications |
| `web` | Browser application |
| `desktop` | Native desktop application |
| `mobile` | Android and mobile application |

The platform crates depend on both `ui` and `hisho-core`. Keep business rules
and command behavior in `hisho-core`; keep Dioxus components and platform
integration in the platform crates.

## Prerequisites

Install the Rust toolchain and Cargo:

```bash
rustup toolchain install stable
rustup default stable
```

Install the Dioxus CLI:

```bash
curl -sSL http://dioxus.dev/install.sh | sh
```

Confirm the tools are available:

```bash
rustc --version
cargo --version
dx --version
```

Fetch dependencies and verify every workspace crate:

```bash
cargo check --workspace
```

## Web

Start the web development server with hot reload:

```bash
cd packages/web
dx serve --open false
```

Open [http://127.0.0.1:8080](http://127.0.0.1:8080). To let Dioxus open the
browser automatically, omit `--open false`.

Build the web application without starting the server:

```bash
cd packages/web
dx build --release
```

## Desktop

The desktop target opens a native application window. It does not provide a
browser URL like the web target.

### Start the desktop app

From the repository root, run:

```bash
dx serve --package desktop --platform desktop
```

Alternatively, run it from the Desktop package directory:

```bash
cd packages/desktop
dx serve --platform desktop
```

The package must include the desktop feature. This workspace already enables it
by default in `packages/desktop/Cargo.toml`:

```bash
[features]
default = ["desktop"]
desktop = ["dioxus/desktop"]
```

### Verify the desktop package

Before troubleshooting the window, confirm that the Rust package compiles:

```bash
cargo check -p desktop
```

If this succeeds, the application code and desktop dependencies are compiling
correctly. A failure after compilation is usually related to the Linux display
environment or the Dioxus CLI runtime.

### Verify the Dioxus CLI

This project uses the Cargo-installed Dioxus CLI. Check which executable is
selected and its version:

```bash
type -a dx
command -v dx
dx --version
```

The preferred executable is:

```text
/home/ekomabasi/.cargo/bin/dx
```

If another installation is selected, use the preferred executable explicitly:

```bash
/home/ekomabasi/.cargo/bin/dx serve --package desktop --platform desktop
```

The shell may cache an older executable. Refresh its command lookup before
retrying:

```bash
hash -r
command -v dx
```

### Linux display and sandbox troubleshooting

Run the desktop command from a normal system terminal with access to the active
graphical session. A Dioxus CLI started inside a sandboxed editor can compile
successfully but fail to create the native window.

If the output contains an error like this:

```text
/snap/core20/current/lib/x86_64-linux-gnu/libpthread.so.0:
undefined symbol: __libc_pthread_init
```

the application is loading an incompatible Snap `core20` library. Retry from a
normal terminal with the Cargo-installed CLI, rather than from the sandboxed
editor terminal:

```bash
cd /path/to/hisho_app
/home/ekomabasi/.cargo/bin/dx serve --package desktop --platform desktop
```

If the output instead contains:

```text
Gtk-WARNING **: cannot open display
Authorization required, but no authorization protocol specified
```

the process can build but does not have permission to access the graphical
display. Use a terminal opened in the logged-in desktop session and ensure
`DISPLAY`, `WAYLAND_DISPLAY`, and `XDG_RUNTIME_DIR` point to that session:

```bash
echo "$DISPLAY"
echo "$WAYLAND_DISPLAY"
echo "$XDG_RUNTIME_DIR"
```

Do not expect a browser page at port `8080` for Desktop. That address is used by
the Web target; Desktop reports `Address: no server addr` and opens a native
window instead.

Build a release desktop bundle:

```bash
cd packages/desktop
dx bundle --release
```

The exact bundle format depends on the host operating system and the installed
native packaging tools.

## Mobile and Android

Android development requires Android Studio or the Android SDK, an Android
device or emulator, and a configured `ANDROID_HOME`. Verify that a device is
available with:

```bash
adb devices
```

Run the mobile application with Dioxus development tooling:

```bash
cd packages/mobile
dx serve
```

Build a standalone release APK:

```bash
cd packages/mobile
dx build --android --release


dx serve --package web
```

Install the generated APK using the path printed by Dioxus, or locate it under
`target/dx/mobile/release/` and install it with:

```bash
adb install -r path/to/app.apk
```

## Core tests

Run tests for the shared core library:

```bash
cargo test -p hisho-core
```

Run all workspace tests:

```bash
cargo test --workspace
```


