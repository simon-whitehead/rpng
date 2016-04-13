pub enum PngChunkType {
    // Critical
    ImageHeader,
    Palette,
    ImageData,
    End,

    // Ancillary
    Chromaticity,
    Gamma,
    ICCProfile,
    SignificantBits,
    RGBColorSpace,
    BackgroundColor,
    Histogram,
    Transparency,
    PhysicalPixelDimensions,
    SuggestedPalette,
    LastModifiedTime,
    InternationalTextualData,
    TextualData,
    CompressedTextualData
}
