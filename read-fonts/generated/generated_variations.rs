// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

impl Format<u8> for DeltaSetIndexMapFormat0Marker {
    const FORMAT: u8 = 0;
}

/// The [DeltaSetIndexMap](https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#associating-target-items-to-variation-data) table format 0
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct DeltaSetIndexMapFormat0Marker {
    map_data_byte_len: usize,
}

impl DeltaSetIndexMapFormat0Marker {
    fn format_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u8::RAW_BYTE_LEN
    }
    fn entry_format_byte_range(&self) -> Range<usize> {
        let start = self.format_byte_range().end;
        start..start + u8::RAW_BYTE_LEN
    }
    fn map_count_byte_range(&self) -> Range<usize> {
        let start = self.entry_format_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn map_data_byte_range(&self) -> Range<usize> {
        let start = self.map_count_byte_range().end;
        start..start + self.map_data_byte_len
    }
}

impl<'a> FontRead<'a> for DeltaSetIndexMapFormat0<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u8>();
        let entry_format: u8 = cursor.read()?;
        let map_count: u16 = cursor.read()?;
        let map_data_byte_len = map_data_size(entry_format, map_count as u32) * u8::RAW_BYTE_LEN;
        cursor.advance_by(map_data_byte_len);
        cursor.finish(DeltaSetIndexMapFormat0Marker { map_data_byte_len })
    }
}

/// The [DeltaSetIndexMap](https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#associating-target-items-to-variation-data) table format 0
pub type DeltaSetIndexMapFormat0<'a> = TableRef<'a, DeltaSetIndexMapFormat0Marker>;

impl<'a> DeltaSetIndexMapFormat0<'a> {
    /// DeltaSetIndexMap format: set to 0.
    pub fn format(&self) -> u8 {
        let range = self.shape.format_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// A packed field that describes the compressed representation of
    /// delta-set indices. See details below.
    pub fn entry_format(&self) -> u8 {
        let range = self.shape.entry_format_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The number of mapping entries.
    pub fn map_count(&self) -> u16 {
        let range = self.shape.map_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The delta-set index mapping data. See details below.
    pub fn map_data(&self) -> &'a [u8] {
        let range = self.shape.map_data_byte_range();
        self.data.read_array(range).unwrap()
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for DeltaSetIndexMapFormat0<'a> {
    fn type_name(&self) -> &str {
        "DeltaSetIndexMapFormat0"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("format", self.format())),
            1usize => Some(Field::new("entry_format", self.entry_format())),
            2usize => Some(Field::new("map_count", self.map_count())),
            3usize => Some(Field::new("map_data", self.map_data())),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for DeltaSetIndexMapFormat0<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

impl Format<u8> for DeltaSetIndexMapFormat1Marker {
    const FORMAT: u8 = 1;
}

/// The [DeltaSetIndexMap](https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#associating-target-items-to-variation-data) table format 1
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct DeltaSetIndexMapFormat1Marker {
    map_data_byte_len: usize,
}

impl DeltaSetIndexMapFormat1Marker {
    fn format_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u8::RAW_BYTE_LEN
    }
    fn entry_format_byte_range(&self) -> Range<usize> {
        let start = self.format_byte_range().end;
        start..start + u8::RAW_BYTE_LEN
    }
    fn map_count_byte_range(&self) -> Range<usize> {
        let start = self.entry_format_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }
    fn map_data_byte_range(&self) -> Range<usize> {
        let start = self.map_count_byte_range().end;
        start..start + self.map_data_byte_len
    }
}

impl<'a> FontRead<'a> for DeltaSetIndexMapFormat1<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u8>();
        let entry_format: u8 = cursor.read()?;
        let map_count: u32 = cursor.read()?;
        let map_data_byte_len = map_data_size(entry_format, map_count) * u8::RAW_BYTE_LEN;
        cursor.advance_by(map_data_byte_len);
        cursor.finish(DeltaSetIndexMapFormat1Marker { map_data_byte_len })
    }
}

/// The [DeltaSetIndexMap](https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#associating-target-items-to-variation-data) table format 1
pub type DeltaSetIndexMapFormat1<'a> = TableRef<'a, DeltaSetIndexMapFormat1Marker>;

impl<'a> DeltaSetIndexMapFormat1<'a> {
    /// DeltaSetIndexMap format: set to 1.
    pub fn format(&self) -> u8 {
        let range = self.shape.format_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// A packed field that describes the compressed representation of
    /// delta-set indices. See details below.
    pub fn entry_format(&self) -> u8 {
        let range = self.shape.entry_format_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The number of mapping entries.
    pub fn map_count(&self) -> u32 {
        let range = self.shape.map_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The delta-set index mapping data. See details below.
    pub fn map_data(&self) -> &'a [u8] {
        let range = self.shape.map_data_byte_range();
        self.data.read_array(range).unwrap()
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for DeltaSetIndexMapFormat1<'a> {
    fn type_name(&self) -> &str {
        "DeltaSetIndexMapFormat1"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("format", self.format())),
            1usize => Some(Field::new("entry_format", self.entry_format())),
            2usize => Some(Field::new("map_count", self.map_count())),
            3usize => Some(Field::new("map_data", self.map_data())),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for DeltaSetIndexMapFormat1<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// The [DeltaSetIndexMap](https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#associating-target-items-to-variation-data) table
pub enum DeltaSetIndexMap<'a> {
    Format0(DeltaSetIndexMapFormat0<'a>),
    Format1(DeltaSetIndexMapFormat1<'a>),
}

impl<'a> FontRead<'a> for DeltaSetIndexMap<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let format: u8 = data.read_at(0)?;
        match format {
            DeltaSetIndexMapFormat0Marker::FORMAT => Ok(Self::Format0(FontRead::read(data)?)),
            DeltaSetIndexMapFormat1Marker::FORMAT => Ok(Self::Format1(FontRead::read(data)?)),
            other => Err(ReadError::InvalidFormat(other.into())),
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> DeltaSetIndexMap<'a> {
    fn dyn_inner<'b>(&'b self) -> &'b dyn SomeTable<'a> {
        match self {
            Self::Format0(table) => table,
            Self::Format1(table) => table,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for DeltaSetIndexMap<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.dyn_inner().fmt(f)
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for DeltaSetIndexMap<'a> {
    fn type_name(&self) -> &str {
        self.dyn_inner().type_name()
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        self.dyn_inner().get_field(idx)
    }
}

bitflags::bitflags! {
    /// Entry format for a [DeltaSetIndexMap].
    #[derive(Default)]
    pub struct EntryFormat: u8 {
        /// Mask for the low 4 bits, which give the count of bits minus one that are used in each entry for the inner-level index.
        const INNER_INDEX_BIT_COUNT_MASK = 0x0F;
        /// Mask for bits that indicate the size in bytes minus one of each entry.
        const MAP_ENTRY_SIZE_MASK = 0x30;
    }
}

impl font_types::Scalar for EntryFormat {
    type Raw = <u8 as font_types::Scalar>::Raw;
    fn to_raw(self) -> Self::Raw {
        self.bits().to_raw()
    }
    fn from_raw(raw: Self::Raw) -> Self {
        let t = <u8>::from_raw(raw);
        Self::from_bits_truncate(t)
    }
}

#[cfg(feature = "traversal")]
impl<'a> From<EntryFormat> for FieldType<'a> {
    fn from(src: EntryFormat) -> FieldType<'a> {
        src.bits().into()
    }
}

/// The [VariationRegionList](https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#variation-regions) table
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct VariationRegionListMarker {
    variation_regions_byte_len: usize,
}

impl VariationRegionListMarker {
    fn axis_count_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }
    fn region_count_byte_range(&self) -> Range<usize> {
        let start = self.axis_count_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn variation_regions_byte_range(&self) -> Range<usize> {
        let start = self.region_count_byte_range().end;
        start..start + self.variation_regions_byte_len
    }
}

impl<'a> FontRead<'a> for VariationRegionList<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        let axis_count: u16 = cursor.read()?;
        let region_count: u16 = cursor.read()?;
        let variation_regions_byte_len =
            region_count as usize * <VariationRegion as ComputeSize>::compute_size(&axis_count);
        cursor.advance_by(variation_regions_byte_len);
        cursor.finish(VariationRegionListMarker {
            variation_regions_byte_len,
        })
    }
}

/// The [VariationRegionList](https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#variation-regions) table
pub type VariationRegionList<'a> = TableRef<'a, VariationRegionListMarker>;

impl<'a> VariationRegionList<'a> {
    /// The number of variation axes for this font. This must be the
    /// same number as axisCount in the 'fvar' table.
    pub fn axis_count(&self) -> u16 {
        let range = self.shape.axis_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The number of variation region tables in the variation region
    /// list. Must be less than 32,768.
    pub fn region_count(&self) -> u16 {
        let range = self.shape.region_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Array of variation regions.
    pub fn variation_regions(&self) -> ComputedArray<'a, VariationRegion<'a>> {
        let range = self.shape.variation_regions_byte_range();
        self.data.read_with_args(range, &self.axis_count()).unwrap()
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for VariationRegionList<'a> {
    fn type_name(&self) -> &str {
        "VariationRegionList"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("axis_count", self.axis_count())),
            1usize => Some(Field::new("region_count", self.region_count())),
            2usize => Some(Field::new(
                "variation_regions",
                traversal::FieldType::computed_array(
                    "VariationRegion",
                    self.variation_regions(),
                    self.offset_data(),
                ),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for VariationRegionList<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// The [VariationRegion](https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#variation-regions) record
#[derive(Clone, Debug)]
pub struct VariationRegion<'a> {
    /// Array of region axis coordinates records, in the order of axes
    /// given in the 'fvar' table.
    pub region_axes: &'a [RegionAxisCoordinates],
}

impl<'a> VariationRegion<'a> {
    /// Array of region axis coordinates records, in the order of axes
    /// given in the 'fvar' table.
    pub fn region_axes(&self) -> &'a [RegionAxisCoordinates] {
        self.region_axes
    }
}

impl ReadArgs for VariationRegion<'_> {
    type Args = u16;
}

impl ComputeSize for VariationRegion<'_> {
    fn compute_size(args: &u16) -> usize {
        let axis_count = *args;
        axis_count as usize * RegionAxisCoordinates::RAW_BYTE_LEN
    }
}

impl<'a> FontReadWithArgs<'a> for VariationRegion<'a> {
    fn read_with_args(data: FontData<'a>, args: &u16) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        let axis_count = *args;
        Ok(Self {
            region_axes: cursor.read_array(axis_count as usize)?,
        })
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeRecord<'a> for VariationRegion<'a> {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "VariationRegion",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new(
                    "region_axes",
                    traversal::FieldType::array_of_records(
                        stringify!(RegionAxisCoordinates),
                        self.region_axes(),
                        _data,
                    ),
                )),
                _ => None,
            }),
            data,
        }
    }
}

/// The [RegionAxisCoordinates](https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#variation-regions) record
#[derive(Clone, Debug)]
#[repr(C)]
#[repr(packed)]
pub struct RegionAxisCoordinates {
    /// The region start coordinate value for the current axis.
    pub start_coord: BigEndian<F2Dot14>,
    /// The region peak coordinate value for the current axis.
    pub peak_coord: BigEndian<F2Dot14>,
    /// The region end coordinate value for the current axis.
    pub end_coord: BigEndian<F2Dot14>,
}

impl RegionAxisCoordinates {
    /// The region start coordinate value for the current axis.
    pub fn start_coord(&self) -> F2Dot14 {
        self.start_coord.get()
    }

    /// The region peak coordinate value for the current axis.
    pub fn peak_coord(&self) -> F2Dot14 {
        self.peak_coord.get()
    }

    /// The region end coordinate value for the current axis.
    pub fn end_coord(&self) -> F2Dot14 {
        self.end_coord.get()
    }
}

impl FixedSize for RegionAxisCoordinates {
    const RAW_BYTE_LEN: usize =
        F2Dot14::RAW_BYTE_LEN + F2Dot14::RAW_BYTE_LEN + F2Dot14::RAW_BYTE_LEN;
}

#[cfg(feature = "traversal")]
impl<'a> SomeRecord<'a> for RegionAxisCoordinates {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "RegionAxisCoordinates",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("start_coord", self.start_coord())),
                1usize => Some(Field::new("peak_coord", self.peak_coord())),
                2usize => Some(Field::new("end_coord", self.end_coord())),
                _ => None,
            }),
            data,
        }
    }
}

/// The [ItemVariationStore](https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#item-variation-store-header-and-item-variation-data-subtables) table
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct ItemVariationStoreMarker {
    item_variation_data_offsets_byte_len: usize,
}

impl ItemVariationStoreMarker {
    fn format_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }
    fn variation_region_list_offset_byte_range(&self) -> Range<usize> {
        let start = self.format_byte_range().end;
        start..start + Offset32::RAW_BYTE_LEN
    }
    fn item_variation_data_count_byte_range(&self) -> Range<usize> {
        let start = self.variation_region_list_offset_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn item_variation_data_offsets_byte_range(&self) -> Range<usize> {
        let start = self.item_variation_data_count_byte_range().end;
        start..start + self.item_variation_data_offsets_byte_len
    }
}

impl<'a> FontRead<'a> for ItemVariationStore<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u16>();
        cursor.advance::<Offset32>();
        let item_variation_data_count: u16 = cursor.read()?;
        let item_variation_data_offsets_byte_len =
            item_variation_data_count as usize * Offset32::RAW_BYTE_LEN;
        cursor.advance_by(item_variation_data_offsets_byte_len);
        cursor.finish(ItemVariationStoreMarker {
            item_variation_data_offsets_byte_len,
        })
    }
}

/// The [ItemVariationStore](https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#item-variation-store-header-and-item-variation-data-subtables) table
pub type ItemVariationStore<'a> = TableRef<'a, ItemVariationStoreMarker>;

impl<'a> ItemVariationStore<'a> {
    /// Format— set to 1
    pub fn format(&self) -> u16 {
        let range = self.shape.format_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Offset in bytes from the start of the item variation store to
    /// the variation region list.
    pub fn variation_region_list_offset(&self) -> Offset32 {
        let range = self.shape.variation_region_list_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`variation_region_list_offset`][Self::variation_region_list_offset].
    pub fn variation_region_list(&self) -> Result<VariationRegionList<'a>, ReadError> {
        let data = self.data;
        self.variation_region_list_offset().resolve(data)
    }

    /// The number of item variation data subtables.
    pub fn item_variation_data_count(&self) -> u16 {
        let range = self.shape.item_variation_data_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Offsets in bytes from the start of the item variation store to
    /// each item variation data subtable.
    pub fn item_variation_data_offsets(&self) -> &'a [BigEndian<Nullable<Offset32>>] {
        let range = self.shape.item_variation_data_offsets_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// Attempt to resolve [`item_variation_data_offsets`][Self::item_variation_data_offsets].
    pub fn item_variation_datas(
        &self,
    ) -> impl Iterator<Item = Option<Result<ItemVariationData<'a>, ReadError>>> + 'a {
        let data = self.data;
        self.item_variation_data_offsets()
            .iter()
            .map(move |off| off.get().resolve(data))
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for ItemVariationStore<'a> {
    fn type_name(&self) -> &str {
        "ItemVariationStore"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("format", self.format())),
            1usize => Some(Field::new(
                "variation_region_list_offset",
                FieldType::offset(
                    self.variation_region_list_offset(),
                    self.variation_region_list(),
                ),
            )),
            2usize => Some(Field::new(
                "item_variation_data_count",
                self.item_variation_data_count(),
            )),
            3usize => Some({
                let data = self.data;
                Field::new(
                    "item_variation_data_offsets",
                    FieldType::array_of_offsets(
                        better_type_name::<ItemVariationData>(),
                        self.item_variation_data_offsets(),
                        move |off| {
                            let target = off.get().resolve::<ItemVariationData>(data);
                            FieldType::offset(off.get(), target)
                        },
                    ),
                )
            }),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for ItemVariationStore<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// The [ItemVariationData](https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#item-variation-store-header-and-item-variation-data-subtables) subtable
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct ItemVariationDataMarker {
    region_indexes_byte_len: usize,
    delta_sets_byte_len: usize,
}

impl ItemVariationDataMarker {
    fn item_count_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }
    fn word_delta_count_byte_range(&self) -> Range<usize> {
        let start = self.item_count_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn region_index_count_byte_range(&self) -> Range<usize> {
        let start = self.word_delta_count_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn region_indexes_byte_range(&self) -> Range<usize> {
        let start = self.region_index_count_byte_range().end;
        start..start + self.region_indexes_byte_len
    }
    fn delta_sets_byte_range(&self) -> Range<usize> {
        let start = self.region_indexes_byte_range().end;
        start..start + self.delta_sets_byte_len
    }
}

impl<'a> FontRead<'a> for ItemVariationData<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u16>();
        cursor.advance::<u16>();
        let region_index_count: u16 = cursor.read()?;
        let region_indexes_byte_len = region_index_count as usize * u16::RAW_BYTE_LEN;
        cursor.advance_by(region_indexes_byte_len);
        let delta_sets_byte_len = cursor.remaining_bytes();
        cursor.advance_by(delta_sets_byte_len);
        cursor.finish(ItemVariationDataMarker {
            region_indexes_byte_len,
            delta_sets_byte_len,
        })
    }
}

/// The [ItemVariationData](https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#item-variation-store-header-and-item-variation-data-subtables) subtable
pub type ItemVariationData<'a> = TableRef<'a, ItemVariationDataMarker>;

impl<'a> ItemVariationData<'a> {
    /// The number of delta sets for distinct items.
    pub fn item_count(&self) -> u16 {
        let range = self.shape.item_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// A packed field: the high bit is a flag—see details below.
    pub fn word_delta_count(&self) -> u16 {
        let range = self.shape.word_delta_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The number of variation regions referenced.
    pub fn region_index_count(&self) -> u16 {
        let range = self.shape.region_index_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Array of indices into the variation region list for the regions
    /// referenced by this item variation data table.
    pub fn region_indexes(&self) -> &'a [BigEndian<u16>] {
        let range = self.shape.region_indexes_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// Delta-set rows.
    pub fn delta_sets(&self) -> &'a [u8] {
        let range = self.shape.delta_sets_byte_range();
        self.data.read_array(range).unwrap()
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for ItemVariationData<'a> {
    fn type_name(&self) -> &str {
        "ItemVariationData"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("item_count", self.item_count())),
            1usize => Some(Field::new("word_delta_count", self.word_delta_count())),
            2usize => Some(Field::new("region_index_count", self.region_index_count())),
            3usize => Some(Field::new("region_indexes", self.region_indexes())),
            4usize => Some(Field::new("delta_sets", self.delta_sets())),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for ItemVariationData<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}
