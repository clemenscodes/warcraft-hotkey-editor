mod domain;

pub use domain::*;

use std::{
    borrow::Borrow,
    cmp::Ordering,
    collections::BTreeMap,
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

    fn handle(&self) -> [u8; SIZE] {
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

    fn handle(&self) -> [u8; 4] {
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

#[repr(u8)]
#[derive(Default, Debug, Copy, Clone, Deserialize, Serialize)]

pub enum RacePreference {
    Human = 0x01,
    Orc = 0x02,
    Nightelf = 0x04,
    Undead = 0x08,
    Demon = 0x10,
    #[default]
    Random = 0x20,
    UserSelectable = 0x40,
}

impl From<Byte> for RacePreference {
    fn from(value: Byte) -> Self {
        use RacePreference::*;
        // UserSelectable is masked into the value in memory, so just add its value 0x40
        match value.get_byte() {
            0x41 | 0x01 => Human,
            0x42 | 0x02 => Orc,
            0x44 | 0x04 => Nightelf,
            0x48 | 0x08 => Undead,
            0x50 | 0x10 => Demon,
            0x60 | 0x20 => Random,
            _ => Self::default(),
        }
    }
}

#[repr(u8)]
#[derive(Default, Debug, Copy, Clone, Deserialize, Serialize)]

pub enum PlayerRace {
    #[default]
    Unknown = 0,
    Human = 1,
    Orc = 2,
    Undead = 3,
    NightElf = 4,
    Demon = 5,
    Last = 6,
    Other = 7,
    Creep = 8,
    Commoner = 9,
    Critter = 10,
    Naga = 11,
}

impl From<Byte> for PlayerRace {
    fn from(value: Byte) -> Self {
        use PlayerRace::*;

        match value.get_byte() {
            0 => Unknown,
            1 => Human,
            2 => Orc,
            3 => Undead,
            4 => NightElf,
            5 => Demon,
            6 => Last,
            7 => Other,
            8 => Creep,
            9 => Commoner,
            10 => Critter,
            11 => Naga,
            _ => Unknown,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]

pub enum PlayerType {
    Empty = 0,
    Player = 1,
    Computer = 2,
    Neutral = 3,
    Observer = 4,
    None = 5,
    Other = 6,
}

impl From<Byte> for PlayerType {
    fn from(value: Byte) -> Self {
        use PlayerType::*;

        match value.get_byte() {
            0 => Empty,
            1 => Player,
            2 => Computer,
            3 => Neutral,
            4 => Observer,
            5 => None,
            6 => Other,
            _ => Empty,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]

pub enum PlayerGameResult {
    Victory = 0,
    Defeat = 1,
    Tie = 2,
    Neutral = 3,
}

impl From<Byte> for PlayerGameResult {
    fn from(value: Byte) -> Self {
        use PlayerGameResult::*;

        match value.get_byte() {
            0 => Victory,
            1 => Defeat,
            2 => Tie,
            3 => Neutral,
            _ => Neutral,
        }
    }
}

#[repr(u8)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]

pub enum PlayerSlotState {
    #[default]
    Empty = 0,
    Playing = 1,
    Left = 2,
}

impl From<Byte> for PlayerSlotState {
    fn from(value: Byte) -> Self {
        use PlayerSlotState::*;

        match value.get_byte() {
            0 => Empty,
            1 => Playing,
            2 => Left,
            _ => Self::default(),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]

pub enum AiDifficultyPreference {
    Newbie = 0,
    Normal = 1,
    Insane = 2,
}

impl From<Byte> for AiDifficultyPreference {
    fn from(value: Byte) -> Self {
        use AiDifficultyPreference::*;

        match value.get_byte() {
            0 => Newbie,
            1 => Normal,
            2 => Insane,
            _ => Newbie,
        }
    }
}

#[repr(u8)]
#[derive(Default, Debug, Copy, Clone, Deserialize, Serialize)]

pub enum PlayerColor {
    #[default]
    Red = 0,
    Blue = 1,
    Teal = 2,
    Purple = 3,
    Yellow = 4,
    Orange = 5,
    Green = 6,
    Pink = 7,
    Gray = 8,
    LightBlue = 9,
    DarkGreen = 10,
    Brown = 11,
    Maroon = 12,
    Navy = 13,
    Turquoise = 14,
    Violet = 15,
    Wheat = 16,
    Peach = 17,
    Mint = 18,
    Lavender = 19,
    Coal = 20,
    Snow = 21,
    Emerald = 22,
    Peanut = 23,
}

impl PlayerColor {
    pub fn color_code(&self) -> &'static str {
        match self {
            PlayerColor::Red => "cffff0303",
            PlayerColor::Blue => "cff0042ff",
            PlayerColor::Teal => "cff1be7ba",
            PlayerColor::Purple => "cff550081",
            PlayerColor::Yellow => "cfffefc00",
            PlayerColor::Orange => "cfffe890d",
            PlayerColor::Green => "cff21bf00",
            PlayerColor::Pink => "cffe45caf",
            PlayerColor::Gray => "cff939596",
            PlayerColor::LightBlue => "cff7ebff1",
            PlayerColor::DarkGreen => "cff106247",
            PlayerColor::Brown => "cff4f2b05",
            PlayerColor::Maroon => "cff9c0000",
            PlayerColor::Navy => "cff0000c3",
            PlayerColor::Turquoise => "cff00ebff",
            PlayerColor::Violet => "cffbd00ff",
            PlayerColor::Wheat => "cffecce87",
            PlayerColor::Peach => "cfff7a58b",
            PlayerColor::Mint => "cffbfff81",
            PlayerColor::Lavender => "cffdbb8eb",
            PlayerColor::Coal => "cff4f5055",
            PlayerColor::Snow => "cffecf0ff",
            PlayerColor::Emerald => "cff00781e",
            PlayerColor::Peanut => "cffa56f34",
        }
    }

    pub fn rgba(&self) -> [f32; 4] {
        let code = self.color_code();
        let hex = &code[3..9];
        let red = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
        let green = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
        let blue = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);

        [
            f32::from(red) / 255.0,
            f32::from(green) / 255.0,
            f32::from(blue) / 255.0,
            1.0,
        ]
    }
}

impl From<Byte> for PlayerColor {
    fn from(value: Byte) -> Self {
        use PlayerColor::*;

        match value.get_byte() {
            0 => Red,
            1 => Blue,
            2 => Teal,
            3 => Purple,
            4 => Yellow,
            5 => Orange,
            6 => Green,
            7 => Pink,
            8 => Gray,
            9 => LightBlue,
            10 => DarkGreen,
            11 => Brown,
            12 => Maroon,
            13 => Navy,
            14 => Turquoise,
            15 => Violet,
            16 => Wheat,
            17 => Peach,
            18 => Mint,
            19 => Lavender,
            20 => Coal,
            21 => Snow,
            22 => Emerald,
            23 => Peanut,
            _ => Self::default(),
        }
    }
}

#[derive(Default, Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemClass {
    Permanent = 0x0,
    Charged = 0x1,
    PowerUp = 0x2,
    Artifact = 0x3,
    #[default]
    Purchasable = 0x4,
    Campaign = 0x5,
    Miscellaneous = 0x6,
    Unknown = 0x7,
    Any = 0x8,
}

impl From<Integer> for ItemClass {
    fn from(value: Integer) -> Self {
        use ItemClass::*;
        match value.get_integer() {
            0x0 => Permanent,
            0x1 => Charged,
            0x2 => Artifact,
            0x3 => PowerUp,
            0x4 => Purchasable,
            0x5 => Campaign,
            0x6 => Miscellaneous,
            0x7 => Unknown,
            0x8 => Any,
            _ => Self::default(),
        }
    }
}

impl ItemClass {
    pub fn from_slk(value: &str) -> Option<Self> {
        match value {
            "Artifact" => Some(ItemClass::Artifact),
            "Permanent" => Some(ItemClass::Permanent),
            "Charged" => Some(ItemClass::Charged),
            "PowerUp" => Some(ItemClass::PowerUp),
            "Campaign" => Some(ItemClass::Campaign),
            "Miscellaneous" => Some(ItemClass::Miscellaneous),
            "Purchasable" => Some(ItemClass::Purchasable),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Race {
    Human,
    Nightelf,
    Orc,
    Undead,
    Neutral,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnitKind {
    #[default]
    Soldier,
    Worker,
    Hero,
    Building,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarcraftObjectKind {
    #[default]
    Unit,
    Ability,
    Upgrade,
    Item,
    Command,
}

pub type ObjectMap = BTreeMap<WarcraftObjectId, WarcraftObject>;

#[derive(Default, Debug, Clone)]
pub struct WarcraftDatabase {
    db: ObjectMap,
}

impl<'a> IntoIterator for &'a WarcraftDatabase {
    type Item = (&'a WarcraftObjectId, &'a WarcraftObject);
    type IntoIter = std::collections::btree_map::Iter<'a, WarcraftObjectId, WarcraftObject>;

    fn into_iter(self) -> Self::IntoIter {
        self.db.iter()
    }
}

impl WarcraftDatabase {
    pub fn new(db: ObjectMap) -> Self {
        Self { db }
    }

    pub fn get(&self, id: Identifier) -> Option<&WarcraftObject> {
        self.db.get(id.get_id().as_str())
    }

    pub fn db(&self) -> &ObjectMap {
        &self.db
    }

    pub fn get_icons(&self, id: Identifier) -> Option<&'static [&'static str]> {
        self.get(id).map(|object| object.icons)
    }

    pub fn get_names(&self, id: Identifier) -> Option<&'static [&'static str]> {
        self.get(id).map(|object| object.names)
    }

    pub fn get_ability_max_level(&self, id: Identifier) -> Option<usize> {
        match self.get(id)?.meta {
            WarcraftObjectMeta::Ability(ref meta) => Some(meta.max_level()),
            _ => None,
        }
    }

    pub fn get_upgrade_max_level(&self, id: Identifier) -> Option<usize> {
        match self.get(id)?.meta {
            WarcraftObjectMeta::Upgrade(ref meta) => Some(meta.max_level()),
            _ => None,
        }
    }

    pub fn get_max_level(&self, id: Identifier) -> Option<usize> {
        match self.get(id)?.meta {
            WarcraftObjectMeta::Ability(ref meta) => Some(meta.max_level()),
            WarcraftObjectMeta::Upgrade(ref meta) => Some(meta.max_level()),
            _ => None,
        }
    }

    pub fn is_ultimate_ability(&self, id: Identifier) -> Option<bool> {
        match self.get(id)?.meta {
            WarcraftObjectMeta::Ability(ref meta) => Some(meta.is_ultimate()),
            _ => None,
        }
    }

    pub fn get_ability_cooldown_for_level(&self, id: Identifier, level: usize) -> Option<u32> {
        match self.get(id)?.meta {
            WarcraftObjectMeta::Ability(ref meta) => meta.cooldown_for_level(level),
            _ => None,
        }
    }

    pub fn get_ability_base_cooldown(&self, id: Identifier) -> Option<u32> {
        match self.get(id)?.meta {
            WarcraftObjectMeta::Ability(ref meta) => Some(meta.base_cooldown()),
            _ => None,
        }
    }

    pub fn get_ability_cooldowns(&self, id: Identifier) -> Option<[u32; 4]> {
        match self.get(id)?.meta {
            WarcraftObjectMeta::Ability(ref meta) => Some(meta.cooldowns()),
            _ => None,
        }
    }

    pub fn get_unit_build_time(&self, id: Identifier) -> Option<u32> {
        match self.get(id)?.meta {
            WarcraftObjectMeta::Unit(ref meta) => Some(meta.build_time()),
            _ => None,
        }
    }

    pub fn ability_names(&self) -> impl Iterator<Item = (&'static str, &'static [&'static str])> {
        self.db.iter().filter_map(|(id, object)| {
            if object.kind == WarcraftObjectKind::Ability {
                Some((id.value, object.names))
            } else {
                None
            }
        })
    }

    pub fn all_ability_names(&'static self) -> impl Iterator<Item = &'static str> {
        self.db.values().filter_map(|object| {
            if object.kind == WarcraftObjectKind::Ability {
                object.names.first().copied()
            } else {
                None
            }
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (&WarcraftObjectId, &WarcraftObject)> {
        self.db.iter()
    }
}

impl Borrow<str> for WarcraftObjectId {
    fn borrow(&self) -> &str {
        self.value
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WarcraftObjectId {
    value: &'static str,
}

impl WarcraftObjectId {
    pub const fn new(value: &'static str) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &'static str {
        self.value
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WarcraftObjectText {
    tip_levels: &'static [&'static str],
    ubertip_levels: &'static [&'static str],
    un_tip: Option<&'static str>,
    un_ubertip: Option<&'static str>,
}

impl WarcraftObjectText {
    pub const fn new(
        tip_levels: &'static [&'static str],
        ubertip_levels: &'static [&'static str],
    ) -> Self {
        Self {
            tip_levels,
            ubertip_levels,
            un_tip: None,
            un_ubertip: None,
        }
    }

    pub const fn with_alt(
        tip_levels: &'static [&'static str],
        ubertip_levels: &'static [&'static str],
        un_tip: Option<&'static str>,
        un_ubertip: Option<&'static str>,
    ) -> Self {
        Self {
            tip_levels,
            ubertip_levels,
            un_tip,
            un_ubertip,
        }
    }

    pub fn tip_levels(&self) -> &'static [&'static str] {
        self.tip_levels
    }

    pub fn ubertip_levels(&self) -> &'static [&'static str] {
        self.ubertip_levels
    }

    pub fn un_tip(&self) -> Option<&'static str> {
        self.un_tip
    }

    pub fn un_ubertip(&self) -> Option<&'static str> {
        self.un_ubertip
    }
}

#[derive(Default, Debug, Clone)]
pub struct WarcraftObject {
    id: WarcraftObjectId,
    names: &'static [&'static str],
    icons: &'static [&'static str],
    kind: WarcraftObjectKind,
    race: Option<Race>,
    meta: WarcraftObjectMeta,
    tip_levels: &'static [&'static str],
    ubertip_levels: &'static [&'static str],
    un_tip: Option<&'static str>,
    un_ubertip: Option<&'static str>,
    default_button_position: Option<ButtonPosition>,
    default_research_button_position: Option<ButtonPosition>,
}

impl WarcraftObject {
    pub fn new(
        id: WarcraftObjectId,
        names: &'static [&'static str],
        icons: &'static [&'static str],
        kind: WarcraftObjectKind,
        race: Option<Race>,
        meta: WarcraftObjectMeta,
    ) -> Self {
        Self {
            id,
            names,
            icons,
            kind,
            race,
            meta,
            tip_levels: &[],
            ubertip_levels: &[],
            un_tip: None,
            un_ubertip: None,
            default_button_position: None,
            default_research_button_position: None,
        }
    }

    pub fn with_text(
        id: WarcraftObjectId,
        names: &'static [&'static str],
        icons: &'static [&'static str],
        kind: WarcraftObjectKind,
        race: Option<Race>,
        meta: WarcraftObjectMeta,
        text: WarcraftObjectText,
    ) -> Self {
        Self {
            id,
            names,
            icons,
            kind,
            race,
            meta,
            tip_levels: text.tip_levels,
            ubertip_levels: text.ubertip_levels,
            un_tip: text.un_tip,
            un_ubertip: text.un_ubertip,
            default_button_position: None,
            default_research_button_position: None,
        }
    }

    pub fn with_default_position(mut self, position: Option<ButtonPosition>) -> Self {
        self.default_button_position = position;
        self
    }

    pub fn with_default_research_position(mut self, position: Option<ButtonPosition>) -> Self {
        self.default_research_button_position = position;
        self
    }

    pub fn id(&self) -> WarcraftObjectId {
        self.id
    }

    pub fn names(&self) -> &'static [&'static str] {
        self.names
    }

    pub fn icons(&self) -> &'static [&'static str] {
        self.icons
    }

    pub fn kind(&self) -> WarcraftObjectKind {
        self.kind
    }

    pub fn race(&self) -> Option<Race> {
        self.race
    }

    pub fn meta(&self) -> &WarcraftObjectMeta {
        &self.meta
    }

    pub fn tip(&self) -> Option<&'static str> {
        if let Some(first) = self.tip_levels.first() {
            return Some(*first);
        }
        if let WarcraftObjectMeta::Command(command_meta) = &self.meta {
            return command_meta.tip();
        }
        None
    }

    pub fn ubertip(&self) -> Option<&'static str> {
        if let Some(first) = self.ubertip_levels.first() {
            return Some(*first);
        }
        match &self.meta {
            WarcraftObjectMeta::Ability(ability_meta) => ability_meta.ubertip(),
            WarcraftObjectMeta::Command(command_meta) => command_meta.ubertip(),
            _ => None,
        }
    }

    pub fn tip_levels(&self) -> &'static [&'static str] {
        self.tip_levels
    }

    pub fn ubertip_levels(&self) -> &'static [&'static str] {
        self.ubertip_levels
    }

    pub fn research_ubertip(&self) -> Option<&'static str> {
        if let WarcraftObjectMeta::Ability(ability_meta) = &self.meta {
            return ability_meta.research_ubertip();
        }
        None
    }

    pub fn un_tip(&self) -> Option<&'static str> {
        self.un_tip
    }

    pub fn un_ubertip(&self) -> Option<&'static str> {
        self.un_ubertip
    }

    pub fn default_button_position(&self) -> Option<ButtonPosition> {
        if let Some(position) = self.default_button_position {
            return Some(position);
        }
        match &self.meta {
            WarcraftObjectMeta::Ability(ability_meta) => ability_meta.default_button_position(),
            WarcraftObjectMeta::Command(command_meta) => command_meta.default_button_position(),
            _ => None,
        }
    }

    pub fn default_research_button_position(&self) -> Option<ButtonPosition> {
        if let Some(position) = self.default_research_button_position {
            return Some(position);
        }
        match &self.meta {
            WarcraftObjectMeta::Ability(ability_meta) => {
                ability_meta.default_research_button_position()
            }
            _ => None,
        }
    }

    pub fn is_ultimate_ability(&self) -> bool {
        match self.meta() {
            WarcraftObjectMeta::Ability(ability_meta) => ability_meta.is_ultimate(),
            _ => false,
        }
    }

    pub fn cooldowns(&self) -> Option<[u32; 4]> {
        match self.meta() {
            WarcraftObjectMeta::Ability(ability_meta) => Some(ability_meta.cooldowns),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum WarcraftObjectMeta {
    Unit(UnitMeta),
    Ability(AbilityMeta),
    Upgrade(UpgradeMeta),
    Item(ItemMeta),
    Command(CommandMeta),
}

#[derive(Default, Debug, Clone)]
pub struct CommandMeta {
    default_button_position: Option<ButtonPosition>,
    tip: Option<&'static str>,
    ubertip: Option<&'static str>,
}

impl CommandMeta {
    pub const fn new(default_button_position: Option<ButtonPosition>) -> Self {
        Self {
            default_button_position,
            tip: None,
            ubertip: None,
        }
    }

    pub const fn with_text(
        default_button_position: Option<ButtonPosition>,
        tip: Option<&'static str>,
        ubertip: Option<&'static str>,
    ) -> Self {
        Self {
            default_button_position,
            tip,
            ubertip,
        }
    }

    pub fn default_button_position(&self) -> Option<ButtonPosition> {
        self.default_button_position
    }

    pub fn tip(&self) -> Option<&'static str> {
        self.tip
    }

    pub fn ubertip(&self) -> Option<&'static str> {
        self.ubertip
    }
}

impl Default for WarcraftObjectMeta {
    fn default() -> Self {
        Self::Unit(UnitMeta::default())
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ButtonPosition {
    column: u8,
    row: u8,
}

impl ButtonPosition {
    pub const fn new(column: u8, row: u8) -> Self {
        Self { column, row }
    }

    pub fn column(&self) -> u8 {
        self.column
    }

    pub fn row(&self) -> u8 {
        self.row
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UnitProduction {
    researches: &'static [WarcraftObjectId],
    builds: &'static [WarcraftObjectId],
    trains: &'static [WarcraftObjectId],
    sell_items: &'static [WarcraftObjectId],
    sell_units: &'static [WarcraftObjectId],
}

impl UnitProduction {
    pub const EMPTY: UnitProduction = UnitProduction {
        researches: &[],
        builds: &[],
        trains: &[],
        sell_items: &[],
        sell_units: &[],
    };

    pub const fn new(
        researches: &'static [WarcraftObjectId],
        builds: &'static [WarcraftObjectId],
        trains: &'static [WarcraftObjectId],
        sell_items: &'static [WarcraftObjectId],
        sell_units: &'static [WarcraftObjectId],
    ) -> Self {
        Self {
            researches,
            builds,
            trains,
            sell_items,
            sell_units,
        }
    }

    pub fn researches(&self) -> &'static [WarcraftObjectId] {
        self.researches
    }

    pub fn builds(&self) -> &'static [WarcraftObjectId] {
        self.builds
    }

    pub fn trains(&self) -> &'static [WarcraftObjectId] {
        self.trains
    }

    pub fn sell_items(&self) -> &'static [WarcraftObjectId] {
        self.sell_items
    }

    pub fn sell_units(&self) -> &'static [WarcraftObjectId] {
        self.sell_units
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UnitFlags {
    is_campaign: bool,
    is_in_editor: bool,
    is_hidden_in_editor: bool,
    is_special: bool,
}

impl UnitFlags {
    pub const EDITOR_ONLY: UnitFlags = UnitFlags {
        is_campaign: false,
        is_in_editor: true,
        is_hidden_in_editor: false,
        is_special: false,
    };

    pub const fn new(
        is_campaign: bool,
        is_in_editor: bool,
        is_hidden_in_editor: bool,
        is_special: bool,
    ) -> Self {
        Self {
            is_campaign,
            is_in_editor,
            is_hidden_in_editor,
            is_special,
        }
    }

    pub fn is_campaign(&self) -> bool {
        self.is_campaign
    }

    pub fn is_in_editor(&self) -> bool {
        self.is_in_editor
    }

    pub fn is_hidden_in_editor(&self) -> bool {
        self.is_hidden_in_editor
    }

    pub fn is_special(&self) -> bool {
        self.is_special
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryAttribute {
    #[default]
    Strength,
    Agility,
    Intelligence,
}

impl PrimaryAttribute {
    pub fn parse(raw: &str) -> Option<PrimaryAttribute> {
        let normalized = raw.trim().to_ascii_uppercase();
        match normalized.as_str() {
            "STR" => Some(PrimaryAttribute::Strength),
            "AGI" => Some(PrimaryAttribute::Agility),
            "INT" => Some(PrimaryAttribute::Intelligence),
            _ => None,
        }
    }
}

impl std::fmt::Display for PrimaryAttribute {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            PrimaryAttribute::Strength => "Strength",
            PrimaryAttribute::Agility => "Agility",
            PrimaryAttribute::Intelligence => "Intelligence",
        };
        formatter.write_str(label)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackType {
    Normal,
    Pierce,
    Siege,
    Magic,
    Chaos,
    Hero,
    Spells,
    #[default]
    Unknown,
}

impl AttackType {
    pub fn parse(raw: &str) -> AttackType {
        let normalized = raw.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "normal" => AttackType::Normal,
            "pierce" => AttackType::Pierce,
            "siege" => AttackType::Siege,
            "magic" => AttackType::Magic,
            "chaos" => AttackType::Chaos,
            "hero" => AttackType::Hero,
            "spells" => AttackType::Spells,
            _ => AttackType::Unknown,
        }
    }
}

impl std::fmt::Display for AttackType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            AttackType::Normal => "Normal",
            AttackType::Pierce => "Piercing",
            AttackType::Siege => "Siege",
            AttackType::Magic => "Magic",
            AttackType::Chaos => "Chaos",
            AttackType::Hero => "Hero",
            AttackType::Spells => "Spells",
            AttackType::Unknown => "Unknown",
        };
        formatter.write_str(label)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefenseType {
    Light,
    Medium,
    Heavy,
    Fortified,
    Normal,
    Hero,
    Divine,
    #[default]
    Unarmored,
}

impl DefenseType {
    pub fn parse(raw: &str) -> DefenseType {
        let normalized = raw.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "small" | "light" => DefenseType::Light,
            "medium" => DefenseType::Medium,
            "large" | "heavy" => DefenseType::Heavy,
            "fort" | "fortified" => DefenseType::Fortified,
            "normal" => DefenseType::Normal,
            "hero" => DefenseType::Hero,
            "divine" => DefenseType::Divine,
            _ => DefenseType::Unarmored,
        }
    }

    pub const fn all() -> [DefenseType; 8] {
        [
            DefenseType::Light,
            DefenseType::Medium,
            DefenseType::Heavy,
            DefenseType::Fortified,
            DefenseType::Normal,
            DefenseType::Hero,
            DefenseType::Divine,
            DefenseType::Unarmored,
        ]
    }
}

impl std::fmt::Display for DefenseType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            DefenseType::Light => "Light",
            DefenseType::Medium => "Medium",
            DefenseType::Heavy => "Heavy",
            DefenseType::Fortified => "Fortified",
            DefenseType::Normal => "Normal",
            DefenseType::Hero => "Hero",
            DefenseType::Divine => "Divine",
            DefenseType::Unarmored => "Unarmored",
        };
        formatter.write_str(label)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct UnitAttack {
    damage_min: u32,
    damage_max: u32,
    range: u32,
    cooldown_seconds: f32,
    attack_type: AttackType,
}

impl UnitAttack {
    pub const fn new(
        damage_min: u32,
        damage_max: u32,
        range: u32,
        cooldown_seconds: f32,
        attack_type: AttackType,
    ) -> Self {
        Self {
            damage_min,
            damage_max,
            range,
            cooldown_seconds,
            attack_type,
        }
    }

    pub fn damage_min(&self) -> u32 {
        self.damage_min
    }

    pub fn damage_max(&self) -> u32 {
        self.damage_max
    }

    pub fn range(&self) -> u32 {
        self.range
    }

    pub fn cooldown_seconds(&self) -> f32 {
        self.cooldown_seconds
    }

    pub fn attack_type(&self) -> AttackType {
        self.attack_type
    }
}

// Mirrors the `regenType` column in `unitbalance.slk`. Controls when HP
// regeneration is active; the rate (`hit_points_regen`) is the per-second
// value WHILE active, with no day/night multiplier on top of it.
//
//   Always — regenerates anywhere, anytime (Human, Orc, neutral creeps).
//   Night  — regenerates only between dusk and dawn (Night Elf).
//   Blight — regenerates only while standing on blight (Undead).
//   None   — does not regenerate HP at all (some neutral structures /
//            mechanical creeps).
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegenType {
    #[default]
    Always,
    Night,
    Blight,
    None,
}

impl RegenType {
    pub fn parse(raw_value: &str) -> Self {
        match raw_value.trim().to_ascii_lowercase().as_str() {
            "always" => Self::Always,
            "night" => Self::Night,
            "blight" => Self::Blight,
            "none" | "" | "-" | "_" => Self::None,
            _ => Self::Always,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct UnitCombat {
    hit_points: u32,
    hit_points_regen: f32,
    regen_type: RegenType,
    armor: f32,
    defense_type: DefenseType,
    attack: Option<UnitAttack>,
    mana_pool: Option<ManaPool>,
}

impl UnitCombat {
    pub const EMPTY: UnitCombat = UnitCombat {
        hit_points: 0,
        hit_points_regen: 0.0,
        regen_type: RegenType::Always,
        armor: 0.0,
        defense_type: DefenseType::Unarmored,
        attack: None,
        mana_pool: None,
    };

    pub const fn new(
        hit_points: u32,
        hit_points_regen: f32,
        regen_type: RegenType,
        armor: f32,
        defense_type: DefenseType,
        attack: Option<UnitAttack>,
    ) -> Self {
        Self {
            hit_points,
            hit_points_regen,
            regen_type,
            armor,
            defense_type,
            attack,
            mana_pool: None,
        }
    }

    pub const fn with_mana_pool(mut self, mana_pool: ManaPool) -> Self {
        self.mana_pool = Some(mana_pool);
        self
    }

    pub fn hit_points(&self) -> u32 {
        self.hit_points
    }

    pub fn hit_points_regen(&self) -> f32 {
        self.hit_points_regen
    }

    pub fn regen_type(&self) -> RegenType {
        self.regen_type
    }

    pub fn armor(&self) -> f32 {
        self.armor
    }

    pub fn defense_type(&self) -> DefenseType {
        self.defense_type
    }

    pub fn attack(&self) -> Option<&UnitAttack> {
        self.attack.as_ref()
    }

    pub fn mana_pool(&self) -> Option<ManaPool> {
        self.mana_pool
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ManaPool {
    mana: u32,
    mana_regen: f32,
}

impl ManaPool {
    pub const fn new(mana: u32, mana_regen: f32) -> Self {
        Self { mana, mana_regen }
    }

    pub fn mana(&self) -> u32 {
        self.mana
    }

    pub fn mana_regen(&self) -> f32 {
        self.mana_regen
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttributeBase {
    strength: u32,
    agility: u32,
    intelligence: u32,
}

impl AttributeBase {
    pub const fn new(strength: u32, agility: u32, intelligence: u32) -> Self {
        Self {
            strength,
            agility,
            intelligence,
        }
    }

    pub fn strength(&self) -> u32 {
        self.strength
    }

    pub fn agility(&self) -> u32 {
        self.agility
    }

    pub fn intelligence(&self) -> u32 {
        self.intelligence
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttributeGrowth {
    strength_per_level: f32,
    agility_per_level: f32,
    intelligence_per_level: f32,
}

impl AttributeGrowth {
    pub const fn new(
        strength_per_level: f32,
        agility_per_level: f32,
        intelligence_per_level: f32,
    ) -> Self {
        Self {
            strength_per_level,
            agility_per_level,
            intelligence_per_level,
        }
    }

    pub fn strength_per_level(&self) -> f32 {
        self.strength_per_level
    }

    pub fn agility_per_level(&self) -> f32 {
        self.agility_per_level
    }

    pub fn intelligence_per_level(&self) -> f32 {
        self.intelligence_per_level
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeroAttributes {
    mana_pool: ManaPool,
    base: AttributeBase,
    growth: AttributeGrowth,
    primary: PrimaryAttribute,
}

impl HeroAttributes {
    pub const fn new(
        mana_pool: ManaPool,
        base: AttributeBase,
        growth: AttributeGrowth,
        primary: PrimaryAttribute,
    ) -> Self {
        Self {
            mana_pool,
            base,
            growth,
            primary,
        }
    }

    pub fn mana_pool(&self) -> ManaPool {
        self.mana_pool
    }

    pub fn base(&self) -> AttributeBase {
        self.base
    }

    pub fn growth(&self) -> AttributeGrowth {
        self.growth
    }

    pub fn mana(&self) -> u32 {
        self.mana_pool.mana
    }

    pub fn mana_regen(&self) -> f32 {
        self.mana_pool.mana_regen
    }

    pub fn strength(&self) -> u32 {
        self.base.strength
    }

    pub fn agility(&self) -> u32 {
        self.base.agility
    }

    pub fn intelligence(&self) -> u32 {
        self.base.intelligence
    }

    pub fn primary(&self) -> PrimaryAttribute {
        self.primary
    }

    pub fn strength_per_level(&self) -> f32 {
        self.growth.strength_per_level
    }

    pub fn agility_per_level(&self) -> f32 {
        self.growth.agility_per_level
    }

    pub fn intelligence_per_level(&self) -> f32 {
        self.growth.intelligence_per_level
    }
}

#[derive(Default, Debug, Clone)]
pub struct UnitMeta {
    unit_kind: UnitKind,
    build_time: u32,
    abilities: &'static [WarcraftObjectId],
    hero_abilities: &'static [WarcraftObjectId],
    researches: &'static [WarcraftObjectId],
    builds: &'static [WarcraftObjectId],
    trains: &'static [WarcraftObjectId],
    sell_items: &'static [WarcraftObjectId],
    sell_units: &'static [WarcraftObjectId],
    is_campaign: bool,
    is_in_editor: bool,
    is_hidden_in_editor: bool,
    is_special: bool,
    combat: UnitCombat,
    hero_attributes: Option<HeroAttributes>,
}

impl UnitMeta {
    pub const fn new(unit_kind: UnitKind, build_time: u32) -> Self {
        Self {
            unit_kind,
            build_time,
            abilities: &[],
            hero_abilities: &[],
            researches: &[],
            builds: &[],
            trains: &[],
            sell_items: &[],
            sell_units: &[],
            is_campaign: false,
            is_in_editor: true,
            is_hidden_in_editor: false,
            is_special: false,
            combat: UnitCombat::EMPTY,
            hero_attributes: None,
        }
    }

    pub const fn with_abilities(
        unit_kind: UnitKind,
        build_time: u32,
        abilities: &'static [WarcraftObjectId],
        hero_abilities: &'static [WarcraftObjectId],
    ) -> Self {
        Self {
            unit_kind,
            build_time,
            abilities,
            hero_abilities,
            researches: &[],
            builds: &[],
            trains: &[],
            sell_items: &[],
            sell_units: &[],
            is_campaign: false,
            is_in_editor: true,
            is_hidden_in_editor: false,
            is_special: false,
            combat: UnitCombat::EMPTY,
            hero_attributes: None,
        }
    }

    pub const fn with_full(
        unit_kind: UnitKind,
        build_time: u32,
        abilities: &'static [WarcraftObjectId],
        hero_abilities: &'static [WarcraftObjectId],
        is_campaign: bool,
        is_in_editor: bool,
        is_special: bool,
    ) -> Self {
        Self {
            unit_kind,
            build_time,
            abilities,
            hero_abilities,
            researches: &[],
            builds: &[],
            trains: &[],
            sell_items: &[],
            sell_units: &[],
            is_campaign,
            is_in_editor,
            is_hidden_in_editor: false,
            is_special,
            combat: UnitCombat::EMPTY,
            hero_attributes: None,
        }
    }

    pub const fn with_full_and_extras(
        unit_kind: UnitKind,
        build_time: u32,
        abilities: &'static [WarcraftObjectId],
        hero_abilities: &'static [WarcraftObjectId],
        production: UnitProduction,
        flags: UnitFlags,
    ) -> Self {
        Self {
            unit_kind,
            build_time,
            abilities,
            hero_abilities,
            researches: production.researches,
            builds: production.builds,
            trains: production.trains,
            sell_items: production.sell_items,
            sell_units: production.sell_units,
            is_campaign: flags.is_campaign,
            is_in_editor: flags.is_in_editor,
            is_hidden_in_editor: flags.is_hidden_in_editor,
            is_special: flags.is_special,
            combat: UnitCombat::EMPTY,
            hero_attributes: None,
        }
    }

    pub const fn with_combat(mut self, combat: UnitCombat) -> Self {
        self.combat = combat;
        self
    }

    pub const fn with_hero_attributes(mut self, hero_attributes: HeroAttributes) -> Self {
        self.hero_attributes = Some(hero_attributes);
        self
    }

    pub fn unit_kind(&self) -> UnitKind {
        self.unit_kind
    }

    pub fn build_time(&self) -> u32 {
        self.build_time
    }

    pub fn abilities(&self) -> &'static [WarcraftObjectId] {
        self.abilities
    }

    pub fn hero_abilities(&self) -> &'static [WarcraftObjectId] {
        self.hero_abilities
    }

    pub fn builds(&self) -> &'static [WarcraftObjectId] {
        self.builds
    }

    pub fn trains(&self) -> &'static [WarcraftObjectId] {
        self.trains
    }

    pub fn is_campaign(&self) -> bool {
        self.is_campaign
    }

    pub fn is_in_editor(&self) -> bool {
        self.is_in_editor
    }

    pub fn is_hidden_in_editor(&self) -> bool {
        self.is_hidden_in_editor
    }

    pub fn is_special(&self) -> bool {
        self.is_special
    }

    pub fn researches(&self) -> &'static [WarcraftObjectId] {
        self.researches
    }

    pub fn sell_items(&self) -> &'static [WarcraftObjectId] {
        self.sell_items
    }

    pub fn sell_units(&self) -> &'static [WarcraftObjectId] {
        self.sell_units
    }

    pub fn is_melee_visible(&self) -> bool {
        self.is_in_editor && !self.is_campaign
    }

    pub fn combat(&self) -> &UnitCombat {
        &self.combat
    }

    pub fn hero_attributes(&self) -> Option<&HeroAttributes> {
        self.hero_attributes.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DamageEffectiveness {
    // Eight multipliers, one per defense type, in the order returned by
    // `DefenseType::all()`: Light, Medium, Heavy, Fortified, Normal, Hero,
    // Divine, Unarmored. Sourced from `DamageBonus*` lines in
    // `war3.w3mod:units/miscgame.txt`.
    multipliers: [f32; 8],
}

impl DamageEffectiveness {
    pub const fn new(multipliers: [f32; 8]) -> Self {
        Self { multipliers }
    }

    pub fn against(&self, defense_type: DefenseType) -> f32 {
        let defense_types = DefenseType::all();
        let mut iterator_index = 0;
        while iterator_index < defense_types.len() {
            if defense_types[iterator_index] == defense_type {
                return self.multipliers[iterator_index];
            }
            iterator_index += 1;
        }
        1.0
    }

    pub fn multipliers(&self) -> &[f32; 8] {
        &self.multipliers
    }
}

impl Default for DamageEffectiveness {
    fn default() -> Self {
        Self::new([1.0; 8])
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrengthBonuses {
    attack_bonus: f32,
    hit_point_bonus: u32,
    regen_bonus: f32,
}

impl StrengthBonuses {
    pub const fn new(attack_bonus: f32, hit_point_bonus: u32, regen_bonus: f32) -> Self {
        Self {
            attack_bonus,
            hit_point_bonus,
            regen_bonus,
        }
    }

    pub fn attack_bonus(&self) -> f32 {
        self.attack_bonus
    }

    pub fn hit_point_bonus(&self) -> u32 {
        self.hit_point_bonus
    }

    pub fn regen_bonus(&self) -> f32 {
        self.regen_bonus
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntelligenceBonuses {
    mana_bonus: u32,
    regen_bonus: f32,
}

impl IntelligenceBonuses {
    pub const fn new(mana_bonus: u32, regen_bonus: f32) -> Self {
        Self {
            mana_bonus,
            regen_bonus,
        }
    }

    pub fn mana_bonus(&self) -> u32 {
        self.mana_bonus
    }

    pub fn regen_bonus(&self) -> f32 {
        self.regen_bonus
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AgilityBonuses {
    defense_bonus: f32,
    attack_speed_bonus: f32,
}

impl AgilityBonuses {
    pub const fn new(defense_bonus: f32, attack_speed_bonus: f32) -> Self {
        Self {
            defense_bonus,
            attack_speed_bonus,
        }
    }

    pub fn defense_bonus(&self) -> f32 {
        self.defense_bonus
    }

    pub fn attack_speed_bonus(&self) -> f32 {
        self.attack_speed_bonus
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DamageMatrix {
    normal: DamageEffectiveness,
    pierce: DamageEffectiveness,
    siege: DamageEffectiveness,
    magic: DamageEffectiveness,
    chaos: DamageEffectiveness,
    spells: DamageEffectiveness,
    hero: DamageEffectiveness,
}

impl DamageMatrix {
    pub const fn new(
        normal: DamageEffectiveness,
        pierce: DamageEffectiveness,
        siege: DamageEffectiveness,
        magic: DamageEffectiveness,
        chaos: DamageEffectiveness,
        spells: DamageEffectiveness,
        hero: DamageEffectiveness,
    ) -> Self {
        Self {
            normal,
            pierce,
            siege,
            magic,
            chaos,
            spells,
            hero,
        }
    }

    pub fn effectiveness(&self, attack_type: AttackType) -> DamageEffectiveness {
        match attack_type {
            AttackType::Normal => self.normal,
            AttackType::Pierce => self.pierce,
            AttackType::Siege => self.siege,
            AttackType::Magic => self.magic,
            AttackType::Chaos => self.chaos,
            AttackType::Spells => self.spells,
            AttackType::Hero => self.hero,
            AttackType::Unknown => DamageEffectiveness::new([1.0; 8]),
        }
    }

    pub fn normal(&self) -> DamageEffectiveness {
        self.normal
    }

    pub fn pierce(&self) -> DamageEffectiveness {
        self.pierce
    }

    pub fn siege(&self) -> DamageEffectiveness {
        self.siege
    }

    pub fn magic(&self) -> DamageEffectiveness {
        self.magic
    }

    pub fn chaos(&self) -> DamageEffectiveness {
        self.chaos
    }

    pub fn spells(&self) -> DamageEffectiveness {
        self.spells
    }

    pub fn hero(&self) -> DamageEffectiveness {
        self.hero
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GameplayConstants {
    // Defaults below mirror the standard WC3 Reforged values from
    // war3.w3mod:units/miscgame.txt; used only when extraction is missing
    // the field, so the runtime never sees an all-zero `GameplayConstants`.
    strength_bonuses: StrengthBonuses,
    intelligence_bonuses: IntelligenceBonuses,
    agility_bonuses: AgilityBonuses,
    max_hero_level: u32,
    damage_matrix: DamageMatrix,
}

impl GameplayConstants {
    pub const fn new(
        strength_bonuses: StrengthBonuses,
        intelligence_bonuses: IntelligenceBonuses,
        agility_bonuses: AgilityBonuses,
        max_hero_level: u32,
        damage_matrix: DamageMatrix,
    ) -> Self {
        Self {
            strength_bonuses,
            intelligence_bonuses,
            agility_bonuses,
            max_hero_level,
            damage_matrix,
        }
    }

    pub fn strength_bonuses(&self) -> StrengthBonuses {
        self.strength_bonuses
    }

    pub fn intelligence_bonuses(&self) -> IntelligenceBonuses {
        self.intelligence_bonuses
    }

    pub fn agility_bonuses(&self) -> AgilityBonuses {
        self.agility_bonuses
    }

    pub fn damage_matrix(&self) -> DamageMatrix {
        self.damage_matrix
    }

    pub fn damage_effectiveness(&self, attack_type: AttackType) -> DamageEffectiveness {
        self.damage_matrix.effectiveness(attack_type)
    }

    pub fn str_attack_bonus(&self) -> f32 {
        self.strength_bonuses.attack_bonus
    }

    pub fn str_hit_point_bonus(&self) -> u32 {
        self.strength_bonuses.hit_point_bonus
    }

    pub fn str_regen_bonus(&self) -> f32 {
        self.strength_bonuses.regen_bonus
    }

    pub fn int_mana_bonus(&self) -> u32 {
        self.intelligence_bonuses.mana_bonus
    }

    pub fn int_regen_bonus(&self) -> f32 {
        self.intelligence_bonuses.regen_bonus
    }

    pub fn agi_defense_bonus(&self) -> f32 {
        self.agility_bonuses.defense_bonus
    }

    pub fn agi_attack_speed_bonus(&self) -> f32 {
        self.agility_bonuses.attack_speed_bonus
    }

    pub fn max_hero_level(&self) -> u32 {
        self.max_hero_level
    }
}

impl Default for GameplayConstants {
    fn default() -> Self {
        // SMALL, MEDIUM, LARGE, FORT, NORMAL, HERO, DIVINE, NONE — matches
        // miscgame.txt DamageBonus* line order.
        let damage_normal =
            DamageEffectiveness::new([1.00, 1.50, 1.00, 0.70, 1.00, 1.00, 0.05, 1.00]);
        let damage_pierce =
            DamageEffectiveness::new([2.00, 0.75, 1.00, 0.35, 1.00, 0.50, 0.05, 1.50]);
        let damage_siege =
            DamageEffectiveness::new([1.00, 0.50, 1.00, 1.50, 1.00, 0.50, 0.05, 1.50]);
        let damage_magic =
            DamageEffectiveness::new([1.25, 0.75, 2.00, 0.35, 1.00, 0.50, 0.05, 1.00]);
        let damage_chaos =
            DamageEffectiveness::new([1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00]);
        let damage_spells =
            DamageEffectiveness::new([1.00, 1.00, 1.00, 1.00, 1.00, 0.70, 0.05, 1.00]);
        let damage_hero =
            DamageEffectiveness::new([1.00, 1.00, 1.00, 0.50, 1.00, 1.00, 0.05, 1.00]);
        let damage_matrix = DamageMatrix::new(
            damage_normal,
            damage_pierce,
            damage_siege,
            damage_magic,
            damage_chaos,
            damage_spells,
            damage_hero,
        );
        let strength_bonuses = StrengthBonuses::new(1.0, 25, 0.05);
        let intelligence_bonuses = IntelligenceBonuses::new(15, 0.05);
        let agility_bonuses = AgilityBonuses::new(0.30, 0.02);
        Self::new(
            strength_bonuses,
            intelligence_bonuses,
            agility_bonuses,
            10,
            damage_matrix,
        )
    }
}

#[derive(Default, Debug, Clone)]
pub struct AbilityMeta {
    max_level: usize,
    is_ultimate: bool,
    cooldowns: [u32; 4],
    default_button_position: Option<ButtonPosition>,
    default_research_button_position: Option<ButtonPosition>,
    ubertip: Option<&'static str>,
    research_ubertip: Option<&'static str>,
    code: Option<&'static str>,
    morph_target_unit: Option<WarcraftObjectId>,
}

impl AbilityMeta {
    pub const fn new(max_level: usize, is_ultimate: bool, cooldowns: [u32; 4]) -> Self {
        Self {
            max_level,
            is_ultimate,
            cooldowns,
            default_button_position: None,
            default_research_button_position: None,
            ubertip: None,
            research_ubertip: None,
            code: None,
            morph_target_unit: None,
        }
    }

    pub const fn with_defaults(
        max_level: usize,
        is_ultimate: bool,
        cooldowns: [u32; 4],
        default_button_position: Option<ButtonPosition>,
        default_research_button_position: Option<ButtonPosition>,
    ) -> Self {
        Self {
            max_level,
            is_ultimate,
            cooldowns,
            default_button_position,
            default_research_button_position,
            ubertip: None,
            research_ubertip: None,
            code: None,
            morph_target_unit: None,
        }
    }

    pub const fn with_ubertips(
        max_level: usize,
        is_ultimate: bool,
        cooldowns: [u32; 4],
        default_button_position: Option<ButtonPosition>,
        default_research_button_position: Option<ButtonPosition>,
        ubertip: Option<&'static str>,
        research_ubertip: Option<&'static str>,
    ) -> Self {
        Self {
            max_level,
            is_ultimate,
            cooldowns,
            default_button_position,
            default_research_button_position,
            ubertip,
            research_ubertip,
            code: None,
            morph_target_unit: None,
        }
    }

    pub const fn with_code(mut self, code: Option<&'static str>) -> Self {
        self.code = code;
        self
    }

    pub const fn with_morph_target(mut self, target: Option<WarcraftObjectId>) -> Self {
        self.morph_target_unit = target;
        self
    }

    pub fn ubertip(&self) -> Option<&'static str> {
        self.ubertip
    }

    pub fn research_ubertip(&self) -> Option<&'static str> {
        self.research_ubertip
    }

    /// Game-mechanic class as listed in `units/abilitydata.slk`'s `code`
    /// column. Independent of the per-unit alias — e.g. multiple aliases
    /// can resolve to `code = "Apit"` (Purchase Item / shop button).
    pub fn code(&self) -> Option<&'static str> {
        self.code
    }

    /// For one-way morph abilities (Avenger Form, Crow Form, etc.) the
    /// unit id this ability transforms its caster into. Sourced from the
    /// `UnitID1` column of `abilitydata.slk`.
    pub fn morph_target_unit(&self) -> Option<&WarcraftObjectId> {
        self.morph_target_unit.as_ref()
    }

    pub fn default_button_position(&self) -> Option<ButtonPosition> {
        self.default_button_position
    }

    pub fn default_research_button_position(&self) -> Option<ButtonPosition> {
        self.default_research_button_position
    }

    pub fn max_level(&self) -> usize {
        self.max_level
    }

    pub fn is_ultimate(&self) -> bool {
        self.is_ultimate
    }

    pub fn cooldown_for_level(&self, level: usize) -> Option<u32> {
        if level == 0 || level > self.max_level {
            None
        } else {
            Some(self.cooldowns[level - 1])
        }
    }

    pub fn base_cooldown(&self) -> u32 {
        self.cooldowns[0]
    }

    pub fn cooldowns(&self) -> [u32; 4] {
        self.cooldowns
    }
}

#[derive(Default, Debug, Clone)]
pub struct UpgradeMeta {
    max_level: usize,
}

impl UpgradeMeta {
    pub fn new(max_level: usize) -> Self {
        Self { max_level }
    }

    pub fn max_level(&self) -> usize {
        self.max_level
    }
}

#[derive(Default, Debug, Clone)]
pub struct ItemMeta {
    class: ItemClass,
    abilities: &'static [WarcraftObjectId],
    cooldown_id: Option<WarcraftObjectId>,
}

impl ItemMeta {
    pub fn new(
        class: ItemClass,
        abilities: &'static [WarcraftObjectId],
        cooldown_id: Option<WarcraftObjectId>,
    ) -> Self {
        Self {
            class,
            abilities,
            cooldown_id,
        }
    }

    pub fn cooldown_id(&self) -> Option<WarcraftObjectId> {
        self.cooldown_id
    }

    pub fn abilities(&self) -> &'static [WarcraftObjectId] {
        self.abilities
    }

    pub fn class(&self) -> &ItemClass {
        &self.class
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemKeybindClass {
    Menu,
    ControlGroup,
    Game,
    Camera,
    Observer,
    Replay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemKeybindModifier {
    None,
    Alt,
    Ctrl,
    CtrlOrAlt,
    Shift,
}

#[derive(Debug, Clone, Copy)]
pub struct SystemKeybind {
    section_id: &'static str,
    comment: &'static str,
    default_hotkey: u32,
    default_modifier: SystemKeybindModifier,
    class: SystemKeybindClass,
}

impl SystemKeybind {
    pub const fn new(
        section_id: &'static str,
        comment: &'static str,
        default_hotkey: u32,
        default_modifier: SystemKeybindModifier,
        class: SystemKeybindClass,
    ) -> Self {
        Self {
            section_id,
            comment,
            default_hotkey,
            default_modifier,
            class,
        }
    }

    pub fn section_id(&self) -> &'static str {
        self.section_id
    }

    pub fn comment(&self) -> &'static str {
        self.comment
    }

    pub fn default_hotkey(&self) -> u32 {
        self.default_hotkey
    }

    pub fn default_modifier(&self) -> SystemKeybindModifier {
        self.default_modifier
    }

    pub fn class(&self) -> SystemKeybindClass {
        self.class
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchType {
    Melee(MeleeMatchType),
    Custom(CustomMatchType),
    Campaign(CampaignMatchType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeleeMatchType {
    OneVsOne,
    TwoVsTwo,
    ThreeVsThree,
    FourVsFour,
    FreeForAll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomMatchType {
    DirectStrike,
    Legion,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CampaignMatchType {}

pub type Team = BTreeMap<u32, TeamPlayer>;
pub type Teams = BTreeMap<u8, Team>;

#[derive(Default, Debug, Clone, Serialize)]
pub struct TeamPlayer {
    name: String,
    race_preference: RacePreference,
    state: PlayerSlotState,
    color: PlayerColor,
}

impl TeamPlayer {
    pub fn new(
        name: String,
        race_preference: RacePreference,
        state: PlayerSlotState,
        color: PlayerColor,
    ) -> Self {
        Self {
            name,
            race_preference,
            state,
            color,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn race_preference(&self) -> RacePreference {
        self.race_preference
    }

    pub fn state(&self) -> PlayerSlotState {
        self.state
    }

    pub fn color(&self) -> PlayerColor {
        self.color
    }
}
