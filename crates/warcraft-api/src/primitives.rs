use std::{
    cmp::Ordering,
    ops::{Add, AddAssign},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bytes<const SIZE: usize> {
    handle: [u8; SIZE],
}

impl<const SIZE: usize> From<[u8; SIZE]> for Bytes<SIZE> {
    fn from(value: [u8; SIZE]) -> Self {
        Self::new(value)
    }
}

impl<const SIZE: usize> From<&[u8; SIZE]> for Bytes<SIZE> {
    fn from(value: &[u8; SIZE]) -> Self {
        Self { handle: *value }
    }
}

impl<const SIZE: usize> From<Bytes<SIZE>> for [u8; SIZE] {
    fn from(value: Bytes<SIZE>) -> Self {
        value.handle
    }
}

impl<const SIZE: usize> AsRef<[u8]> for Bytes<SIZE> {
    fn as_ref(&self) -> &[u8] {
        &self.handle
    }
}

impl<const SIZE: usize> AsMut<[u8]> for Bytes<SIZE> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.handle
    }
}

impl<const SIZE: usize> Bytes<SIZE> {
    pub fn new(handle: [u8; SIZE]) -> Self {
        Self { handle }
    }

    pub(crate) fn handle(&self) -> [u8; SIZE] {
        self.handle
    }
}

impl<const SIZE: usize> std::fmt::Display for Bytes<SIZE> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let end = self
            .handle
            .iter()
            .position(|&byte| byte == 0)
            .unwrap_or(SIZE);
        formatter.write_str(std::str::from_utf8(&self.handle[..end]).unwrap_or_default())
    }
}

impl<const SIZE: usize> Default for Bytes<SIZE> {
    fn default() -> Self {
        Self::new([0; SIZE])
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Boolean {
    data: Bytes<1>,
}

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        let byte = u8::from(value);
        Self::new([byte].into())
    }
}

impl From<Boolean> for bool {
    fn from(value: Boolean) -> Self {
        value.get_boolean()
    }
}

impl Boolean {
    pub fn new(data: Bytes<1>) -> Self {
        Self { data }
    }

    pub fn get_boolean(&self) -> bool {
        self.handle()[0] != 0
    }

    fn handle(&self) -> [u8; 1] {
        self.data.handle()
    }
}

impl Serialize for Boolean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(self.get_boolean())
    }
}

impl<'de> Deserialize<'de> for Boolean {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = bool::deserialize(deserializer)?;
        Ok(value.into())
    }
}

impl std::fmt::Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string_pretty(self).unwrap())
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Byte {
    data: Bytes<1>,
}

impl<'de> Deserialize<'de> for Byte {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        Ok(value.into())
    }
}

impl From<[u8; 1]> for Byte {
    fn from(value: [u8; 1]) -> Self {
        Self::new(value.into())
    }
}

impl From<u8> for Byte {
    fn from(value: u8) -> Self {
        Self::new([value].into())
    }
}

impl From<Byte> for u8 {
    fn from(value: Byte) -> Self {
        value.get_byte()
    }
}

impl PartialEq for Byte {
    fn eq(&self, other: &Self) -> bool {
        self.get_byte() == other.get_byte()
    }
}

impl Eq for Byte {}

impl PartialOrd for Byte {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Byte {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_byte().cmp(&other.get_byte())
    }
}

impl Byte {
    pub fn new(data: Bytes<1>) -> Self {
        Self { data }
    }

    pub fn get_byte(&self) -> u8 {
        self.handle()[0]
    }

    fn handle(&self) -> [u8; 1] {
        self.data.handle()
    }
}

impl Serialize for Byte {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.get_byte())
    }
}

impl std::fmt::Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string_pretty(self).unwrap())
    }
}

pub type OwnerId = Integer;

#[derive(Default, Debug, Copy, Clone)]
pub struct Integer {
    data: Bytes<4>,
}

impl From<Bytes<4>> for Integer {
    fn from(value: Bytes<4>) -> Self {
        Self { data: value }
    }
}

impl From<[u8; 4]> for Integer {
    fn from(value: [u8; 4]) -> Self {
        Self::new(value.into())
    }
}

impl From<&[u8; 4]> for Integer {
    fn from(value: &[u8; 4]) -> Self {
        Self::new((*value).into())
    }
}

impl From<u32> for Integer {
    fn from(value: u32) -> Self {
        Self::new(value.to_le_bytes().into())
    }
}

impl From<usize> for Integer {
    fn from(value: usize) -> Self {
        let narrowed = u32::try_from(value).expect("usize fits in u32");
        Self::new(narrowed.to_le_bytes().into())
    }
}

impl From<Integer> for u32 {
    fn from(value: Integer) -> Self {
        value.get_integer()
    }
}

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        self.get_integer() == other.get_integer()
    }
}

impl Eq for Integer {}

impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Integer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_integer().cmp(&other.get_integer())
    }
}

impl Integer {
    pub fn new(data: Bytes<4>) -> Self {
        Self { data }
    }

    pub fn get_integer(&self) -> u32 {
        u32::from_le_bytes(self.handle())
    }

    pub(crate) fn handle(&self) -> [u8; 4] {
        self.data.handle()
    }
}

impl Serialize for Integer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.get_integer())
    }
}

impl<'de> Deserialize<'de> for Integer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Ok(value.into())
    }
}

impl std::fmt::Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string_pretty(self).unwrap())
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Float {
    data: Bytes<4>,
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Self::new(value.to_le_bytes().into())
    }
}

impl From<Float> for f32 {
    fn from(value: Float) -> Self {
        value.get_float()
    }
}

impl Float {
    pub fn new(data: Bytes<4>) -> Self {
        Self { data }
    }

    pub fn get_float(&self) -> f32 {
        f32::from_le_bytes(self.handle())
    }

    fn handle(&self) -> [u8; 4] {
        self.data.handle()
    }
}

impl Serialize for Float {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f32(self.get_float())
    }
}

impl<'de> Deserialize<'de> for Float {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = f32::deserialize(deserializer)?;
        Ok(value.into())
    }
}

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string_pretty(self).unwrap())
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Identifier {
    data: Bytes<4>,
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.data.as_ref() == other.data.as_ref()
    }
}

impl Eq for Identifier {}

impl std::hash::Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.as_ref().hash(state)
    }
}

impl From<[u8; 4]> for Identifier {
    fn from(value: [u8; 4]) -> Self {
        Self::new(value.into())
    }
}

impl From<Bytes<4>> for Identifier {
    fn from(value: Bytes<4>) -> Self {
        Self::new(value)
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        let mut buffer: [u8; 4] = [0; 4];
        let bytes = value.as_bytes();
        let length = bytes.len().min(4);
        buffer[..length].copy_from_slice(&bytes[..length]);
        buffer.reverse();
        Self::new(buffer.into())
    }
}

impl Identifier {
    pub fn new(data: Bytes<4>) -> Self {
        Self { data }
    }

    pub fn get_id(&self) -> String {
        let mut buffer = self.handle();
        buffer.reverse();
        let end = buffer.iter().position(|&byte| byte == 0).unwrap_or(4);
        std::str::from_utf8(&buffer[..end])
            .unwrap_or_default()
            .to_string()
    }

    pub fn is_invalid(&self) -> bool {
        self.handle().iter().all(|byte| *byte == 0)
    }

    fn handle(&self) -> [u8; 4] {
        self.data.handle()
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.get_id())
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw_string = String::deserialize(deserializer)?;
        Ok(raw_string.as_str().into())
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string_pretty(self).unwrap())
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Time {
    data: Integer,
}

impl From<Integer> for Time {
    fn from(value: Integer) -> Self {
        Self::new(value)
    }
}

impl From<[u8; 4]> for Time {
    fn from(value: [u8; 4]) -> Self {
        Self::new(value.into())
    }
}

impl From<&[u8; 4]> for Time {
    fn from(value: &[u8; 4]) -> Self {
        Self::new((*value).into())
    }
}

impl From<Bytes<4>> for Time {
    fn from(value: Bytes<4>) -> Self {
        Self::new(value.into())
    }
}

impl From<u32> for Time {
    fn from(ms: u32) -> Self {
        Self::new(ms.into())
    }
}

impl From<Time> for u32 {
    fn from(value: Time) -> Self {
        value.get_ms()
    }
}

impl std::ops::Add for Time {
    type Output = Time;

    fn add(self, rhs: Self) -> Self {
        Self::from(self.get_ms().saturating_add(rhs.get_ms()))
    }
}

impl std::ops::Sub for Time {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::from(self.get_ms().saturating_sub(rhs.get_ms()))
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.get_ms() == other.get_ms()
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_ms().partial_cmp(&other.get_ms())
    }
}

impl Time {
    pub fn new(data: Integer) -> Self {
        Self { data }
    }

    pub fn from_seconds(seconds: u32) -> Self {
        Self::from(seconds * 1000)
    }

    pub fn from_millis(ms: u32) -> Self {
        Self::from(ms)
    }

    pub fn get_time(&self) -> String {
        let ms = self.get_ms();
        let total_seconds = ms / 1000;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds / 60) % 60;
        let seconds = total_seconds % 60;

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    pub fn get_ms(&self) -> u32 {
        self.data.get_integer()
    }

    pub fn zero() -> Self {
        Self::from(0)
    }

    pub fn format_human(&self) -> String {
        let ms = self.get_ms();

        if ms < 1000 {
            return format!("{}ms", ms);
        }

        let seconds = ms / 1000;
        let millis = ms % 1000;

        if seconds < 60 {
            return format!("{}.{:03}s", seconds, millis);
        }

        let minutes = seconds / 60;
        let seconds = seconds % 60;

        if minutes < 60 {
            return format!("{:02}:{:02}", minutes, seconds);
        }

        let hours = minutes / 60;
        let minutes = minutes % 60;

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.get_time())
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw_string = String::deserialize(deserializer)?;

        let parts: Vec<u32> = raw_string
            .split(':')
            .map(|p| p.parse::<u32>())
            .collect::<Result<_, _>>()
            .map_err(serde::de::Error::custom)?;

        if parts.len() != 3 {
            return Err(serde::de::Error::custom("Invalid time format"));
        }

        let ms = (parts[0] * 3600 + parts[1] * 60 + parts[2]) * 1000;
        Ok(ms.into())
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string_pretty(self).unwrap())
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct ByteString<const SIZE: usize> {
    data: Bytes<SIZE>,
}

impl<const SIZE: usize> From<Bytes<SIZE>> for ByteString<SIZE> {
    fn from(value: Bytes<SIZE>) -> Self {
        Self { data: value }
    }
}

impl<const SIZE: usize> From<[u8; SIZE]> for ByteString<SIZE> {
    fn from(value: [u8; SIZE]) -> Self {
        Self::new(value.into())
    }
}

impl<const SIZE: usize> From<&[u8; SIZE]> for ByteString<SIZE> {
    fn from(value: &[u8; SIZE]) -> Self {
        Self::new((*value).into())
    }
}

impl<const SIZE: usize> From<&str> for ByteString<SIZE> {
    fn from(value: &str) -> Self {
        let mut buffer: [u8; SIZE] = [0; SIZE];
        let bytes = value.as_bytes();
        let length = bytes.len().min(SIZE);
        buffer[..length].copy_from_slice(&bytes[..length]);
        Self {
            data: buffer.into(),
        }
    }
}

impl<const SIZE: usize> From<String> for ByteString<SIZE> {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl<const SIZE: usize> ByteString<SIZE> {
    pub fn new(data: Bytes<SIZE>) -> Self {
        Self { data }
    }

    pub fn get_string(&self) -> String {
        let handle = self.data.handle();
        let end = handle.iter().position(|&byte| byte == 0).unwrap_or(SIZE);

        let decoded = std::str::from_utf8(&handle[..end])
            .unwrap_or_default()
            .to_string();

        decoded
            .strip_prefix('"')
            .and_then(|without_leading_quote| without_leading_quote.strip_suffix('"'))
            .unwrap_or(&decoded)
            .to_string()
    }
}

impl<const SIZE: usize> Serialize for ByteString<SIZE> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.get_string())
    }
}

impl<'de, const SIZE: usize> Deserialize<'de> for ByteString<SIZE> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw_string = String::deserialize(deserializer)?;
        Ok(raw_string.into())
    }
}

impl<const SIZE: usize> std::fmt::Display for ByteString<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string_pretty(self).unwrap())
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Address {
    data: Bytes<8>,
}

impl From<Bytes<8>> for Address {
    fn from(value: Bytes<8>) -> Self {
        Self { data: value }
    }
}

impl From<[u8; 8]> for Address {
    fn from(value: [u8; 8]) -> Self {
        Self::new(value.into())
    }
}

impl From<&[u8; 8]> for Address {
    fn from(value: &[u8; 8]) -> Self {
        Self::new((*value).into())
    }
}

impl From<u64> for Address {
    fn from(value: u64) -> Self {
        Self::new(value.to_le_bytes().into())
    }
}

impl From<usize> for Address {
    fn from(value: usize) -> Self {
        let widened = u64::try_from(value).expect("usize fits in u64");
        Self::from(widened)
    }
}

impl From<Address> for u64 {
    fn from(value: Address) -> Self {
        value.get_address()
    }
}

impl From<Address> for usize {
    fn from(value: Address) -> Self {
        let address = value.get_address();
        usize::try_from(address).expect("address fits in usize")
    }
}

impl PartialEq for Address {
    fn eq(&self, other: &Self) -> bool {
        self.get_address() == other.get_address()
    }
}

impl Eq for Address {}

impl PartialOrd for Address {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Address {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_address().cmp(&other.get_address())
    }
}

impl Add<usize> for Address {
    type Output = Address;

    fn add(self, offset: usize) -> Self::Output {
        let base = self.get_address();
        let offset = u64::try_from(offset).expect("offset fits in u64");
        base.wrapping_add(offset).into()
    }
}

impl AddAssign<usize> for Address {
    fn add_assign(&mut self, offset: usize) {
        let base = self.get_address();
        let offset = u64::try_from(offset).expect("offset fits in u64");
        *self = base.wrapping_add(offset).into()
    }
}

impl PartialEq<usize> for Address {
    fn eq(&self, other: &usize) -> bool {
        let other = u64::try_from(*other).expect("usize fits in u64");
        self.get_address() == other
    }
}

impl PartialEq<Address> for usize {
    fn eq(&self, other: &Address) -> bool {
        let this = u64::try_from(*self).expect("usize fits in u64");
        this == other.get_address()
    }
}

impl PartialOrd<usize> for Address {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        let other = u64::try_from(*other).expect("usize fits in u64");
        self.get_address().partial_cmp(&other)
    }
}

impl PartialOrd<Address> for usize {
    fn partial_cmp(&self, other: &Address) -> Option<Ordering> {
        let this = u64::try_from(*self).expect("usize fits in u64");
        this.partial_cmp(&other.get_address())
    }
}

impl Address {
    pub fn new(data: Bytes<8>) -> Self {
        Self { data }
    }

    pub fn get_address(&self) -> u64 {
        u64::from_le_bytes(self.handle())
    }

    pub fn is_null(&self) -> bool {
        self.get_address() == 0
    }

    fn handle(&self) -> [u8; 8] {
        self.data.handle()
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw_string = String::deserialize(deserializer)?;

        let hex_string = raw_string
            .strip_prefix("0x")
            .ok_or_else(|| serde::de::Error::custom("address must start with 0x"))?;

        let value = u64::from_str_radix(hex_string, 16).map_err(serde::de::Error::custom)?;

        Ok(value.into())
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:8X}", self.get_address())
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct AgentReference {
    data: Bytes<12>,
}

impl From<Bytes<12>> for AgentReference {
    fn from(value: Bytes<12>) -> Self {
        Self::new(value)
    }
}

impl From<[u8; 12]> for AgentReference {
    fn from(value: [u8; 12]) -> Self {
        Self::new(value.into())
    }
}

impl From<&[u8; 12]> for AgentReference {
    fn from(value: &[u8; 12]) -> Self {
        Self::new((*value).into())
    }
}

impl AgentReference {
    pub fn with_tags(presence_tag: Integer, birth_tag: Integer, flags: Integer) -> Self {
        let mut buffer: [u8; 12] = [0; 12];
        buffer[0..4].copy_from_slice(&presence_tag.handle());
        buffer[4..8].copy_from_slice(&birth_tag.handle());
        buffer[8..12].copy_from_slice(&flags.handle());
        Self::new(buffer.into())
    }

    pub fn new(data: Bytes<12>) -> Self {
        Self { data }
    }

    pub fn presence_tag(&self) -> Integer {
        Integer::from(self.slice::<0>())
    }

    pub fn birth_tag(&self) -> Integer {
        Integer::from(self.slice::<4>())
    }

    pub fn flags(&self) -> Integer {
        Integer::from(self.slice::<8>())
    }

    pub fn is_valid(&self) -> bool {
        self.presence_tag()
            .handle()
            .iter()
            .any(|byte| *byte != 0xFF)
    }

    fn slice<const OFFSET: usize>(&self) -> [u8; 4] {
        let bytes = self.data.as_ref();
        bytes[OFFSET..OFFSET + 4].try_into().unwrap()
    }
}

impl Serialize for AgentReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut s = serializer.serialize_struct("AgentReference", 3)?;
        s.serialize_field("presence_tag", &self.presence_tag())?;
        s.serialize_field("birth_tag", &self.birth_tag())?;
        s.serialize_field("flags", &self.flags())?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for AgentReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            presence_tag: Integer,
            birth_tag: Integer,
            flags: Integer,
        }

        let helper = Helper::deserialize(deserializer)?;
        Ok(AgentReference::with_tags(
            helper.presence_tag,
            helper.birth_tag,
            helper.flags,
        ))
    }
}

impl std::fmt::Display for AgentReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string_pretty(self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Bytes

    #[test]
    fn bytes_round_trips_through_array() {
        let original: [u8; 4] = [1, 2, 3, 4];
        let bytes = Bytes::from(original);
        let back: [u8; 4] = bytes.into();
        assert_eq!(back, original);
    }

    #[test]
    fn bytes_display_stops_at_null() {
        let bytes = Bytes::from(*b"ab\0d");
        assert_eq!(bytes.to_string(), "ab");
    }

    #[test]
    fn bytes_as_ref_yields_full_slice() {
        let bytes = Bytes::from([10u8, 20, 30]);
        assert_eq!(bytes.as_ref(), &[10u8, 20, 30]);
    }

    // Boolean

    #[test]
    fn boolean_from_true_is_nonzero() {
        let boolean = Boolean::from(true);
        assert!(boolean.get_boolean());
    }

    #[test]
    fn boolean_from_false_is_zero() {
        let boolean = Boolean::from(false);
        assert!(!boolean.get_boolean());
    }

    #[test]
    fn boolean_serde_round_trip() {
        let original = Boolean::from(true);
        let json = serde_json::to_string(&original).unwrap();
        let parsed: Boolean = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.get_boolean(), original.get_boolean());
    }

    // Byte

    #[test]
    fn byte_from_u8_round_trips() {
        let byte = Byte::from(42u8);
        let back: u8 = byte.into();
        assert_eq!(back, 42);
    }

    #[test]
    fn byte_ordering_matches_u8_ordering() {
        let small = Byte::from(1u8);
        let large = Byte::from(255u8);
        assert!(small < large);
        assert_eq!(small.cmp(&small), Ordering::Equal);
    }

    // Integer

    #[test]
    fn integer_from_u32_round_trips() {
        let integer = Integer::from(0x1234_5678u32);
        let back: u32 = integer.into();
        assert_eq!(back, 0x1234_5678);
    }

    #[test]
    fn integer_from_usize_round_trips() {
        let integer = Integer::from(99usize);
        assert_eq!(integer.get_integer(), 99);
    }

    #[test]
    fn integer_ordering_is_numeric() {
        let small = Integer::from(1u32);
        let large = Integer::from(1000u32);
        assert!(small < large);
    }

    #[test]
    fn integer_serde_round_trip() {
        let original = Integer::from(12345u32);
        let json = serde_json::to_string(&original).unwrap();
        let parsed: Integer = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.get_integer(), original.get_integer());
    }

    // Float

    #[test]
    fn float_from_f32_round_trips() {
        let value: f32 = 1.5;
        let float = Float::from(value);
        let back: f32 = float.into();
        assert!((back - value).abs() < f32::EPSILON);
    }

    // Identifier

    #[test]
    fn identifier_from_str_recovers_original() {
        let identifier = Identifier::from("hpea");
        assert_eq!(identifier.get_id(), "hpea");
    }

    #[test]
    fn identifier_truncates_to_four_bytes() {
        let identifier = Identifier::from("toolong");
        assert_eq!(identifier.get_id().len(), 4);
    }

    #[test]
    fn identifier_all_zeros_is_invalid() {
        let identifier = Identifier::from([0u8; 4]);
        assert!(identifier.is_invalid());
    }

    #[test]
    fn identifier_with_content_is_valid() {
        let identifier = Identifier::from("Amov");
        assert!(!identifier.is_invalid());
    }

    #[test]
    fn identifier_equality_is_byte_wise() {
        let first = Identifier::from("hpea");
        let second = Identifier::from("hpea");
        let third = Identifier::from("hfoo");
        assert_eq!(first, second);
        assert_ne!(first, third);
    }

    // Time

    #[test]
    fn time_zero_is_zero_ms() {
        assert_eq!(Time::zero().get_ms(), 0);
    }

    #[test]
    fn time_from_seconds_converts_to_ms() {
        let time = Time::from_seconds(5);
        assert_eq!(time.get_ms(), 5000);
    }

    #[test]
    fn time_from_millis_identity() {
        let time = Time::from_millis(1234);
        assert_eq!(time.get_ms(), 1234);
    }

    #[test]
    fn time_addition_saturates() {
        let near_max = Time::from(u32::MAX - 1);
        let one = Time::from_millis(10);
        let result = near_max + one;
        assert_eq!(result.get_ms(), u32::MAX);
    }

    #[test]
    fn time_subtraction_saturates_at_zero() {
        let small = Time::from_millis(100);
        let large = Time::from_millis(200);
        let result = small - large;
        assert_eq!(result.get_ms(), 0);
    }

    #[test]
    fn time_get_time_formats_hh_mm_ss() {
        let time = Time::from_seconds(3661);
        assert_eq!(time.get_time(), "01:01:01");
    }

    #[test]
    fn time_format_human_uses_ms_below_one_second() {
        assert_eq!(Time::from_millis(500).format_human(), "500ms");
    }

    #[test]
    fn time_format_human_uses_seconds_below_one_minute() {
        assert_eq!(Time::from_millis(12345).format_human(), "12.345s");
    }

    #[test]
    fn time_format_human_uses_mm_ss_below_one_hour() {
        assert_eq!(Time::from_seconds(90).format_human(), "01:30");
    }

    #[test]
    fn time_format_human_uses_hh_mm_ss_at_or_above_one_hour() {
        assert_eq!(Time::from_seconds(3661).format_human(), "01:01:01");
    }

    // ByteString

    #[test]
    fn byte_string_from_str_round_trips() {
        let byte_string = ByteString::<16>::from("hello");
        assert_eq!(byte_string.get_string(), "hello");
    }

    #[test]
    fn byte_string_strips_surrounding_quotes() {
        let byte_string = ByteString::<16>::from("\"quoted\"");
        assert_eq!(byte_string.get_string(), "quoted");
    }

    #[test]
    fn byte_string_truncates_to_capacity() {
        let byte_string = ByteString::<4>::from("toolongstring");
        assert_eq!(byte_string.get_string().len(), 4);
    }

    // Address

    #[test]
    fn address_from_u64_round_trips() {
        let address = Address::from(0xDEAD_BEEF_u64);
        let back: u64 = address.into();
        assert_eq!(back, 0xDEAD_BEEF);
    }

    #[test]
    fn address_zero_is_null() {
        assert!(Address::from(0u64).is_null());
        assert!(!Address::from(1u64).is_null());
    }

    #[test]
    fn address_add_usize_wraps_correctly() {
        let base = Address::from(100u64);
        let result = base + 50usize;
        assert_eq!(result.get_address(), 150);
    }

    #[test]
    fn address_add_assign_usize_mutates_in_place() {
        let mut address = Address::from(100u64);
        address += 25usize;
        assert_eq!(address.get_address(), 125);
    }

    #[test]
    fn address_partial_eq_with_usize() {
        let address = Address::from(42u64);
        assert!(address == 42usize);
        assert!(42usize == address);
    }

    #[test]
    fn address_ordering_with_usize() {
        let address = Address::from(10u64);
        assert!(address < 20usize);
        assert!(5usize < address);
    }

    #[test]
    fn address_display_is_hex_with_prefix() {
        let address = Address::from(255u64);
        assert!(address.to_string().starts_with("0x"));
    }

    // AgentReference

    #[test]
    fn agent_reference_with_tags_stores_all_three() {
        let presence = Integer::from(1u32);
        let birth = Integer::from(2u32);
        let flags = Integer::from(3u32);
        let agent = AgentReference::with_tags(presence, birth, flags);
        assert_eq!(agent.presence_tag().get_integer(), 1);
        assert_eq!(agent.birth_tag().get_integer(), 2);
        assert_eq!(agent.flags().get_integer(), 3);
    }

    #[test]
    fn agent_reference_all_ff_bytes_is_invalid() {
        let agent = AgentReference::from([0xFF_u8; 12]);
        assert!(!agent.is_valid());
    }

    #[test]
    fn agent_reference_with_nonzero_presence_is_valid() {
        let presence = Integer::from(1u32);
        let birth = Integer::from(0u32);
        let flags = Integer::from(0u32);
        let agent = AgentReference::with_tags(presence, birth, flags);
        assert!(agent.is_valid());
    }

    #[test]
    fn agent_reference_serde_round_trip() {
        let presence = Integer::from(10u32);
        let birth = Integer::from(20u32);
        let flags = Integer::from(30u32);
        let original = AgentReference::with_tags(presence, birth, flags);
        let json = serde_json::to_string(&original).unwrap();
        let parsed: AgentReference = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, original);
    }
}
