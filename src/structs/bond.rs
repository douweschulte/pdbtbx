/// Bond types between two atoms
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Bond {
    /// A covalent bond
    Covalent,
    /// A disulfide bond S-S
    Disulfide,
    /// A hydrogen bond H-H
    Hydrogen,
    /// ?
    MetalCoordination,
    /// ?
    MisMatchedBasePairs,
    /// ?
    SaltBridge,
    /// ?
    CovalentModificationResidue,
    /// ?
    CovalentModificationNucleotideBase,
    /// ?
    CovalentModificationNucleotideSugar,
    /// ?
    CovalentModificationNucleotidePhosphate,
}
