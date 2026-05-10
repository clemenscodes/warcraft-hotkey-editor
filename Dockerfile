FROM ubuntu:24.04 AS base

ENV DEBIAN_FRONTEND=noninteractive
ENV CI=true
ENV MOON_TOOLCHAIN_FORCE_GLOBALS=true
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME/bin:$PNPM_HOME:/root/.cargo/bin:$PATH"
ENV PLAYWRIGHT_BROWSERS_PATH="/ms-playwright"
ENV DX_HOME="/root/.dx"

RUN apt-get update && apt-get install -y --no-install-recommends \
    curl ca-certificates build-essential pkg-config binaryen git \
    && rm -rf /var/lib/apt/lists/*

# Node.js 24 via NodeSource
RUN curl -fsSL https://deb.nodesource.com/setup_24.x | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*

# Rust 1.95.0 + wasm32 target (matches rust-toolchain.toml)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    --profile minimal \
    --default-toolchain 1.95.0 \
    --target wasm32-unknown-unknown \
    --component clippy,rustfmt \
    --no-modify-path

# dioxus-cli 0.7.9 — GitHub release binary, no crates.io involved
RUN set -eux; \
    arch=$(uname -m); \
    case "$arch" in \
      x86_64)  triple="x86_64-unknown-linux-gnu" ;; \
      aarch64) triple="aarch64-unknown-linux-gnu" ;; \
      *) echo "unsupported arch: $arch" >&2; exit 1 ;; \
    esac; \
    curl --retry 5 --retry-delay 2 -fsSL \
      "https://github.com/DioxusLabs/dioxus/releases/download/v0.7.9/dx-${triple}.tar.gz" \
      | tar -xz -C /usr/local/bin

# wasm-bindgen-cli 0.2.121 — GitHub release binary, no crates.io involved
RUN set -eux; \
    arch=$(uname -m); \
    case "$arch" in \
      x86_64)  triple="x86_64-unknown-linux-musl" ;; \
      aarch64) triple="aarch64-unknown-linux-musl" ;; \
      *) echo "unsupported arch: $arch" >&2; exit 1 ;; \
    esac; \
    curl --retry 5 --retry-delay 2 -fsSL \
      "https://github.com/rustwasm/wasm-bindgen/releases/download/0.2.121/wasm-bindgen-0.2.121-${triple}.tar.gz" \
      | tar -xz --strip-components=1 -C /usr/local/bin \
          "wasm-bindgen-0.2.121-${triple}/wasm-bindgen"

# pnpm + moon + tailwindcss (v4) + playwright (pinned to match Nix devshell)
RUN corepack enable && corepack prepare pnpm@11.0.9 --activate
RUN pnpm add -g @moonrepo/cli@2.0.3 @tailwindcss/cli@4.3.0 @playwright/test@1.59.1

# Chromium + its system library dependencies
RUN playwright install --with-deps chromium

RUN git config --global --add safe.directory '*'

WORKDIR /app

FROM base AS development

COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
COPY crates/hotkey-editor/package.json crates/hotkey-editor/package.json
RUN pnpm install --frozen-lockfile
