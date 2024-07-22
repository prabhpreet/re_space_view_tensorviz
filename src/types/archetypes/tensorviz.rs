use re_types::ComponentName;

use crate::types::components::TeraSVG;

#[derive(Clone, Debug, PartialEq)]
pub enum TensorViz {
    TeraSVG(TeraSVG),
}

impl TensorViz {
    #[inline]
    pub fn new_svg(svg_template: impl Into<crate::types::components::TeraSVG>) -> Self {
        TensorViz::TeraSVG(svg_template.into())
    }
}

impl From<crate::types::components::TeraSVG> for TensorViz {
    fn from(value: crate::types::components::TeraSVG) -> Self {
        TensorViz::TeraSVG(value)
    }
}

impl re_types::SizeBytes for TensorViz {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        match self {
            TensorViz::TeraSVG(v) => v.heap_size_bytes(),
        }
    }
    #[inline]
    fn is_pod() -> bool {
        false
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["tv.components.TensorVizIndicator".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| [TeraSVG::NAME.into()]);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "tv.components.TensorVizIndicator".into(),
            TeraSVG::NAME.into(),
        ]
    });

impl TensorViz {
    /// The total number of components in the archetype: 1 required, 1 recommended, 0 optional
    pub const NUM_COMPONENTS: usize = 2usize;
}

/// Indicator component for the [`TensorViz`] [`re_types::Archetype`]
pub type TensorVizIndicator = re_types::GenericIndicatorComponent<TensorViz>;

impl re_types::Archetype for TensorViz {
    type Indicator = TensorVizIndicator;

    fn name() -> re_sdk::ArchetypeName {
        "tv.archetypes.TensorViz".into()
    }

    fn required_components() -> std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    fn recommended_components() -> std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    fn optional_components() -> std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    fn all_components() -> std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    fn indicator() -> re_sdk::MaybeOwnedComponentBatch<'static> {
        re_sdk::MaybeOwnedComponentBatch::Owned(
            Box::<<Self as re_sdk::Archetype>::Indicator>::default(),
        )
    }

    fn display_name() -> &'static str {
        "TensorViz"
    }
}

impl re_types::AsComponents for TensorViz {
    fn as_component_batches(&self) -> Vec<re_sdk::MaybeOwnedComponentBatch<'_>> {
        use re_types::Archetype as _;
        match self {
            TensorViz::TeraSVG(v) => vec![
                Self::indicator(),
                (v as &dyn re_types::ComponentBatch).into(),
            ],
        }
    }
}
