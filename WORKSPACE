# WORKSPACE file for all external dependencies
# Note: Bzlmod is disabled, using WORKSPACE for all dependency management

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# bazel_skylib is a dependency of rules_rust
http_archive(
    name = "bazel_skylib",
    sha256 = "51b5105a760b353773f904d2bbc5e664d0987fbaf22265164de65d43e910d8ac",
    urls = ["https://github.com/bazelbuild/bazel-skylib/releases/download/1.8.1/bazel-skylib-1.8.1.tar.gz"],
)

# Rust rules version 0.40.0
http_archive(
    name = "rules_rust",
    sha256 = "c30dfdf1e86fd50650a76ea645b3a45f2f00667b06187a685e9554e167ca97ee",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.40.0/rules_rust-v0.40.0.tar.gz"],
)

# C++ toolchain (required for Rust linking)
# hermetic_cc_toolchain version 2.2.1
http_archive(
    name = "hermetic_cc_toolchain",
    sha256 = "3b8107de0d017fe32e6434086a9568f97c60a111b49dc34fc7001e139c30fdea",
    urls = [
        "https://mirror.bazel.build/github.com/uber/hermetic_cc_toolchain/releases/download/v2.2.1/hermetic_cc_toolchain-v2.2.1.tar.gz",
        "https://github.com/uber/hermetic_cc_toolchain/releases/download/v2.2.1/hermetic_cc_toolchain-v2.2.1.tar.gz",
    ],
)

# toolchains_llvm version 1.2.0
http_archive(
    name = "toolchains_llvm",
    sha256 = "e3fb6dc6b77eaf167cb2b0c410df95d09127cbe20547e5a329c771808a816ab4",
    strip_prefix = "toolchains_llvm-v1.2.0",
    urls = ["https://github.com/bazel-contrib/toolchains_llvm/releases/download/v1.2.0/toolchains_llvm-v1.2.0.tar.gz"],
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

#########################
# Toolchains
#########################

# C++ toolchain setup (zig_sdk for Windows)
load("@hermetic_cc_toolchain//toolchain:defs.bzl", zig_toolchains = "toolchains")
zig_toolchains()

# LLVM toolchain for macOS and Linux
load("@toolchains_llvm//toolchain:deps.bzl", "bazel_toolchain_dependencies")
load("@toolchains_llvm//toolchain:rules.bzl", "llvm_toolchain")

bazel_toolchain_dependencies()

llvm_toolchain(
    name = "llvm_toolchain",
    llvm_versions = {
        "darwin-aarch64": "16.0.0",
        "darwin-x86_64": "15.0.7",
    },
)

