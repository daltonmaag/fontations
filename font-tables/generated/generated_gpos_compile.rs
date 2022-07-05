// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

use super::ValueFormat;

#[allow(unused_imports)]
use crate::compile::*;
use crate::layout::compile::ClassDef;
use crate::layout::compile::CoverageTable;
use crate::layout::compile::Device;
use crate::layout::compile::FeatureList;
use crate::layout::compile::FeatureVariations;
use crate::layout::compile::ScriptList;

#[allow(unused_imports)]
use font_types::*;

#[derive(Debug, PartialEq)]
pub struct Gpos1_0 {
    pub major_version: u16,
    pub minor_version: u16,
    pub script_list_offset: OffsetMarker<Offset16, ScriptList>,
    pub feature_list_offset: OffsetMarker<Offset16, FeatureList>,
    pub lookup_list_offset: OffsetMarker<Offset16, PositionLookupList>,
}

impl ToOwnedObj for super::Gpos1_0<'_> {
    type Owned = Gpos1_0;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        let offset_data = self.bytes();
        Some(Gpos1_0 {
            major_version: self.major_version(),
            minor_version: self.minor_version(),
            script_list_offset: OffsetMarker::new_maybe_null(
                self.script_list_offset()
                    .read::<super::ScriptList>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            feature_list_offset: OffsetMarker::new_maybe_null(
                self.feature_list_offset()
                    .read::<super::FeatureList>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            lookup_list_offset: OffsetMarker::new_maybe_null(
                self.lookup_list_offset()
                    .read::<super::PositionLookupList>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
        })
    }
}

impl ToOwnedTable for super::Gpos1_0<'_> {}

impl FontWrite for Gpos1_0 {
    fn write_into(&self, writer: &mut TableWriter) {
        self.major_version.write_into(writer);
        self.minor_version.write_into(writer);
        self.script_list_offset.write_into(writer);
        self.feature_list_offset.write_into(writer);
        self.lookup_list_offset.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct Gpos1_1 {
    pub major_version: u16,
    pub minor_version: u16,
    pub script_list_offset: OffsetMarker<Offset16, ScriptList>,
    pub feature_list_offset: OffsetMarker<Offset16, FeatureList>,
    pub lookup_list_offset: OffsetMarker<Offset16, PositionLookupList>,
    pub feature_variations_offset: NullableOffsetMarker<Offset32, FeatureVariations>,
}

impl ToOwnedObj for super::Gpos1_1<'_> {
    type Owned = Gpos1_1;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        let offset_data = self.bytes();
        Some(Gpos1_1 {
            major_version: self.major_version(),
            minor_version: self.minor_version(),
            script_list_offset: OffsetMarker::new_maybe_null(
                self.script_list_offset()
                    .read::<super::ScriptList>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            feature_list_offset: OffsetMarker::new_maybe_null(
                self.feature_list_offset()
                    .read::<super::FeatureList>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            lookup_list_offset: OffsetMarker::new_maybe_null(
                self.lookup_list_offset()
                    .read::<super::PositionLookupList>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            feature_variations_offset: NullableOffsetMarker::new(
                self.feature_variations_offset()
                    .read::<super::FeatureVariations>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
        })
    }
}

impl ToOwnedTable for super::Gpos1_1<'_> {}

impl FontWrite for Gpos1_1 {
    fn write_into(&self, writer: &mut TableWriter) {
        self.major_version.write_into(writer);
        self.minor_version.write_into(writer);
        self.script_list_offset.write_into(writer);
        self.feature_list_offset.write_into(writer);
        self.lookup_list_offset.write_into(writer);
        self.feature_variations_offset.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub enum Gpos {
    Version1_0(Gpos1_0),
    Version1_1(Gpos1_1),
}

impl ToOwnedObj for super::Gpos<'_> {
    type Owned = Gpos;
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        Some(match self {
            super::Gpos::Version1_0(item) => Gpos::Version1_0(item.to_owned_obj(offset_data)?),
            super::Gpos::Version1_1(item) => Gpos::Version1_1(item.to_owned_obj(offset_data)?),
        })
    }
}

impl ToOwnedTable for super::Gpos<'_> {}

impl FontWrite for Gpos {
    fn write_into(&self, writer: &mut TableWriter) {
        match self {
            Self::Version1_0(item) => item.write_into(writer),
            Self::Version1_1(item) => item.write_into(writer),
        }
    }
}

impl FontWrite for ValueFormat {
    fn write_into(&self, writer: &mut TableWriter) {
        self.bits().write_into(writer)
    }
}

#[derive(Debug, PartialEq)]
pub enum AnchorTable {
    Format1(AnchorFormat1),
    Format2(AnchorFormat2),
    Format3(AnchorFormat3),
}

impl ToOwnedObj for super::AnchorTable<'_> {
    type Owned = AnchorTable;
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        Some(match self {
            super::AnchorTable::Format1(item) => {
                AnchorTable::Format1(item.to_owned_obj(offset_data)?)
            }
            super::AnchorTable::Format2(item) => {
                AnchorTable::Format2(item.to_owned_obj(offset_data)?)
            }
            super::AnchorTable::Format3(item) => {
                AnchorTable::Format3(item.to_owned_obj(offset_data)?)
            }
        })
    }
}

impl FontWrite for AnchorTable {
    fn write_into(&self, writer: &mut TableWriter) {
        match self {
            Self::Format1(item) => item.write_into(writer),
            Self::Format2(item) => item.write_into(writer),
            Self::Format3(item) => item.write_into(writer),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AnchorFormat1 {
    pub x_coordinate: i16,
    pub y_coordinate: i16,
}

impl ToOwnedObj for super::AnchorFormat1 {
    type Owned = AnchorFormat1;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        Some(AnchorFormat1 {
            x_coordinate: self.x_coordinate(),
            y_coordinate: self.y_coordinate(),
        })
    }
}

impl FontWrite for AnchorFormat1 {
    fn write_into(&self, writer: &mut TableWriter) {
        let anchor_format: u16 = 1;
        anchor_format.write_into(writer);
        self.x_coordinate.write_into(writer);
        self.y_coordinate.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct AnchorFormat2 {
    pub x_coordinate: i16,
    pub y_coordinate: i16,
    pub anchor_point: u16,
}

impl ToOwnedObj for super::AnchorFormat2 {
    type Owned = AnchorFormat2;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        Some(AnchorFormat2 {
            x_coordinate: self.x_coordinate(),
            y_coordinate: self.y_coordinate(),
            anchor_point: self.anchor_point(),
        })
    }
}

impl FontWrite for AnchorFormat2 {
    fn write_into(&self, writer: &mut TableWriter) {
        let anchor_format: u16 = 2;
        anchor_format.write_into(writer);
        self.x_coordinate.write_into(writer);
        self.y_coordinate.write_into(writer);
        self.anchor_point.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct AnchorFormat3 {
    pub x_coordinate: i16,
    pub y_coordinate: i16,
    pub x_device_offset: NullableOffsetMarker<Offset16, Device>,
    pub y_device_offset: NullableOffsetMarker<Offset16, Device>,
}

impl ToOwnedObj for super::AnchorFormat3<'_> {
    type Owned = AnchorFormat3;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        let offset_data = self.bytes();
        Some(AnchorFormat3 {
            x_coordinate: self.x_coordinate(),
            y_coordinate: self.y_coordinate(),
            x_device_offset: NullableOffsetMarker::new(
                self.x_device_offset()
                    .read::<super::Device>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            y_device_offset: NullableOffsetMarker::new(
                self.y_device_offset()
                    .read::<super::Device>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
        })
    }
}

impl ToOwnedTable for super::AnchorFormat3<'_> {}

impl FontWrite for AnchorFormat3 {
    fn write_into(&self, writer: &mut TableWriter) {
        let anchor_format: u16 = 3;
        anchor_format.write_into(writer);
        self.x_coordinate.write_into(writer);
        self.y_coordinate.write_into(writer);
        self.x_device_offset.write_into(writer);
        self.y_device_offset.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct MarkArray {
    pub mark_records: Vec<MarkRecord>,
}

impl ToOwnedObj for super::MarkArray<'_> {
    type Owned = MarkArray;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        let offset_data = self.bytes();
        Some(MarkArray {
            mark_records: self
                .mark_records()
                .iter()
                .map(|item| item.to_owned_obj(offset_data))
                .collect::<Option<Vec<_>>>()?,
        })
    }
}

impl ToOwnedTable for super::MarkArray<'_> {}

impl FontWrite for MarkArray {
    fn write_into(&self, writer: &mut TableWriter) {
        u16::try_from(self.mark_records.len())
            .unwrap()
            .write_into(writer);
        self.mark_records.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct MarkRecord {
    pub mark_class: u16,
    pub mark_anchor_offset: OffsetMarker<Offset16, AnchorTable>,
}

impl ToOwnedObj for super::MarkRecord {
    type Owned = MarkRecord;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        Some(MarkRecord {
            mark_class: self.mark_class(),
            mark_anchor_offset: OffsetMarker::new_maybe_null(
                self.mark_anchor_offset()
                    .read::<super::AnchorTable>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
        })
    }
}

impl FontWrite for MarkRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.mark_class.write_into(writer);
        self.mark_anchor_offset.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub enum SinglePos {
    Format1(SinglePosFormat1),
    Format2(SinglePosFormat2),
}

impl ToOwnedObj for super::SinglePos<'_> {
    type Owned = SinglePos;
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        Some(match self {
            super::SinglePos::Format1(item) => SinglePos::Format1(item.to_owned_obj(offset_data)?),
            super::SinglePos::Format2(item) => SinglePos::Format2(item.to_owned_obj(offset_data)?),
        })
    }
}

impl ToOwnedTable for super::SinglePos<'_> {}

impl FontWrite for SinglePos {
    fn write_into(&self, writer: &mut TableWriter) {
        match self {
            Self::Format1(item) => item.write_into(writer),
            Self::Format2(item) => item.write_into(writer),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SinglePosFormat1 {
    pub coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    pub value_format: ValueFormat,
    pub value_record: ValueRecord,
}

impl ToOwnedObj for super::SinglePosFormat1<'_> {
    type Owned = SinglePosFormat1;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        let offset_data = self.bytes();
        Some(SinglePosFormat1 {
            coverage_offset: OffsetMarker::new_maybe_null(
                self.coverage_offset()
                    .read::<super::CoverageTable>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            value_format: self.value_format(),
            value_record: self.value_record.to_owned_obj(offset_data)?,
        })
    }
}

impl ToOwnedTable for super::SinglePosFormat1<'_> {}

impl FontWrite for SinglePosFormat1 {
    fn write_into(&self, writer: &mut TableWriter) {
        let pos_format: u16 = 1;
        pos_format.write_into(writer);
        self.coverage_offset.write_into(writer);
        self.value_format.write_into(writer);
        self.value_record.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct SinglePosFormat2 {
    pub coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    pub value_format: ValueFormat,
    pub value_records: Vec<ValueRecord>,
}

impl ToOwnedObj for super::SinglePosFormat2<'_> {
    type Owned = SinglePosFormat2;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        let offset_data = self.bytes();
        Some(SinglePosFormat2 {
            coverage_offset: OffsetMarker::new_maybe_null(
                self.coverage_offset()
                    .read::<super::CoverageTable>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            value_format: self.value_format(),
            value_records: self.value_records.to_owned_obj(offset_data)?,
        })
    }
}

impl ToOwnedTable for super::SinglePosFormat2<'_> {}

impl FontWrite for SinglePosFormat2 {
    fn write_into(&self, writer: &mut TableWriter) {
        let pos_format: u16 = 2;
        pos_format.write_into(writer);
        self.coverage_offset.write_into(writer);
        self.value_format.write_into(writer);
        u16::try_from(self.value_records.len())
            .unwrap()
            .write_into(writer);
        self.value_records.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub enum PairPos {
    Format1(PairPosFormat1),
    Format2(PairPosFormat2),
}

impl ToOwnedObj for super::PairPos<'_> {
    type Owned = PairPos;
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        Some(match self {
            super::PairPos::Format1(item) => PairPos::Format1(item.to_owned_obj(offset_data)?),
            super::PairPos::Format2(item) => PairPos::Format2(item.to_owned_obj(offset_data)?),
        })
    }
}

impl ToOwnedTable for super::PairPos<'_> {}

impl FontWrite for PairPos {
    fn write_into(&self, writer: &mut TableWriter) {
        match self {
            Self::Format1(item) => item.write_into(writer),
            Self::Format2(item) => item.write_into(writer),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PairPosFormat1 {
    pub coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    pub value_format1: ValueFormat,
    pub value_format2: ValueFormat,
    pub pair_set_offsets: Vec<OffsetMarker<Offset16, PairSet>>,
}

impl ToOwnedObj for super::PairPosFormat1<'_> {
    type Owned = PairPosFormat1;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        let offset_data = self.bytes();
        Some(PairPosFormat1 {
            coverage_offset: OffsetMarker::new_maybe_null(
                self.coverage_offset()
                    .read::<super::CoverageTable>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            value_format1: self.value_format1(),
            value_format2: self.value_format2(),
            pair_set_offsets: self.pair_sets_to_owned()?,
        })
    }
}

impl ToOwnedTable for super::PairPosFormat1<'_> {}

impl FontWrite for PairPosFormat1 {
    fn write_into(&self, writer: &mut TableWriter) {
        let pos_format: u16 = 1;
        pos_format.write_into(writer);
        self.coverage_offset.write_into(writer);
        self.value_format1.write_into(writer);
        self.value_format2.write_into(writer);
        u16::try_from(self.pair_set_offsets.len())
            .unwrap()
            .write_into(writer);
        self.pair_set_offsets.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct PairSet {
    pub pair_value_records: Vec<PairValueRecord>,
}

impl ToOwnedObj for super::PairSet<'_> {
    type Owned = PairSet;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        Some(PairSet {
            pair_value_records: self.pair_value_records.to_owned_obj(offset_data)?,
        })
    }
}

impl FontWrite for PairSet {
    fn write_into(&self, writer: &mut TableWriter) {
        u16::try_from(self.pair_value_records.len())
            .unwrap()
            .write_into(writer);
        self.pair_value_records.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct PairPosFormat2 {
    pub coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    pub value_format1: ValueFormat,
    pub value_format2: ValueFormat,
    pub class_def1_offset: OffsetMarker<Offset16, ClassDef>,
    pub class_def2_offset: OffsetMarker<Offset16, ClassDef>,
    pub class1_records: Vec<Class1Record>,
}

impl ToOwnedObj for super::PairPosFormat2<'_> {
    type Owned = PairPosFormat2;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        let offset_data = self.bytes();
        Some(PairPosFormat2 {
            coverage_offset: OffsetMarker::new_maybe_null(
                self.coverage_offset()
                    .read::<super::CoverageTable>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            value_format1: self.value_format1(),
            value_format2: self.value_format2(),
            class_def1_offset: OffsetMarker::new_maybe_null(
                self.class_def1_offset()
                    .read::<super::ClassDef>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            class_def2_offset: OffsetMarker::new_maybe_null(
                self.class_def2_offset()
                    .read::<super::ClassDef>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            class1_records: self.class1_records.to_owned_obj(offset_data)?,
        })
    }
}

impl ToOwnedTable for super::PairPosFormat2<'_> {}

impl FontWrite for PairPosFormat2 {
    fn write_into(&self, writer: &mut TableWriter) {
        let pos_format: u16 = 2;
        pos_format.write_into(writer);
        self.coverage_offset.write_into(writer);
        self.value_format1.write_into(writer);
        self.value_format2.write_into(writer);
        self.class_def1_offset.write_into(writer);
        self.class_def2_offset.write_into(writer);
        let class1_count: u16 = self.class_def1_offset.get().unwrap().class_count();
        class1_count.write_into(writer);
        let class2_count: u16 = self.class_def2_offset.get().unwrap().class_count();
        class2_count.write_into(writer);
        self.class1_records.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct Class1Record {
    pub class2_records: Vec<Class2Record>,
}

impl ToOwnedObj for super::Class1Record<'_> {
    type Owned = Class1Record;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        Some(Class1Record {
            class2_records: self.class2_records.to_owned_obj(offset_data)?,
        })
    }
}

impl FontWrite for Class1Record {
    fn write_into(&self, writer: &mut TableWriter) {
        self.class2_records.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct Class2Record {
    pub value_record1: ValueRecord,
    pub value_record2: ValueRecord,
}

impl ToOwnedObj for super::Class2Record {
    type Owned = Class2Record;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        Some(Class2Record {
            value_record1: self.value_record1.to_owned_obj(offset_data)?,
            value_record2: self.value_record2.to_owned_obj(offset_data)?,
        })
    }
}

impl FontWrite for Class2Record {
    fn write_into(&self, writer: &mut TableWriter) {
        self.value_record1.write_into(writer);
        self.value_record2.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct CursivePosFormat1 {
    pub coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    pub entry_exit_record: Vec<EntryExitRecord>,
}

impl ToOwnedObj for super::CursivePosFormat1<'_> {
    type Owned = CursivePosFormat1;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        let offset_data = self.bytes();
        Some(CursivePosFormat1 {
            coverage_offset: OffsetMarker::new_maybe_null(
                self.coverage_offset()
                    .read::<super::CoverageTable>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            entry_exit_record: self
                .entry_exit_record()
                .iter()
                .map(|item| item.to_owned_obj(offset_data))
                .collect::<Option<Vec<_>>>()?,
        })
    }
}

impl ToOwnedTable for super::CursivePosFormat1<'_> {}

impl FontWrite for CursivePosFormat1 {
    fn write_into(&self, writer: &mut TableWriter) {
        let pos_format: u16 = 1;
        pos_format.write_into(writer);
        self.coverage_offset.write_into(writer);
        u16::try_from(self.entry_exit_record.len())
            .unwrap()
            .write_into(writer);
        self.entry_exit_record.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct EntryExitRecord {
    pub entry_anchor_offset: NullableOffsetMarker<Offset16, AnchorTable>,
    pub exit_anchor_offset: NullableOffsetMarker<Offset16, AnchorTable>,
}

impl ToOwnedObj for super::EntryExitRecord {
    type Owned = EntryExitRecord;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        Some(EntryExitRecord {
            entry_anchor_offset: NullableOffsetMarker::new(
                self.entry_anchor_offset()
                    .read::<super::AnchorTable>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            exit_anchor_offset: NullableOffsetMarker::new(
                self.exit_anchor_offset()
                    .read::<super::AnchorTable>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
        })
    }
}

impl FontWrite for EntryExitRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.entry_anchor_offset.write_into(writer);
        self.exit_anchor_offset.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct MarkBasePosFormat1 {
    pub mark_coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    pub base_coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    pub mark_array_offset: OffsetMarker<Offset16, MarkArray>,
    pub base_array_offset: OffsetMarker<Offset16, BaseArray>,
}

impl ToOwnedObj for super::MarkBasePosFormat1<'_> {
    type Owned = MarkBasePosFormat1;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        let offset_data = self.bytes();
        Some(MarkBasePosFormat1 {
            mark_coverage_offset: OffsetMarker::new_maybe_null(
                self.mark_coverage_offset()
                    .read::<super::CoverageTable>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            base_coverage_offset: OffsetMarker::new_maybe_null(
                self.base_coverage_offset()
                    .read::<super::CoverageTable>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            mark_array_offset: OffsetMarker::new_maybe_null(
                self.mark_array_offset()
                    .read::<super::MarkArray>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            base_array_offset: self.base_array_to_owned()?,
        })
    }
}

impl ToOwnedTable for super::MarkBasePosFormat1<'_> {}

impl FontWrite for MarkBasePosFormat1 {
    fn write_into(&self, writer: &mut TableWriter) {
        let pos_format: u16 = 1;
        pos_format.write_into(writer);
        self.mark_coverage_offset.write_into(writer);
        self.base_coverage_offset.write_into(writer);
        let mark_class_count: u16 = self.mark_array_offset.get().unwrap().class_count();
        mark_class_count.write_into(writer);
        self.mark_array_offset.write_into(writer);
        self.base_array_offset.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct BaseArray {
    pub base_records: Vec<BaseRecord>,
}

impl ToOwnedObj for super::BaseArray<'_> {
    type Owned = BaseArray;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        let offset_data = self.bytes();
        Some(BaseArray {
            base_records: self.base_records.to_owned_obj(offset_data)?,
        })
    }
}

impl ToOwnedTable for super::BaseArray<'_> {}

impl FontWrite for BaseArray {
    fn write_into(&self, writer: &mut TableWriter) {
        u16::try_from(self.base_records.len())
            .unwrap()
            .write_into(writer);
        self.base_records.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct BaseRecord {
    pub base_anchor_offsets: Vec<NullableOffsetMarker<Offset16, AnchorTable>>,
}

impl ToOwnedObj for super::BaseRecord<'_> {
    type Owned = BaseRecord;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        Some(BaseRecord {
            base_anchor_offsets: self
                .base_anchor_offsets()
                .iter()
                .map(|item| {
                    Some(NullableOffsetMarker::new(
                        item.get()
                            .read::<super::AnchorTable>(offset_data)
                            .and_then(|obj| obj.to_owned_obj(offset_data)),
                    ))
                })
                .collect::<Option<Vec<_>>>()?,
        })
    }
}

impl FontWrite for BaseRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.base_anchor_offsets.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct MarkLigPosFormat1 {
    pub mark_coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    pub ligature_coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    pub mark_array_offset: OffsetMarker<Offset16, MarkArray>,
    pub ligature_array_offset: OffsetMarker<Offset16, LigatureArray>,
}

impl ToOwnedObj for super::MarkLigPosFormat1<'_> {
    type Owned = MarkLigPosFormat1;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        let offset_data = self.bytes();
        Some(MarkLigPosFormat1 {
            mark_coverage_offset: OffsetMarker::new_maybe_null(
                self.mark_coverage_offset()
                    .read::<super::CoverageTable>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            ligature_coverage_offset: OffsetMarker::new_maybe_null(
                self.ligature_coverage_offset()
                    .read::<super::CoverageTable>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            mark_array_offset: OffsetMarker::new_maybe_null(
                self.mark_array_offset()
                    .read::<super::MarkArray>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            ligature_array_offset: self.ligature_array_to_owned()?,
        })
    }
}

impl ToOwnedTable for super::MarkLigPosFormat1<'_> {}

impl FontWrite for MarkLigPosFormat1 {
    fn write_into(&self, writer: &mut TableWriter) {
        let pos_format: u16 = 1;
        pos_format.write_into(writer);
        self.mark_coverage_offset.write_into(writer);
        self.ligature_coverage_offset.write_into(writer);
        let mark_class_count: u16 = self.mark_array_offset.get().unwrap().class_count();
        mark_class_count.write_into(writer);
        self.mark_array_offset.write_into(writer);
        self.ligature_array_offset.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct LigatureArray {
    pub ligature_attach_offsets: Vec<OffsetMarker<Offset16, LigatureAttach>>,
}

impl FontWrite for LigatureArray {
    fn write_into(&self, writer: &mut TableWriter) {
        u16::try_from(self.ligature_attach_offsets.len())
            .unwrap()
            .write_into(writer);
        self.ligature_attach_offsets.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct LigatureAttach {
    pub component_records: Vec<ComponentRecord>,
}

impl ToOwnedObj for super::LigatureAttach<'_> {
    type Owned = LigatureAttach;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        let offset_data = self.bytes();
        Some(LigatureAttach {
            component_records: self.component_records.to_owned_obj(offset_data)?,
        })
    }
}

impl ToOwnedTable for super::LigatureAttach<'_> {}

impl FontWrite for LigatureAttach {
    fn write_into(&self, writer: &mut TableWriter) {
        u16::try_from(self.component_records.len())
            .unwrap()
            .write_into(writer);
        self.component_records.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct ComponentRecord {
    pub ligature_anchor_offsets: Vec<NullableOffsetMarker<Offset16, AnchorTable>>,
}

impl ToOwnedObj for super::ComponentRecord<'_> {
    type Owned = ComponentRecord;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        Some(ComponentRecord {
            ligature_anchor_offsets: self
                .ligature_anchor_offsets()
                .iter()
                .map(|item| {
                    Some(NullableOffsetMarker::new(
                        item.get()
                            .read::<super::AnchorTable>(offset_data)
                            .and_then(|obj| obj.to_owned_obj(offset_data)),
                    ))
                })
                .collect::<Option<Vec<_>>>()?,
        })
    }
}

impl FontWrite for ComponentRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.ligature_anchor_offsets.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct MarkMarkPosFormat1 {
    pub mark1_coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    pub mark2_coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    pub mark1_array_offset: OffsetMarker<Offset16, MarkArray>,
    pub mark2_array_offset: OffsetMarker<Offset16, Mark2Array>,
}

impl ToOwnedObj for super::MarkMarkPosFormat1<'_> {
    type Owned = MarkMarkPosFormat1;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        let offset_data = self.bytes();
        Some(MarkMarkPosFormat1 {
            mark1_coverage_offset: OffsetMarker::new_maybe_null(
                self.mark1_coverage_offset()
                    .read::<super::CoverageTable>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            mark2_coverage_offset: OffsetMarker::new_maybe_null(
                self.mark2_coverage_offset()
                    .read::<super::CoverageTable>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            mark1_array_offset: OffsetMarker::new_maybe_null(
                self.mark1_array_offset()
                    .read::<super::MarkArray>(offset_data)
                    .and_then(|obj| obj.to_owned_obj(offset_data)),
            ),
            mark2_array_offset: self.mark2_array_to_owned()?,
        })
    }
}

impl ToOwnedTable for super::MarkMarkPosFormat1<'_> {}

impl FontWrite for MarkMarkPosFormat1 {
    fn write_into(&self, writer: &mut TableWriter) {
        let pos_format: u16 = 1;
        pos_format.write_into(writer);
        self.mark1_coverage_offset.write_into(writer);
        self.mark2_coverage_offset.write_into(writer);
        let mark_class_count: u16 = self.mark1_array_offset.get().unwrap().class_count();
        mark_class_count.write_into(writer);
        self.mark1_array_offset.write_into(writer);
        self.mark2_array_offset.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct Mark2Array {
    pub mark2_records: Vec<Mark2Record>,
}

impl ToOwnedObj for super::Mark2Array<'_> {
    type Owned = Mark2Array;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        let offset_data = self.bytes();
        Some(Mark2Array {
            mark2_records: self.mark2_records.to_owned_obj(offset_data)?,
        })
    }
}

impl ToOwnedTable for super::Mark2Array<'_> {}

impl FontWrite for Mark2Array {
    fn write_into(&self, writer: &mut TableWriter) {
        u16::try_from(self.mark2_records.len())
            .unwrap()
            .write_into(writer);
        self.mark2_records.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct Mark2Record {
    pub mark2_anchor_offsets: Vec<NullableOffsetMarker<Offset16, AnchorTable>>,
}

impl ToOwnedObj for super::Mark2Record<'_> {
    type Owned = Mark2Record;

    #[allow(unused_variables)]
    fn to_owned_obj(&self, offset_data: &[u8]) -> Option<Self::Owned> {
        Some(Mark2Record {
            mark2_anchor_offsets: self
                .mark2_anchor_offsets()
                .iter()
                .map(|item| {
                    Some(NullableOffsetMarker::new(
                        item.get()
                            .read::<super::AnchorTable>(offset_data)
                            .and_then(|obj| obj.to_owned_obj(offset_data)),
                    ))
                })
                .collect::<Option<Vec<_>>>()?,
        })
    }
}

impl FontWrite for Mark2Record {
    fn write_into(&self, writer: &mut TableWriter) {
        self.mark2_anchor_offsets.write_into(writer);
    }
}
