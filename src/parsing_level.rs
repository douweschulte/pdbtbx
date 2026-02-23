/// Represents the current parsing level. Enables control of what records in the input PDB file are processed.
/// # Examples
///
/// ```no_run
/// use pdbtbx::*;
///
/// let pdb = ReadOptions::new()
///     .set_format(Format::Auto)
///     .set_level(StrictnessLevel::Loose)
///     .set_discard_hydrogens(true)
///     .set_parsing_level(ParsingLevel::default().set_hetatm(false).set_header(false))
///     .read("1CRN.pdb");
/// ```
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct ParsingLevel {
    pub(crate) hetatm: bool,
    pub(crate) atom: bool,
    pub(crate) header: bool,
    pub(crate) remark: bool,
    pub(crate) anisou: bool,
    pub(crate) cryst1: bool,
    pub(crate) scale1: bool,
    pub(crate) scale2: bool,
    pub(crate) scale3: bool,
    pub(crate) origx1: bool,
    pub(crate) origx2: bool,
    pub(crate) origx3: bool,
    pub(crate) mtrix1: bool,
    pub(crate) mtrix2: bool,
    pub(crate) mtrix3: bool,
    pub(crate) model: bool,
    pub(crate) master: bool,
    pub(crate) dbref: bool,
    pub(crate) dbref1: bool,
    pub(crate) dbref2: bool,
    pub(crate) seqres: bool,
    pub(crate) seqadv: bool,
    pub(crate) modres: bool,
    pub(crate) ssbond: bool,
}

impl Default for ParsingLevel {
    fn default() -> Self {
        Self {
            hetatm: true,
            atom: true,
            header: true,
            remark: true,
            anisou: true,
            cryst1: true,
            scale1: true,
            scale2: true,
            scale3: true,
            origx1: true,
            origx2: true,
            origx3: true,
            mtrix1: true,
            mtrix2: true,
            mtrix3: true,
            model: true,
            master: true,
            dbref: true,
            dbref1: true,
            dbref2: true,
            seqres: true,
            seqadv: true,
            modres: true,
            ssbond: true,
        }
    }
}

impl ParsingLevel {
    /// Returns a new ParsingLevel with all options set to true.
    pub fn all() -> Self {
        Self::default()
    }

    /// Returns a new ParsingLevel with all options set to false.
    pub fn none() -> Self {
        Self {
            hetatm: false,
            atom: false,
            header: false,
            remark: false,
            anisou: false,
            cryst1: false,
            scale1: false,
            scale2: false,
            scale3: false,
            origx1: false,
            origx2: false,
            origx3: false,
            mtrix1: false,
            mtrix2: false,
            mtrix3: false,
            model: false,
            master: false,
            dbref: false,
            dbref1: false,
            dbref2: false,
            seqres: false,
            seqadv: false,
            modres: false,
            ssbond: false,
        }
    }
}

macro_rules! parsing_level_setters {
    ($($field:ident, $setter:ident);+ $(;)?) => {
        impl ParsingLevel {
            $(
                #[doc = concat!("Set whether to parse ", stringify!($field), " records.")]
                pub fn $setter(&mut self, value: bool) -> &mut Self {
                    self.$field = value;
                    self
                }
            )+
        }
    };
}

parsing_level_setters!(
    hetatm, set_hetatm;
    atom, set_atom;
    header, set_header;
    remark, set_remark;
    anisou, set_anisou;
    cryst1, set_cryst1;
    scale1, set_scale1;
    scale2, set_scale2;
    scale3, set_scale3;
    origx1, set_origx1;
    origx2, set_origx2;
    origx3, set_origx3;
    mtrix1, set_mtrix1;
    mtrix2, set_mtrix2;
    mtrix3, set_mtrix3;
    model, set_model;
    master, set_master;
    dbref, set_dbref;
    dbref1, set_dbref1;
    dbref2, set_dbref2;
    seqres, set_seqres;
    seqadv, set_seqadv;
    modres, set_modres;
    ssbond, set_ssbond;
);
