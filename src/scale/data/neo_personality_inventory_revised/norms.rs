use super::{
    AgreeablenessSubdimension, ConscientiousnessSubdimension, Dimension, DimensionNorm,
    ExtraversionSubdimension, NegativeEmotionalitySubdimension, Norm, NormData, NormGender,
    OpenMindednessSubdimension, Subdimension, SubdimensionNorm,
};

pub(super) const NORM: Norm = Norm {
    male: NormGender {
        dimension_norm: &[
            DimensionNorm {
                dimension: Dimension::N,
                data: NormData { m: 83.8, sd: 24.41 },
            },
            DimensionNorm {
                dimension: Dimension::E,
                data: NormData {
                    m: 109.97,
                    sd: 18.27,
                },
            },
            DimensionNorm {
                dimension: Dimension::O,
                data: NormData {
                    m: 110.65,
                    sd: 15.88,
                },
            },
            DimensionNorm {
                dimension: Dimension::A,
                data: NormData { m: 113.0, sd: 14.9 },
            },
            DimensionNorm {
                dimension: Dimension::C,
                data: NormData {
                    m: 123.89,
                    sd: 20.32,
                },
            },
        ],
        subdimension_norm: &[
            // 维度 N
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N1),
                data: NormData { m: 14.15, sd: 5.7 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N2),
                data: NormData { m: 13.82, sd: 5.24 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N3),
                data: NormData { m: 14.37, sd: 5.7 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N4),
                data: NormData { m: 15.35, sd: 4.73 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N5),
                data: NormData { m: 14.86, sd: 4.88 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N6),
                data: NormData { m: 11.23, sd: 5.09 },
            },
            // 维度 E
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E1),
                data: NormData { m: 21.53, sd: 4.27 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E2),
                data: NormData { m: 18.69, sd: 4.65 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E3),
                data: NormData { m: 15.1, sd: 4.37 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E4),
                data: NormData { m: 18.33, sd: 4.74 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E5),
                data: NormData { m: 17.24, sd: 4.64 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E6),
                data: NormData { m: 19.07, sd: 4.85 },
            },
            // 维度 O
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O1),
                data: NormData { m: 15.19, sd: 4.19 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O2),
                data: NormData { m: 20.7, sd: 4.88 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O3),
                data: NormData { m: 19.47, sd: 3.95 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O4),
                data: NormData { m: 15.02, sd: 3.61 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O5),
                data: NormData { m: 19.62, sd: 4.98 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O6),
                data: NormData { m: 20.64, sd: 3.97 },
            },
            // 维度 A
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A1),
                data: NormData { m: 21.07, sd: 3.94 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A2),
                data: NormData { m: 19.03, sd: 5.11 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A3),
                data: NormData { m: 20.99, sd: 3.53 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A4),
                data: NormData { m: 15.33, sd: 4.64 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A5),
                data: NormData { m: 16.67, sd: 4.06 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A6),
                data: NormData { m: 19.91, sd: 3.59 },
            },
            // 维度 C
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C1),
                data: NormData { m: 20.8, sd: 4.34 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C2),
                data: NormData { m: 18.39, sd: 4.21 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C3),
                data: NormData { m: 23.47, sd: 4.22 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C4),
                data: NormData { m: 20.02, sd: 4.34 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C5),
                data: NormData { m: 20.48, sd: 4.66 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C6),
                data: NormData { m: 20.72, sd: 4.63 },
            },
        ],
    },
    female: NormGender {
        dimension_norm: &[
            DimensionNorm {
                dimension: Dimension::N,
                data: NormData {
                    m: 87.26,
                    sd: 24.57,
                },
            },
            DimensionNorm {
                dimension: Dimension::E,
                data: NormData {
                    m: 107.13,
                    sd: 18.85,
                },
            },
            DimensionNorm {
                dimension: Dimension::O,
                data: NormData {
                    m: 110.31,
                    sd: 17.26,
                },
            },
            DimensionNorm {
                dimension: Dimension::A,
                data: NormData {
                    m: 119.0,
                    sd: 15.79,
                },
            },
            DimensionNorm {
                dimension: Dimension::C,
                data: NormData {
                    m: 125.08,
                    sd: 17.67,
                },
            },
        ],
        subdimension_norm: &[
            // 维度 N
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N1),
                data: NormData { m: 14.81, sd: 5.45 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N2),
                data: NormData { m: 13.99, sd: 5.16 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N3),
                data: NormData { m: 14.46, sd: 5.45 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N4),
                data: NormData { m: 16.38, sd: 5.27 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N5),
                data: NormData { m: 14.89, sd: 5.1 },
            },
            SubdimensionNorm {
                dimension: Subdimension::N(NegativeEmotionalitySubdimension::N6),
                data: NormData { m: 12.73, sd: 5.18 },
            },
            // 维度 E
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E1),
                data: NormData { m: 22.05, sd: 4.31 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E2),
                data: NormData { m: 18.3, sd: 4.85 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E3),
                data: NormData { m: 14.59, sd: 4.67 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E4),
                data: NormData { m: 18.01, sd: 4.93 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E5),
                data: NormData { m: 14.25, sd: 5.35 },
            },
            SubdimensionNorm {
                dimension: Subdimension::E(ExtraversionSubdimension::E6),
                data: NormData { m: 18.94, sd: 5.09 },
            },
            // 维度 O
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O1),
                data: NormData { m: 15.89, sd: 4.78 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O2),
                data: NormData { m: 24.05, sd: 4.78 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O3),
                data: NormData { m: 19.59, sd: 4.32 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O4),
                data: NormData { m: 15.02, sd: 3.88 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O5),
                data: NormData { m: 17.68, sd: 5.33 },
            },
            SubdimensionNorm {
                dimension: Subdimension::O(OpenMindednessSubdimension::O6),
                data: NormData { m: 20.68, sd: 3.76 },
            },
            // 维度 A
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A1),
                data: NormData { m: 21.84, sd: 4.43 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A2),
                data: NormData { m: 21.54, sd: 4.97 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A3),
                data: NormData { m: 21.19, sd: 3.88 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A4),
                data: NormData { m: 16.19, sd: 4.72 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A5),
                data: NormData { m: 17.68, sd: 4.62 },
            },
            SubdimensionNorm {
                dimension: Subdimension::A(AgreeablenessSubdimension::A6),
                data: NormData { m: 20.56, sd: 3.76 },
            },
            // 维度 C
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C1),
                data: NormData { m: 20.44, sd: 4.33 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C2),
                data: NormData { m: 19.32, sd: 4.75 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C3),
                data: NormData { m: 24.18, sd: 3.48 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C4),
                data: NormData { m: 19.69, sd: 4.07 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C5),
                data: NormData { m: 21.01, sd: 4.03 },
            },
            SubdimensionNorm {
                dimension: Subdimension::C(ConscientiousnessSubdimension::C6),
                data: NormData { m: 20.43, sd: 4.66 },
            },
        ],
    },
};
