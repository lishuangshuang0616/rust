//! Martian stage DETECT_CHEMISTRY

use crate::barcode_overlap::FRPGemBarcodeOverlapRow;
use crate::detect_chemistry::chemistry_filter::{
    detect_chemistry_units, ChemistryFilter, DetectChemistryUnit,
};
use crate::detect_chemistry::errors::DetectChemistryErrors;
use crate::detect_chemistry::identity_check::check_fastq_identity;
use crate::detect_chemistry::length_filter::LengthFilter;
use crate::detect_chemistry::mapping_filter::ReadMappingFilter;
use crate::detect_chemistry::probe_bc_check::validate_no_probe_bc_mixture_in_sfrp;
use crate::detect_chemistry::probe_bc_pairing::{
    detect_probe_barcode_pairing, should_detect_probe_barcode_pairing,
};
use crate::detect_chemistry::whitelist_filter::WhitelistMatchFilter;
use anyhow::{bail, ensure, Context, Result};
use barcode::whitelist::BarcodeId;
use cr_types::chemistry::{
    normalize_chemistry_def, AutoChemistryName, AutoOrRefinedChemistry, ChemistryDef,
    ChemistryDefs, ChemistryName, ChemistrySpecs,
};
use cr_types::reference::feature_reference::{FeatureConfig, FeatureReferenceFile};
use cr_types::sample_def::SampleDef;
use cr_types::LibraryType;
use fastq_set::read_pair::ReadPair;
use itertools::Itertools;
use martian::prelude::*;
use martian_derive::{make_mro, MartianStruct};
use martian_filetypes::json_file::JsonFile;
use martian_filetypes::tabular_file::CsvFile;
use martian_filetypes::FileTypeWrite;
use metric::{join_metric_name, set, TxHashMap, TxHashSet};
use multi::config::{
    multiconst, MultiConfigCsv, MultiConfigCsvFile, ProbeBarcodeIterationMode, SamplesCsv,
};
use parameters_toml::{fiveprime_multiplexing, threeprime_lt_multiplexing};
use serde::{Deserialize, Serialize};
use slice_group_by::GroupBy;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
#[allow(clippy::enum_glob_use)]
use ChemistryName::*;

const MIN_READS_NEEDED: usize = 10_000;

#[derive(Clone, Serialize, Deserialize, MartianStruct)]
#[cfg_attr(test, derive(Default))]
pub struct DetectChemistryStageInputs {
    pub sample_def: Vec<SampleDef>,
    pub reference_path: Option<PathBuf>,
    pub feature_reference: Option<FeatureReferenceFile>,
    pub chemistry_specs: ChemistrySpecs,
    pub allowed_chems: Option<Vec<AutoOrRefinedChemistry>>,
    pub r1_length: Option<usize>,
    pub r2_length: Option<usize>,
    pub multi_config: Option<MultiConfigCsvFile>,
    pub is_pd: bool,
    pub custom_chemistry_def: Option<ChemistryDef>,
    pub feature_config: Option<FeatureConfig>,
}

impl DetectChemistryStageInputs {
    fn to_metadata(&self) -> Metadata<'_> {
        Metadata {
            allowed_chems: self.allowed_chems.as_deref(),
            sample_defs: &self.sample_def,
            reference_path: self.reference_path.as_deref(),
            feature_reference: self.feature_reference.as_ref(),
            feature_config: self.feature_config.as_ref(),
            is_pd: self.is_pd,
        }
    }
}

/// A subset of args data that is used across the stage.
struct Metadata<'a> {
    pub allowed_chems: Option<&'a [AutoOrRefinedChemistry]>,
    pub sample_defs: &'a [SampleDef],
    pub reference_path: Option<&'a Path>,
    pub feature_reference: Option<&'a FeatureReferenceFile>,
    pub feature_config: Option<&'a FeatureConfig>,
    pub is_pd: bool,
}

pub type DetectedProbeBarcodePairingFile = JsonFile<TxHashMap<BarcodeId, BarcodeId>>;

#[derive(Clone, Serialize, Deserialize, MartianStruct)]
#[cfg_attr(test, derive(Debug))]
pub struct DetectChemistryStageOutputs {
    pub chemistry_defs: ChemistryDefs,
    pub is_antibody_only: bool,
    #[mro_retain]
    pub probe_barcode_overlap: Option<CsvFile<FRPGemBarcodeOverlapRow>>,
    #[mro_retain]
    pub detected_probe_barcode_pairing: Option<DetectedProbeBarcodePairingFile>,
}

pub struct DetectChemistry;

#[make_mro(mem_gb = 20, volatile = strict)]
impl MartianMain for DetectChemistry {
    type StageInputs = DetectChemistryStageInputs;
    type StageOutputs = DetectChemistryStageOutputs;

    fn main(&self, args: Self::StageInputs, rover: MartianRover) -> Result<Self::StageOutputs> {
        // Bail out if any of chemistry_specs are not in the optional list of allowed
        // chemistry names.
        for spec in args.chemistry_specs.values() {
            validate_chemistry_spec(*spec, args.allowed_chems.as_deref())?;
        }

        let multi_config_csv = args
            .multi_config
            .as_ref()
            .map(|csv| csv.read().with_context(|| csv.display().to_string()))
            .transpose()?;

        let units = &load_reads(&args)?;
        ensure!(!units.is_empty(), "no reads loaded");

        let chemistry_defs = select_chemistries(&args, multi_config_csv.as_ref(), units)?;

        let is_antibody_only = args
            .sample_def
            .iter()
            .all(|x| x.library_type == Some(LibraryType::Antibody));

        let mut outputs = DetectChemistryStageOutputs {
            chemistry_defs,
            is_antibody_only,
            probe_barcode_overlap: None,
            detected_probe_barcode_pairing: None,
        };

        if let Some(multi_config_csv) = &multi_config_csv {
            handle_probe_barcode_translation(&rover, multi_config_csv, units, &mut outputs)?;
        }

        Ok(outputs)
    }
}

/// Top-level entry point to define the map of chemistries per library type.
fn select_chemistries(
    args: &DetectChemistryStageInputs,
    multi_config_csv: Option<&MultiConfigCsv>,
    units: &[(DetectChemistryUnit, Vec<ReadPair>)],
) -> Result<ChemistryDefs> {
    let metadata = args.to_metadata();
    // Handle manually-specified chemistry.
    Ok(match unpack_chemistry_specs(&args.chemistry_specs)? {
        UnpackedChemistrySpecs::Manual(chems) => use_manual_chemistries(
            &chems,
            &metadata,
            multi_config_csv,
            args.custom_chemistry_def.as_ref(),
            units,
        )?,

        UnpackedChemistrySpecs::Auto(mode) => {
            match detect_chemistry(mode, &metadata, multi_config_csv, units) {
                Ok(chemistry_def) => {
                    // One chemistry was compatible with all units.
                    clone_chemistry_for_libraries(&chemistry_def, units)
                }

                Err(err) => {
                    // No single chemistry was compatible with all units.
                    // Attempt to detect chemistry per library type for RTL experiments.
                    let Some(DetectChemistryErrors::ConflictingChemistries {
                        per_unit_chems, ..
                    }) = err.downcast_ref::<DetectChemistryErrors>()
                    else {
                        bail!(err);
                    };

                    let is_rtl = per_unit_chems
                        .iter()
                        .flatten()
                        .all(|chem| chem.is_rtl().unwrap_or(false));
                    ensure!(is_rtl, err);

                    detect_chemistry_per_library_type(mode, &metadata, multi_config_csv, units)?
                }
            }
        }
    })
}

/// Represent the possible valid ways of unpacking chemistry specs.
enum UnpackedChemistrySpecs {
    Auto(AutoChemistryName),
    Manual(HashMap<LibraryType, ChemistryName>),
}

/// Unpack the provided chemistry specs into either auto or manual modes.
/// Expect a single auto chemistry mode, or a refined chemistry for every lib.
fn unpack_chemistry_specs(chemistry_specs: &ChemistrySpecs) -> Result<UnpackedChemistrySpecs> {
    let mut auto_modes: Vec<_> = chemistry_specs
        .values()
        .filter_map(AutoOrRefinedChemistry::auto)
        .collect();
    if !auto_modes.is_empty() {
        ensure!(
            auto_modes.len() == chemistry_specs.len(),
            "mixing auto and refined chemistries for different library types is not supported"
        );
        auto_modes.sort();
        auto_modes.dedup();
        ensure!(
            auto_modes.len() == 1,
            "multiple conflicting auto modes provided: {}",
            auto_modes.iter().format(", ")
        );
        return Ok(UnpackedChemistrySpecs::Auto(auto_modes[0]));
    }
    Ok(UnpackedChemistrySpecs::Manual(
        chemistry_specs
            .iter()
            .map(|(lib_type, spec)| (*lib_type, spec.refined().unwrap()))
            .collect(),
    ))
}

/// Validate manual chemistries for all units.
///
/// If any libraries are specified as custom chemistry, use the provided custom
/// chemistry for them.  No validation is performed for custom chemistries.
fn use_manual_chemistries(
    chems: &HashMap<LibraryType, ChemistryName>,
    metadata: &Metadata<'_>,
    multi_config_csv: Option<&MultiConfigCsv>,
    custom_chemistry_def: Option<&ChemistryDef>,
    all_units: &[(DetectChemistryUnit, Vec<ReadPair>)],
) -> Result<ChemistryDefs> {
    divide_units_by_library_type(all_units)
        .map(|(library_type, units)| {
            let chem_name = chems[&library_type];
            if chem_name == ChemistryName::Custom {
                use_custom_chemistry(custom_chemistry_def)
            } else {
                use_manual_chemistry(chem_name, metadata, multi_config_csv, units)
            }
            .map(|chem_def| (library_type, chem_def))
        })
        .try_collect()
}

/// If custom chemistry is specified, extract it from the args.
fn use_custom_chemistry(custom_chemistry_def: Option<&ChemistryDef>) -> Result<ChemistryDef> {
    let Some(custom_def) = custom_chemistry_def else {
        bail!(
            "custom chemistry def should be present if the input chemistry is '{}'",
            ChemistryName::Custom
        );
    };
    ensure!(
        custom_def.name == ChemistryName::Custom,
        "expected a custom chemistry but found {}",
        custom_def.name
    );
    Ok(normalize_chemistry_def(custom_def.clone()).unwrap_or_else(|| custom_def.clone()))
}

/// If manual chemistry is specified, extract and validate it.
///
/// Check that it passes the whitelist filter. Manual chemistry is used as an
/// escape hatch by customers for data which is usually of lower quality or by
/// QA to run non-standard fuzzed FASTQs. No minimum number of reads is enforced
/// here. Emit a warning if there are few valid barcodes.
fn use_manual_chemistry(
    chem: ChemistryName,
    metadata: &Metadata<'_>,
    multi_config_csv: Option<&MultiConfigCsv>,
    units: &[(DetectChemistryUnit, Vec<ReadPair>)],
) -> Result<ChemistryDef> {
    let chems = &set![chem];

    if chem.is_rtl_multiplexed() {
        // Check the read length to ensure that the probe barcode is sequenced.
        let _matching_chemistries = LengthFilter::new(
            chems,
            chems,
            metadata.feature_reference,
            metadata.feature_config,
        )?
        .filter_chemistries(units)?;
    }

    if let Err(err) = WhitelistMatchFilter::new(chems, chems)?.filter_chemistries(units) {
        if chem.is_spatial() {
            bail!(err);
        }
        println!("WARNING: {err:#}");
    }

    validate_multiplexing(chem, metadata.sample_defs)?;
    validate_rtl(multi_config_csv, chem)?;
    Ok(ChemistryDef::named(chem))
}

/// Populate a ChemistryDefs map with a copy of the chemistry for each library type.
fn clone_chemistry_for_libraries(
    chemistry_def: &ChemistryDef,
    units: &[(DetectChemistryUnit, Vec<ReadPair>)],
) -> ChemistryDefs {
    units
        .iter()
        .map(|(unit, _reads)| unit.library_type)
        .unique()
        .zip(std::iter::repeat(chemistry_def.clone()))
        .collect()
}

/// Load a subsample of reads from each input unit.
fn load_reads(
    args: &DetectChemistryStageInputs,
) -> Result<Vec<(DetectChemistryUnit, Vec<ReadPair>)>> {
    let units = detect_chemistry_units(&args.sample_def, args.r1_length, args.r2_length)?;
    println!("Number of fastq units = {}", units.len());

    // Check for duplicate R1 and R2 files amongst units.
    check_fastq_identity(&units)?;

    units
        .into_iter()
        .sorted_by_key(|unit| unit.library_type)
        .map(|unit| {
            println!("Sampling reads from: {unit}");
            let reads = unit.sampled_read_pairs()?;
            Ok((unit, reads))
        })
        .try_collect()
}

/// Determine which chemistry to use for the specified auto detection mode.
fn detect_chemistry(
    mode: AutoChemistryName,
    metadata: &Metadata<'_>,
    multi_config_csv: Option<&MultiConfigCsv>,
    units: &[(DetectChemistryUnit, Vec<ReadPair>)],
) -> Result<ChemistryDef> {
    // Check for overhang multiplexing
    let is_overhang_multiplexed =
        multi_config_csv.is_some_and(MultiConfigCsv::is_overhang_multiplexed);

    // -----------------------------------------------------------------------------------------
    // If the sample def contains VDJ, none of the other library types should be present and
    // the chemistry def should be `SCVDJ_auto`.
    if metadata
        .sample_defs
        .iter()
        .any(|sd| sd.library_type.is_some_and(|lt| lt.is_vdj()))
    {
        ensure!(
            metadata
                .sample_defs
                .iter()
                .all(|sd| sd.library_type.is_some_and(|lt| lt.is_vdj())),
            "VDJ cannot be mixed with other library types"
        );
        ensure!(
            mode == AutoChemistryName::Vdj,
            "VDJ chemistry_name_spec should be SCVDJ_auto"
        );
    }

    let possible_chemistries = mode.allowed_chemistries(
        metadata.is_pd,
        multi_config_csv.is_some_and(|x| x.samples.is_some()),
        multi_config_csv.is_some_and(is_rtl_uncollapsed),
    );

    let allowed_chemistries = metadata.allowed_chems.map_or_else(
        || possible_chemistries.clone(),
        |chems| {
            chems
                .iter()
                .filter_map(AutoOrRefinedChemistry::refined)
                .collect()
        },
    );

    for (unit, read_pairs) in units {
        ensure!(
            read_pairs.len() >= MIN_READS_NEEDED,
            DetectChemistryErrors::NotEnoughReads {
                num_reads: read_pairs.len(),
                min_reads: MIN_READS_NEEDED,
                unit: Box::new(unit.clone()),
            }
        );
    }

    println!(
        "Potential chemistries: {}",
        possible_chemistries.iter().sorted().format(", ")
    );

    // Read length based filtering
    let length_matching_chemistries = LengthFilter::new(
        &allowed_chemistries,
        &possible_chemistries,
        metadata.feature_reference,
        metadata.feature_config,
    )?
    .filter_chemistries(units)?;

    println!(
        "After length filter: {}",
        length_matching_chemistries.iter().sorted().format(", ")
    );

    // -----------------------------------------------------------------------------------------
    // For each unit of fastqs, ensure that a sufficient fraction of reads contain barcodes
    // which match the whitelist for at least one of the possible chemistries
    let wl_matching_chemistries =
        WhitelistMatchFilter::new(&allowed_chemistries, &length_matching_chemistries)?
            .filter_chemistries(units)?;

    let chosen_chemistry_def = if wl_matching_chemistries.len() == 1 {
        let &chem = wl_matching_chemistries.iter().exactly_one().unwrap();
        if chem == SFRP {
            // Bail out if a mixture of probe barcodes is observed for singleplex FRP chemistry
            validate_no_probe_bc_mixture_in_sfrp(
                units,
                metadata.feature_reference,
                metadata.feature_config,
            )?;
        }
        Some(validate_chemistry(
            chem,
            metadata.allowed_chems,
            metadata.sample_defs,
            multi_config_csv,
            is_overhang_multiplexed,
        )?)
    } else if wl_matching_chemistries == set![ThreePrimeV3, ThreePrimeV3HT, ThreePrimeV3LT] {
        Some(validate_chemistry(
            ThreePrimeV3LT,
            metadata.allowed_chems,
            metadata.sample_defs,
            multi_config_csv,
            is_overhang_multiplexed,
        )?)
    } else if wl_matching_chemistries == set![ThreePrimeV3, ThreePrimeV3HT] {
        Some(validate_chemistry(
            ThreePrimeV3,
            metadata.allowed_chems,
            metadata.sample_defs,
            multi_config_csv,
            is_overhang_multiplexed,
        )?)
    } else if wl_matching_chemistries == set![ThreePrimeV4, ThreePrimeV4HT] {
        Some(validate_chemistry(
            ThreePrimeV4,
            metadata.allowed_chems,
            metadata.sample_defs,
            multi_config_csv,
            is_overhang_multiplexed,
        )?)
    } else if wl_matching_chemistries == set![FivePrimeR2, FivePrimeHT] {
        Some(validate_chemistry(
            FivePrimeR2,
            metadata.allowed_chems,
            metadata.sample_defs,
            multi_config_csv,
            is_overhang_multiplexed,
        )?)
    } else if wl_matching_chemistries == set![FivePrimeR2V3, FivePrimeHTV3] {
        Some(validate_chemistry(
            FivePrimeR2V3,
            metadata.allowed_chems,
            metadata.sample_defs,
            multi_config_csv,
            is_overhang_multiplexed,
        )?)
    } else {
        None
    };
    if let Some(chosen_chemistry_def) = chosen_chemistry_def {
        return Ok(chosen_chemistry_def);
    }

    println!(
        "After whitelist filter: {}",
        wl_matching_chemistries.iter().sorted().format(", ")
    );

    let is_antibody_only = metadata
        .sample_defs
        .iter()
        .all(|sd| sd.library_type == Some(LibraryType::Antibody));
    let expected_mapping_chemistries = if is_antibody_only {
        let mut result = wl_matching_chemistries;
        // we define a new chemistry named "SC-FB" because this could be 3' v2 polyA capture
        // antibody library or a 5' antibody library
        let redundant_ab_chems = [ThreePrimeV2, FivePrimeR2, FivePrimePE];
        if redundant_ab_chems.iter().any(|chem| result.contains(chem)) {
            result.insert(FeatureBarcodingOnly);
            for chem in &redundant_ab_chems {
                result.remove(chem);
            }
        }
        result
    } else {
        let mut mapper = ReadMappingFilter::new(
            metadata.reference_path.unwrap(),
            &allowed_chemistries,
            wl_matching_chemistries,
        )?;
        mapper.filter_chemistries(units)?
    };

    println!(
        "After mapping filter: {}",
        expected_mapping_chemistries.iter().sorted().format(", ")
    );

    ensure!(
        expected_mapping_chemistries.len() == 1,
        "Could not distinguish between {}",
        expected_mapping_chemistries.iter().sorted().format(", ")
    );

    validate_chemistry(
        expected_mapping_chemistries
            .into_iter()
            .exactly_one()
            .unwrap(),
        metadata.allowed_chems,
        metadata.sample_defs,
        multi_config_csv,
        is_overhang_multiplexed,
    )
}

/// Run chemistry detection on every library independently.
/// This variant produces an independent result for each library without any
/// constraint on the individual chemistries being mutually compatible.
fn detect_chemistry_per_library_type(
    mode: AutoChemistryName,
    metadata: &Metadata<'_>,
    multi_config_csv: Option<&MultiConfigCsv>,
    all_units: &[(DetectChemistryUnit, Vec<ReadPair>)],
) -> Result<ChemistryDefs> {
    divide_units_by_library_type(all_units)
        .map(|(library_type, units)| {
            println!("\nDetecting chemistry for {library_type}");
            let chemistry = detect_chemistry(mode, metadata, multi_config_csv, units)?;
            ensure!(
                chemistry.name.compatible_with_library_type(library_type),
                "The chemistry {} was detected for {library_type} but they are not compatible; \
                 please check that your library configurations are associated with the correct \
                 library type.",
                chemistry.name,
            );
            println!("\nDetected chemistry {} for {library_type}", chemistry.name);
            anyhow::Ok((library_type, chemistry))
        })
        .try_collect()
}

/// Divide up the read units into slices based on library type.
fn divide_units_by_library_type(
    units: &[(DetectChemistryUnit, Vec<ReadPair>)],
) -> impl Iterator<Item = (LibraryType, &[(DetectChemistryUnit, Vec<ReadPair>)])> {
    units
        .linear_group_by_key(|(unit, _reads)| unit.library_type)
        .map(|units| (units[0].0.library_type, units))
}

/// Validate that the specified chemistry is allowed.
fn validate_chemistry_spec(
    chemistry_spec: AutoOrRefinedChemistry,
    allowed_chems: Option<&[AutoOrRefinedChemistry]>,
) -> Result<()> {
    let Some(allowed_chems) = allowed_chems else {
        return Ok(());
    };

    ensure!(
        allowed_chems.contains(&chemistry_spec),
        DetectChemistryErrors::ChemistryNotAllowed {
            input: chemistry_spec,
            allowed: allowed_chems.to_vec(),
        }
    );

    Ok(())
}

/// Validate a chemistry.
fn validate_chemistry(
    chemistry: ChemistryName,
    allowed_chems: Option<&[AutoOrRefinedChemistry]>,
    sample_defs: &[SampleDef],
    multi_config_csv: Option<&MultiConfigCsv>,
    overhang_multiplexing: bool,
) -> Result<ChemistryDef> {
    if chemistry == ChemistryName::ThreePrimeV3LT {
        bail!("The chemistry SC3Pv3LT (Single Cell 3'v3 LT) is no longer supported. To analyze this data, use Cell Ranger 7.2 or earlier.");
    }
    if chemistry == ChemistryName::ArcV1 {
        bail!(
            "Cell Ranger detected the chemistry {chemistry}, which may indicate a workflow \
            error during sample preparation. Please check the reagents used to prepare this \
            sample and contact 10x Genomics support for further assistance. If this workflow is \
            intentional, you can force Cell Ranger to process this data by manually specifying \
            the {chemistry} chemistry in your analysis configuration."
        );
    }
    if chemistry == ChemistryName::ThreePrimeV4
        && sample_defs
            .iter()
            .any(|sdef| sdef.library_type == Some(LibraryType::Crispr))
    {
        bail!("The chemistry SC3Pv4 (Single Cell 3'v4) is not supported with CRISPR Guide Capture libraries.")
    }

    validate_multiplexing(chemistry, sample_defs)?;
    validate_rtl(multi_config_csv, chemistry)?;

    let chemistry_with_overhang = if overhang_multiplexing {
        chemistry.get_overhang_version()?
    } else {
        chemistry
    };

    validate_chemistry_spec(
        AutoOrRefinedChemistry::Refined(chemistry_with_overhang),
        allowed_chems,
    )?;

    Ok(ChemistryDef::named(chemistry_with_overhang))
}

/// Validate combinations of chemistry and types of multiplexing.
fn validate_multiplexing(chemistry_type: ChemistryName, sample_defs: &[SampleDef]) -> Result<()> {
    if !sample_defs
        .iter()
        .any(|sdef| sdef.library_type == Some(LibraryType::Cellplex))
    {
        return Ok(());
    }

    match chemistry_type {
        ThreePrimeV3LT => ensure!(
            *threeprime_lt_multiplexing()?,
            "Multiplexing Capture libraries are not supported with Single Cell 3' v3 LT chemistry"
        ),
        FivePrimeR1 | FivePrimeR2 | FivePrimePE => ensure!(
            *fiveprime_multiplexing()?,
            "Multiplexing Capture libraries are not supported with Single Cell 5' chemistries"
        ),
        _ => (),
    }
    Ok(())
}

/// Validate the chemistry with RTL-related parameters.
fn validate_rtl(multi_config_csv: Option<&MultiConfigCsv>, chemistry: ChemistryName) -> Result<()> {
    let Some(config) = multi_config_csv else {
        // cellranger count does not support RTL chemistries.
        return Ok(());
    };

    if !chemistry.is_rtl().expect("is_rtl is None only for custom and spatial chemistries, which do not use chemistry detection") {
        let Some(samples) = &config.samples else {
            return Ok(());
        };
        ensure!(
            !samples.has_probe_barcode_ids(),
            "A non-Fixed RNA Profiling chemistry {chemistry} was detected, and the [samples] \
             section has a probe_barcode_ids column. The probe_barcode_ids column may only be \
             specified with Fixed RNA Profiling chemistries.",
        );
        return Ok(());
    }

    if let Some(gex) = &config.gene_expression {
        if config.libraries.has_gene_expression() {
            ensure!(
                gex.probe_set.is_some(),
                "Fixed RNA Profiling chemistries require a probe-set."
            );
        }
        ensure!(
            gex.include_introns == multiconst::DEFAULT_INCLUDE_INTRONS,
            "The [gene-expression] section specifies the parameter include-introns, \
             which is not valid for Fixed RNA Profiling chemistries."
        );
    }

    if chemistry == SFRP {
        ensure!(
            config.samples.is_none(),
            "We detected singleplex Fixed RNA Profiling chemistry from the data. \
             Sample definitions are unsupported for singleplex inputs. \
             To process this data as multiplex Fixed RNA Profiling you will need to specify `MFRP` \
             as the chemistry in the config.csv."
        );
    }

    Ok(())
}

/// Return true if probe barcode IDs are uncollapsed, like BC001A. For PD use.
fn is_rtl_uncollapsed(multi_config_csv: &MultiConfigCsv) -> bool {
    multi_config_csv
        .sample_barcode_ids_used_in_experiment(ProbeBarcodeIterationMode::All)
        .into_iter()
        .all(|x| x.ends_with(&['A', 'B', 'C', 'D']))
}

/// Identify probe barcode pairings if necessary.
/// If pairings are present, modify the translation whitelist to include those pairings.
fn handle_probe_barcode_translation(
    rover: &MartianRover,
    multi_config_csv: &MultiConfigCsv,
    units: &[(DetectChemistryUnit, Vec<ReadPair>)],
    outputs: &mut DetectChemistryStageOutputs,
) -> Result<()> {
    let detected_pairing_id_translation = if !should_detect_probe_barcode_pairing(multi_config_csv)
    {
        None
    } else {
        let (overlaps, pairings) = detect_probe_barcode_pairing(&outputs.chemistry_defs, units)?;

        let probe_barcode_overlap: CsvFile<_> = rover.make_path("probe_barcode_overlap");
        probe_barcode_overlap.write(&overlaps)?;
        outputs.probe_barcode_overlap = Some(probe_barcode_overlap);

        let pairings = pairings.into_iter().collect();
        let detected_probe_barcode_pairing: JsonFile<_> =
            rover.make_path("detected_probe_barcode_pairing");
        detected_probe_barcode_pairing.write(&pairings)?;
        outputs.detected_probe_barcode_pairing = Some(detected_probe_barcode_pairing);

        if pairings.is_empty() {
            None
        } else {
            // The pairings are generated as RTL: AB; invert the mapping so they
            // serve to translate an AB barcode into the paired RTL barcode.
            Some(pairings.into_iter().map(|(rtl, ab)| (ab, rtl)).collect())
        }
    };

    let explicit_pairing = multi_config_csv
        .samples
        .as_ref()
        .map(SamplesCsv::get_translated_probe_barcodes)
        .and_then(|pairing| {
            if pairing.is_empty() {
                None
            } else {
                Some(pairing)
            }
        });

    // Remap the probe barcode whitelist(s) if we have a probe barcode pairing.
    // Use the explicit pairing if one is provided.
    // Otherwise, use the detected pairing if there are any.
    if let Some(pairing) = explicit_pairing.or(detected_pairing_id_translation) {
        // Use the GEX probe barcode whitelist as the target.
        let target_probe_bc_whitelist = outputs.chemistry_defs[&LibraryType::Gex]
            .barcode_whitelist()
            .probe()
            .as_source(true)?;
        for (&library_type, chemistry_def) in &mut outputs.chemistry_defs {
            chemistry_def.translate_probe_barcode_whitelist_with_id_map(
                &pairing,
                &target_probe_bc_whitelist,
                rover.make_path(join_metric_name(
                    library_type,
                    "probe_barcode_translation_whitelist.tsv",
                )),
            )?;
        }
    };
    Ok(())
}

