# WORKSPACE file for crate_universe setup
# Note: In bzlmod mode, we still need WORKSPACE for crate_universe
# rules_rust is loaded via MODULE.bazel, but we need to ensure dependencies are available

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# bazel_skylib is a dependency of rules_rust
http_archive(
    name = "bazel_skylib",
    sha256 = "51b5105a760b353773f904d2bbc5e664d0987fbaf22265164de65d43e910d8ac",
    urls = ["https://github.com/bazelbuild/bazel-skylib/releases/download/1.8.1/bazel-skylib-1.8.1.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")
load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")
load("@rules_rust//crate_universe:defs.bzl", "crates_repository")

# Ensure rules_rust dependencies are available
rules_rust_dependencies()

# Register Rust toolchains (needed for crate_universe)
rust_register_toolchains(
    edition = "2021",
    versions = ["1.77.0"],
)

crate_universe_dependencies()

# Set up crate_universe for dependency management
crates_repository(
    name = "crates",
    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:Cargo.Bazel.lock",
    manifests = [
        "//:Cargo.toml",
        "//src/db:Cargo.toml",
        "//src/http:Cargo.toml",
        "//src/utils:Cargo.toml",
    ],
)

load("@crates//:defs.bzl", "crate_repositories")

crate_repositories()


