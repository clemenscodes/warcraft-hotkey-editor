//! The `const fn` machinery the generated `classes!`/`states!` macros expand
//! into. Public so the macro expansions (which land in the consumer crate) can
//! call it, but `#[doc(hidden)]` at the crate root: it is not part of the
//! supported surface. Everything here runs at compile time.

use crate::TailwindClass;

const fn to_lower(byte: u8) -> u8 {
    if byte >= b'A' && byte <= b'Z' {
        byte + 32
    } else {
        byte
    }
}

const fn to_upper(byte: u8) -> u8 {
    if byte >= b'a' && byte <= b'z' {
        byte - 32
    } else {
        byte
    }
}

const fn equal(left: &str, right: &str) -> bool {
    let left = left.as_bytes();
    let right = right.as_bytes();
    if left.len() != right.len() {
        return false;
    }
    let mut index = 0;
    while index < left.len() {
        if left[index] != right[index] {
            return false;
        }
        index += 1;
    }
    true
}

/// True when `class` begins with the lowercase `band` followed by `:`.
const fn has_band_prefix(class: &str, band: &str) -> bool {
    let class = class.as_bytes();
    let band = band.as_bytes();
    if class.len() < band.len() + 1 {
        return false;
    }
    let mut index = 0;
    while index < band.len() {
        if to_lower(class[index]) != band[index] {
            return false;
        }
        index += 1;
    }
    class[band.len()] == b':'
}

/// True when `class` carries any of the declared responsive band prefixes.
const fn has_any_band_prefix(class: &str, bands: &[&str]) -> bool {
    let mut index = 0;
    while index < bands.len() {
        if has_band_prefix(class, bands[index]) {
            return true;
        }
        index += 1;
    }
    false
}

/// True when `key` names one of the declared responsive bands.
const fn is_declared_band(key: &str, bands: &[&str]) -> bool {
    let mut index = 0;
    while index < bands.len() {
        if equal(key, bands[index]) {
            return true;
        }
        index += 1;
    }
    false
}

/// The byte range of a segment within a module path.
struct SegmentBounds {
    start: usize,
    end: usize,
}

impl SegmentBounds {
    const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    const fn start(&self) -> usize {
        self.start
    }

    const fn end(&self) -> usize {
        self.end
    }
}

/// The byte range of the component directory in a module path ending `::style`.
const fn directory_bounds(module_path: &str) -> SegmentBounds {
    let bytes = module_path.as_bytes();
    let suffix = b"::style";
    if bytes.len() < suffix.len() + 1 {
        panic!("classes! must be invoked from a component's style.rs");
    }
    let directory_end = bytes.len() - suffix.len();
    let mut offset = 0;
    while offset < suffix.len() {
        if bytes[directory_end + offset] != suffix[offset] {
            panic!("classes! must be invoked from a component's style.rs");
        }
        offset += 1;
    }
    let mut directory_start = 0;
    let mut position = 0;
    while position + 1 < directory_end {
        if bytes[position] == b':' && bytes[position + 1] == b':' {
            directory_start = position + 2;
        }
        position += 1;
    }
    SegmentBounds::new(directory_start, directory_end)
}

/// The byte range of the final segment of a module path (the component module
/// itself, as seen from its `mod.rs`).
const fn last_segment_bounds(module_path: &str) -> SegmentBounds {
    let bytes = module_path.as_bytes();
    let end = bytes.len();
    let mut start = 0;
    let mut position = 0;
    while position + 1 < end {
        if bytes[position] == b':' && bytes[position + 1] == b':' {
            start = position + 2;
        }
        position += 1;
    }
    SegmentBounds::new(start, end)
}

/// Compile-time guard for one keyed entry of `classes!`. The reserved key
/// `base` is always-on and must carry no declared band prefix; every other key
/// must name a declared band and every one of its utilities must carry that
/// band's prefix.
pub const fn assert_key(key: &str, bands: &[&str], classes: &[TailwindClass]) {
    if equal(key, "base") {
        let mut index = 0;
        while index < classes.len() {
            if has_any_band_prefix(classes[index].utility(), bands) {
                panic!("a base class carries a responsive band prefix");
            }
            index += 1;
        }
        return;
    }
    if !is_declared_band(key, bands) {
        panic!("a class key is not a declared responsive band");
    }
    let mut index = 0;
    while index < classes.len() {
        if !has_band_prefix(classes[index].utility(), key) {
            panic!("a class is not prefixed with its band");
        }
        index += 1;
    }
}

/// Compile-time guard: a state overlay (see `states!`) is always-on within its
/// state, so it must carry no declared band prefix.
pub const fn assert_flat(classes: &[TailwindClass], bands: &[&str]) {
    let mut index = 0;
    while index < classes.len() {
        if has_any_band_prefix(classes[index].utility(), bands) {
            panic!("a state overlay carries a responsive band prefix");
        }
        index += 1;
    }
}

/// Byte length of the kebab identity derived from `module_path`'s directory.
pub const fn identity_len(module_path: &str) -> usize {
    let bounds = directory_bounds(module_path);
    bounds.end() - bounds.start()
}

/// Build the kebab identity (the component directory with `_` mapped to `-`).
pub const fn build_identity<const N: usize>(module_path: &str) -> [u8; N] {
    let bounds = directory_bounds(module_path);
    let start = bounds.start();
    let bytes = module_path.as_bytes();
    let mut out = [0u8; N];
    let mut index = 0;
    while index < N {
        let byte = bytes[start + index];
        out[index] = if byte == b'_' { b'-' } else { byte };
        index += 1;
    }
    out
}

/// Compile-time guard: the PascalCase `component` name must equal the snake_case
/// component directory (the final segment of the module path), capitalization
/// included.
pub const fn assert_component_name(component: &str, module_path: &str) {
    let bounds = last_segment_bounds(module_path);
    let end = bounds.end();
    let directory = module_path.as_bytes();
    let pascal = component.as_bytes();
    let mut directory_index = bounds.start();
    let mut pascal_index = 0;
    let mut at_word_start = true;
    while directory_index < end {
        let byte = directory[directory_index];
        if byte == b'_' || byte == b'-' {
            at_word_start = true;
            directory_index += 1;
            continue;
        }
        if pascal_index >= pascal.len() {
            panic!("component name does not match its directory");
        }
        let expected = if at_word_start { to_upper(byte) } else { byte };
        if pascal[pascal_index] != expected {
            panic!("component name does not match its directory");
        }
        pascal_index += 1;
        at_word_start = false;
        directory_index += 1;
    }
    if pascal_index != pascal.len() {
        panic!("component name does not match its directory");
    }
}

/// Length of the joined class string: the identity plus, per band utility, a
/// leading space and its bytes.
pub const fn joined_len(identity: &str, bands: &[&[TailwindClass]]) -> usize {
    let mut len = identity.len();
    let mut band_index = 0;
    while band_index < bands.len() {
        let band = bands[band_index];
        let mut utility_index = 0;
        while utility_index < band.len() {
            let utility = band[utility_index].utility();
            len += 1 + utility.len();
            utility_index += 1;
        }
        band_index += 1;
    }
    len
}

/// Write the identity and every band utility, space separated, into a buffer
/// sized by [`joined_len`].
pub const fn join_into<const N: usize>(identity: &str, bands: &[&[TailwindClass]]) -> [u8; N] {
    let mut out = [0u8; N];
    let mut position = 0;
    let identity_bytes = identity.as_bytes();
    let mut byte_index = 0;
    while byte_index < identity_bytes.len() {
        out[position] = identity_bytes[byte_index];
        position += 1;
        byte_index += 1;
    }
    let mut band_index = 0;
    while band_index < bands.len() {
        let band = bands[band_index];
        let mut utility_index = 0;
        while utility_index < band.len() {
            out[position] = b' ';
            position += 1;
            let utility = band[utility_index].utility();
            let utility_bytes = utility.as_bytes();
            let mut inner = 0;
            while inner < utility_bytes.len() {
                out[position] = utility_bytes[inner];
                position += 1;
                inner += 1;
            }
            utility_index += 1;
        }
        band_index += 1;
    }
    out
}
