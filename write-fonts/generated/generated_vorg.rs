// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The [VORG (Vertical Origin)](https://docs.microsoft.com/en-us/typography/opentype/spec/vorg) table.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vorg {
    /// The y coordinate of a glyph’s vertical origin, in the font’s design
    /// coordinate system, to be used if no entry is present for the glyph
    /// in the vertOriginYMetrics array.
    pub default_vert_origin_y: i16,
    /// Array of VertOriginYMetrics records, sorted by glyph ID.
    pub vert_origin_y_metrics: Vec<VertOriginYMetrics>,
}

impl Vorg {
    /// Construct a new `Vorg`
    pub fn new(default_vert_origin_y: i16, vert_origin_y_metrics: Vec<VertOriginYMetrics>) -> Self {
        Self {
            default_vert_origin_y,
            vert_origin_y_metrics: vert_origin_y_metrics.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for Vorg {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (MajorMinor::VERSION_1_0 as MajorMinor).write_into(writer);
        self.default_vert_origin_y.write_into(writer);
        (u16::try_from(array_len(&self.vert_origin_y_metrics)).unwrap()).write_into(writer);
        self.vert_origin_y_metrics.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::TopLevel(Vorg::TAG)
    }
}

impl Validate for Vorg {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Vorg", |ctx| {
            ctx.in_field("vert_origin_y_metrics", |ctx| {
                if self.vert_origin_y_metrics.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.vert_origin_y_metrics.validate_impl(ctx);
            });
        })
    }
}

impl TopLevelTable for Vorg {
    const TAG: Tag = Tag::new(b"VORG");
}

impl<'a> FromObjRef<read_fonts::tables::vorg::Vorg<'a>> for Vorg {
    fn from_obj_ref(obj: &read_fonts::tables::vorg::Vorg<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        Vorg {
            default_vert_origin_y: obj.default_vert_origin_y(),
            vert_origin_y_metrics: obj.vert_origin_y_metrics().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::vorg::Vorg<'a>> for Vorg {}

impl<'a> FontRead<'a> for Vorg {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::vorg::Vorg as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// Vertical origin Y metrics record.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VertOriginYMetrics {
    /// Glyph index.
    pub glyph_index: GlyphId16,
    /// Y coordinate, in the font’s design coordinate system, of the glyph’s vertical origin.
    pub vert_origin_y: i16,
}

impl VertOriginYMetrics {
    /// Construct a new `VertOriginYMetrics`
    pub fn new(glyph_index: GlyphId16, vert_origin_y: i16) -> Self {
        Self {
            glyph_index,
            vert_origin_y,
        }
    }
}

impl FontWrite for VertOriginYMetrics {
    fn write_into(&self, writer: &mut TableWriter) {
        self.glyph_index.write_into(writer);
        self.vert_origin_y.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("VertOriginYMetrics")
    }
}

impl Validate for VertOriginYMetrics {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::tables::vorg::VertOriginYMetrics> for VertOriginYMetrics {
    fn from_obj_ref(obj: &read_fonts::tables::vorg::VertOriginYMetrics, _: FontData) -> Self {
        VertOriginYMetrics {
            glyph_index: obj.glyph_index(),
            vert_origin_y: obj.vert_origin_y(),
        }
    }
}
