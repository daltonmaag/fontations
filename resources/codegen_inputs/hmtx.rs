#![parse_module(read_fonts::tables::hmtx)]

/// The [hmtx (Horizontal Metrics)](https://docs.microsoft.com/en-us/typography/opentype/spec/hmtx) table
#[read_args(number_of_h_metrics: u16)]
#[tag = "hmtx"]
table Hmtx {
    /// Paired advance width/height and left/top side bearing values for each
    /// glyph. Records are indexed by glyph ID.
    #[count($number_of_h_metrics)]
    h_metrics: [LongMetric],
    /// Leading (left/top) side bearings for glyph IDs greater than or equal to
    /// numberOfLongMetrics.
    // TODO: Instead of always taking the tail, take the minimum of tail and (num_glyphs - number_of_h_metrics).
    #[count(..)]
    left_side_bearings: [i16],
}

record LongMetric {
    /// Advance width/height, in font design units.
    advance: u16,
    /// Glyph leading (left/top) side bearing, in font design units.
    side_bearing: i16,
}

