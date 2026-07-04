use super::TailwindClass;

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

/// The byte range of the component directory in a module path ending `::style`.
const fn directory_bounds(module_path: &str) -> (usize, usize) {
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
    (directory_start, directory_end)
}

/// Compile-time guard: a band const must be named exactly after its band.
pub const fn assert_named(actual: &str, expected: &str) {
    if !equal(actual, expected) {
        panic!("a band const has the wrong name");
    }
}

/// Compile-time guard: every utility in a band must carry that band's prefix.
pub const fn assert_band(band: &str, classes: &[TailwindClass]) {
    let mut index = 0;
    while index < classes.len() {
        let class = classes[index].utility();
        if !has_band_prefix(class, band) {
            panic!("a class is not prefixed with its band");
        }
        index += 1;
    }
}

const fn is_band_prefixed(class: &str) -> bool {
    has_band_prefix(class, "mobile")
        || has_band_prefix(class, "tablet")
        || has_band_prefix(class, "laptop")
        || has_band_prefix(class, "desktop")
        || has_band_prefix(class, "qhd")
        || has_band_prefix(class, "uhd")
}

/// Compile-time guard: a BASE class is always-on, so it must NOT carry a band
/// prefix (a width-specific style belongs in that band, never in BASE).
pub const fn assert_base(classes: &[TailwindClass]) {
    let mut index = 0;
    while index < classes.len() {
        let class = classes[index].utility();
        if is_band_prefixed(class) {
            panic!("a BASE class carries a band prefix");
        }
        index += 1;
    }
}

/// Compile-time guard: a state overlay (see `states!`) is always-on within its
/// state, so it must NOT carry a band prefix.
#[allow(dead_code)]
pub const fn assert_flat(classes: &[TailwindClass]) {
    let mut index = 0;
    while index < classes.len() {
        let class = classes[index].utility();
        if is_band_prefixed(class) {
            panic!("a state overlay carries a band prefix");
        }
        index += 1;
    }
}

/// Byte length of the kebab identity derived from `module_path`'s directory.
pub const fn identity_len(module_path: &str) -> usize {
    let (start, end) = directory_bounds(module_path);
    end - start
}

/// Build the kebab identity (the component directory with `_` mapped to `-`).
pub const fn build_identity<const N: usize>(module_path: &str) -> [u8; N] {
    let (start, _end) = directory_bounds(module_path);
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

/// The byte range of the final segment of a module path (the component module
/// itself, as seen from its `mod.rs`).
const fn last_segment_bounds(module_path: &str) -> (usize, usize) {
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
    (start, end)
}

/// Compile-time guard: the PascalCase `component` name must equal the snake_case
/// component directory (the final segment of the module path), capitalization
/// included.
pub const fn assert_component_name(component: &str, module_path: &str) {
    let (start, end) = last_segment_bounds(module_path);
    let directory = module_path.as_bytes();
    let pascal = component.as_bytes();
    let mut directory_index = start;
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
