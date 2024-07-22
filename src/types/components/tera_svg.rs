use itertools::Itertools;
use re_types::{components::ClassId, external::arrow2};

#[derive(Clone, Debug, PartialEq)]
pub struct TeraSVG(pub re_types::components::Text);

impl re_types::SizeBytes for TeraSVG {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <re_types::components::Text>::is_pod()
    }
}

impl<T: Into<re_types::components::Text>> From<T> for TeraSVG {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

re_types::macros::impl_into_cow!(TeraSVG);

impl TeraSVG {
    pub const NAME: &'static str = "tv.components.TeraSVG";
}

impl re_types::Loggable for TeraSVG {
    type Name = re_types::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        Self::NAME.into()
    }

    #[allow(clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> re_types::external::arrow2::datatypes::DataType {
        re_types::components::Text::arrow_datatype()
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<std::borrow::Cow<'a, Self>>>>,
    ) -> re_types::SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: 'a,
    {
        let data = data.into_iter().map(|d| d.map(|d| d.into().0.to_owned()));
        re_types::components::Text::to_arrow_opt(data)
    }

    fn from_arrow_opt(
        data: &dyn arrow2::array::Array,
    ) -> re_types::DeserializationResult<Vec<Option<Self>>> {
        re_types::components::Text::from_arrow_opt(data)
            .map(|v| v.into_iter().map(|v| v.map(|v| TeraSVG(v))).collect_vec())
    }
}
