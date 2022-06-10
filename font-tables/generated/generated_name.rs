// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

use font_types::*;

/// [Naming table version 0](https://docs.microsoft.com/en-us/typography/opentype/spec/name#naming-table-version-0)
pub struct Name0<'a> {
    version: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    count: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    storage_offset: zerocopy::LayoutVerified<&'a [u8], BigEndian<Offset16>>,
    name_record: zerocopy::LayoutVerified<&'a [u8], [NameRecord]>,
    offset_bytes: &'a [u8],
}

impl<'a> font_types::FontRead<'a> for Name0<'a> {
    fn read(bytes: &'a [u8]) -> Option<Self> {
        let offset_bytes = bytes;
        let (version, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let (count, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let __resolved_count = count.get();
        let (storage_offset, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<Offset16>>::new_unaligned_from_prefix(bytes)?;
        let (name_record, bytes) =
            zerocopy::LayoutVerified::<_, [NameRecord]>::new_slice_unaligned_from_prefix(
                bytes,
                __resolved_count as usize,
            )?;
        let _bytes = bytes;
        let result = Name0 {
            version,
            count,
            storage_offset,
            name_record,
            offset_bytes,
        };
        Some(result)
    }
}

impl<'a> Name0<'a> {
    /// Table version number (=0).
    pub fn version(&self) -> u16 {
        self.version.get()
    }

    /// Number of name records.
    pub fn count(&self) -> u16 {
        self.count.get()
    }

    /// Offset to start of string storage (from start of table).
    pub fn storage_offset(&self) -> Offset16 {
        self.storage_offset.get()
    }

    /// The name records where count is the number of records.
    pub fn name_record(&self) -> &[NameRecord] {
        &self.name_record
    }
}

impl<'a> font_types::OffsetHost<'a> for Name0<'a> {
    fn bytes(&self) -> &'a [u8] {
        self.offset_bytes
    }
}

/// [Naming table version 1](https://docs.microsoft.com/en-us/typography/opentype/spec/name#naming-table-version-1)
pub struct Name1<'a> {
    version: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    count: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    storage_offset: zerocopy::LayoutVerified<&'a [u8], BigEndian<Offset16>>,
    name_record: zerocopy::LayoutVerified<&'a [u8], [NameRecord]>,
    lang_tag_count: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    lang_tag_record: zerocopy::LayoutVerified<&'a [u8], [LangTagRecord]>,
    offset_bytes: &'a [u8],
}

impl<'a> font_types::FontRead<'a> for Name1<'a> {
    fn read(bytes: &'a [u8]) -> Option<Self> {
        let offset_bytes = bytes;
        let (version, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let (count, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let __resolved_count = count.get();
        let (storage_offset, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<Offset16>>::new_unaligned_from_prefix(bytes)?;
        let (name_record, bytes) =
            zerocopy::LayoutVerified::<_, [NameRecord]>::new_slice_unaligned_from_prefix(
                bytes,
                __resolved_count as usize,
            )?;
        let (lang_tag_count, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let __resolved_lang_tag_count = lang_tag_count.get();
        let (lang_tag_record, bytes) =
            zerocopy::LayoutVerified::<_, [LangTagRecord]>::new_slice_unaligned_from_prefix(
                bytes,
                __resolved_lang_tag_count as usize,
            )?;
        let _bytes = bytes;
        let result = Name1 {
            version,
            count,
            storage_offset,
            name_record,
            lang_tag_count,
            lang_tag_record,
            offset_bytes,
        };
        Some(result)
    }
}

impl<'a> Name1<'a> {
    /// Table version number (=0).
    pub fn version(&self) -> u16 {
        self.version.get()
    }

    /// Number of name records.
    pub fn count(&self) -> u16 {
        self.count.get()
    }

    /// Offset to start of string storage (from start of table).
    pub fn storage_offset(&self) -> Offset16 {
        self.storage_offset.get()
    }

    /// The name records where count is the number of records.
    pub fn name_record(&self) -> &[NameRecord] {
        &self.name_record
    }

    /// Number of language-tag records.
    pub fn lang_tag_count(&self) -> u16 {
        self.lang_tag_count.get()
    }

    /// The language-tag records where langTagCount is the number of records.
    pub fn lang_tag_record(&self) -> &[LangTagRecord] {
        &self.lang_tag_record
    }
}

impl<'a> font_types::OffsetHost<'a> for Name1<'a> {
    fn bytes(&self) -> &'a [u8] {
        self.offset_bytes
    }
}

pub enum Name<'a> {
    Version0(Name0<'a>),
    Version1(Name1<'a>),
}

impl<'a> font_types::FontRead<'a> for Name<'a> {
    fn read(bytes: &'a [u8]) -> Option<Self> {
        let version: BigEndian<u16> = font_types::FontRead::read(bytes)?;
        match version.get() {
            0 => Some(Self::Version0(font_types::FontRead::read(bytes)?)),
            1 => Some(Self::Version1(font_types::FontRead::read(bytes)?)),
            _other => {
                #[cfg(feature = "std")]
                {
                    eprintln!(
                        "unknown enum variant {:?} (table {})",
                        version,
                        stringify!(Name)
                    );
                }
                None
            }
        }
    }
}

impl<'a> Name<'a> {
    /// Number of name records.
    pub fn count(&self) -> u16 {
        match self {
            Self::Version0(_inner) => _inner.count(),
            Self::Version1(_inner) => _inner.count(),
        }
    }

    /// Number of language-tag records.
    pub fn lang_tag_count(&self) -> Option<u16> {
        match self {
            Self::Version0(_inner) => None,
            Self::Version1(_inner) => Some(_inner.lang_tag_count()),
        }
    }

    /// The language-tag records where langTagCount is the number of records.
    pub fn lang_tag_record(&self) -> Option<&[LangTagRecord]> {
        match self {
            Self::Version0(_inner) => None,
            Self::Version1(_inner) => Some(_inner.lang_tag_record()),
        }
    }

    /// The name records where count is the number of records.
    pub fn name_record(&self) -> &[NameRecord] {
        match self {
            Self::Version0(_inner) => _inner.name_record(),
            Self::Version1(_inner) => _inner.name_record(),
        }
    }

    /// Offset to start of string storage (from start of table).
    pub fn storage_offset(&self) -> Offset16 {
        match self {
            Self::Version0(_inner) => _inner.storage_offset(),
            Self::Version1(_inner) => _inner.storage_offset(),
        }
    }

    /// Table version number (=0).
    pub fn version(&self) -> u16 {
        match self {
            Self::Version0(_inner) => _inner.version(),
            Self::Version1(_inner) => _inner.version(),
        }
    }
}

impl<'a> font_types::OffsetHost<'a> for Name<'a> {
    fn bytes(&self) -> &'a [u8] {
        match self {
            Self::Version0(_inner) => _inner.bytes(),
            Self::Version1(_inner) => _inner.bytes(),
        }
    }
}

/// Part of [Name1]
#[derive(Clone, Copy, Debug, zerocopy :: FromBytes, zerocopy :: Unaligned)]
#[repr(C)]
pub struct LangTagRecord {
    /// Language-tag string length (in bytes)
    pub length: BigEndian<u16>,
    /// Language-tag string offset from start of storage area (in
    /// bytes).
    pub lang_tag_offset: BigEndian<Offset16>,
}

impl LangTagRecord {
    /// Language-tag string length (in bytes)
    pub fn length(&self) -> u16 {
        self.length.get()
    }

    /// Language-tag string offset from start of storage area (in
    /// bytes).
    pub fn lang_tag_offset(&self) -> Offset16 {
        self.lang_tag_offset.get()
    }
}

///[Name Records](https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-records)
#[derive(Clone, Copy, Debug, zerocopy :: FromBytes, zerocopy :: Unaligned)]
#[repr(C)]
pub struct NameRecord {
    /// Platform ID.
    pub platform_id: BigEndian<u16>,
    /// Platform-specific encoding ID.
    pub encoding_id: BigEndian<u16>,
    /// Language ID.
    pub language_id: BigEndian<u16>,
    /// Name ID.
    pub name_id: BigEndian<u16>,
    /// String length (in bytes).
    pub length: BigEndian<u16>,
    /// String offset from start of storage area (in bytes).
    pub string_offset: BigEndian<Offset16>,
}

impl NameRecord {
    /// Platform ID.
    pub fn platform_id(&self) -> u16 {
        self.platform_id.get()
    }

    /// Platform-specific encoding ID.
    pub fn encoding_id(&self) -> u16 {
        self.encoding_id.get()
    }

    /// Language ID.
    pub fn language_id(&self) -> u16 {
        self.language_id.get()
    }

    /// Name ID.
    pub fn name_id(&self) -> u16 {
        self.name_id.get()
    }

    /// String length (in bytes).
    pub fn length(&self) -> u16 {
        self.length.get()
    }

    /// String offset from start of storage area (in bytes).
    pub fn string_offset(&self) -> Offset16 {
        self.string_offset.get()
    }
}
