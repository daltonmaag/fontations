// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The [fvar (Font Variations)](https://docs.microsoft.com/en-us/typography/opentype/spec/fvar) table
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fvar {
    /// Offset in bytes from the beginning of the table to the start of the VariationAxisRecord array. The
    /// InstanceRecord array directly follows.
    pub axis_instance_arrays: OffsetMarker<AxisInstanceArrays>,
}

impl Fvar {
    /// Construct a new `Fvar`
    pub fn new(axis_instance_arrays: AxisInstanceArrays) -> Self {
        Self {
            axis_instance_arrays: axis_instance_arrays.into(),
        }
    }
}

impl FontWrite for Fvar {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (MajorMinor::VERSION_1_0 as MajorMinor).write_into(writer);
        self.axis_instance_arrays.write_into(writer);
        (2 as u16).write_into(writer);
        (self.axis_count() as u16).write_into(writer);
        (20 as u16).write_into(writer);
        (self.instance_count() as u16).write_into(writer);
        (self.instance_size() as u16).write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::TopLevel(Fvar::TAG)
    }
}

impl Validate for Fvar {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Fvar", |ctx| {
            ctx.in_field("axis_instance_arrays", |ctx| {
                self.axis_instance_arrays.validate_impl(ctx);
            });
            self.check_instances(ctx);
        })
    }
}

impl TopLevelTable for Fvar {
    const TAG: Tag = Tag::new(b"fvar");
}

impl<'a> FromObjRef<read_fonts::tables::fvar::Fvar<'a>> for Fvar {
    fn from_obj_ref(obj: &read_fonts::tables::fvar::Fvar<'a>, _: FontData) -> Self {
        Fvar {
            axis_instance_arrays: obj.axis_instance_arrays().to_owned_table(),
        }
    }
}

#[allow(clippy::needless_lifetimes)]
impl<'a> FromTableRef<read_fonts::tables::fvar::Fvar<'a>> for Fvar {}

impl<'a> FontRead<'a> for Fvar {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::fvar::Fvar as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// Shim table to handle combined axis and instance arrays.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AxisInstanceArrays {
    /// Variation axis record array.
    pub axes: Vec<VariationAxisRecord>,
    /// Instance record array.
    pub instances: Vec<InstanceRecord>,
}

impl AxisInstanceArrays {
    /// Construct a new `AxisInstanceArrays`
    pub fn new(axes: Vec<VariationAxisRecord>, instances: Vec<InstanceRecord>) -> Self {
        Self { axes, instances }
    }
}

impl FontWrite for AxisInstanceArrays {
    fn write_into(&self, writer: &mut TableWriter) {
        self.axes.write_into(writer);
        self.instances.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("AxisInstanceArrays")
    }
}

impl Validate for AxisInstanceArrays {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("AxisInstanceArrays", |ctx| {
            ctx.in_field("axes", |ctx| {
                if self.axes.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.axes.validate_impl(ctx);
            });
            ctx.in_field("instances", |ctx| {
                if self.instances.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.instances.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::fvar::AxisInstanceArrays<'a>> for AxisInstanceArrays {
    fn from_obj_ref(obj: &read_fonts::tables::fvar::AxisInstanceArrays<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        AxisInstanceArrays {
            axes: obj.axes().to_owned_obj(offset_data),
            instances: obj
                .instances()
                .iter()
                .filter_map(|x| x.map(|x| FromObjRef::from_obj_ref(&x, offset_data)).ok())
                .collect(),
        }
    }
}

#[allow(clippy::needless_lifetimes)]
impl<'a> FromTableRef<read_fonts::tables::fvar::AxisInstanceArrays<'a>> for AxisInstanceArrays {}

/// The [VariationAxisRecord](https://learn.microsoft.com/en-us/typography/opentype/spec/fvar#variationaxisrecord)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VariationAxisRecord {
    /// Tag identifying the design variation for the axis.
    pub axis_tag: Tag,
    /// The minimum coordinate value for the axis.
    pub min_value: Fixed,
    /// The default coordinate value for the axis.
    pub default_value: Fixed,
    /// The maximum coordinate value for the axis.
    pub max_value: Fixed,
    /// Axis qualifiers — see details below.
    pub flags: u16,
    /// The name ID for entries in the 'name' table that provide a display name for this axis.
    pub axis_name_id: NameId,
}

impl VariationAxisRecord {
    /// Construct a new `VariationAxisRecord`
    pub fn new(
        axis_tag: Tag,
        min_value: Fixed,
        default_value: Fixed,
        max_value: Fixed,
        flags: u16,
        axis_name_id: NameId,
    ) -> Self {
        Self {
            axis_tag,
            min_value,
            default_value,
            max_value,
            flags,
            axis_name_id,
        }
    }
}

impl FontWrite for VariationAxisRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.axis_tag.write_into(writer);
        self.min_value.write_into(writer);
        self.default_value.write_into(writer);
        self.max_value.write_into(writer);
        self.flags.write_into(writer);
        self.axis_name_id.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("VariationAxisRecord")
    }
}

impl Validate for VariationAxisRecord {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::tables::fvar::VariationAxisRecord> for VariationAxisRecord {
    fn from_obj_ref(obj: &read_fonts::tables::fvar::VariationAxisRecord, _: FontData) -> Self {
        VariationAxisRecord {
            axis_tag: obj.axis_tag(),
            min_value: obj.min_value(),
            default_value: obj.default_value(),
            max_value: obj.max_value(),
            flags: obj.flags(),
            axis_name_id: obj.axis_name_id(),
        }
    }
}
