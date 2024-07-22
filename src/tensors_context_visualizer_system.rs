use re_data_store::LatestAtQuery;
use re_space_view::{DataResultQuery, RangeResultsExt};
use re_types::{
    datatypes::{TensorBuffer, TensorDimension},
    ArrowBuffer,
};
use re_viewer_context::{IdentifiedViewSystem, VisualizerQueryInfo, VisualizerSystem};
use serde::Serialize;

#[derive(Default, Debug)]
pub struct LTVSystem {
    pub context: tera::Context,
}

impl IdentifiedViewSystem for LTVSystem {
    fn identifier() -> re_viewer_context::ViewSystemIdentifier {
        "LabeledTensorVisualizer".into()
    }
}

impl VisualizerSystem for LTVSystem {
    fn visualizer_query_info(&self) -> re_viewer_context::VisualizerQueryInfo {
        let query = VisualizerQueryInfo::from_archetype::<re_types::archetypes::Tensor>();

        query
    }

    fn execute(
        &mut self,
        ctx: &re_viewer_context::ViewContext<'_>,
        query: &re_viewer_context::ViewQuery<'_>,
        _context_systems: &re_viewer_context::ViewContextCollection,
    ) -> Result<Vec<re_renderer::QueueableDrawData>, re_viewer_context::SpaceViewSystemExecutionError>
    {
        let timeline_query = LatestAtQuery::new(query.timeline, query.latest_at);

        for data_result in query.iter_visible_data_results(ctx, Self::identifier()) {
            let results = data_result
                .latest_at_with_blueprint_resolved_data::<re_types::archetypes::Tensor>(
                    ctx,
                    &timeline_query,
                );

            let Some(tensor) = results.get_required_mono::<re_types::components::TensorData>()
            else {
                continue;
            };

            let entity_path = data_result.entity_path.to_string().replace("/", "__");

            let mut context = tera::Context::new();

            match &tensor.buffer {
                TensorBuffer::U8(v) => {
                    visit_tensor(
                        &mut self.context,
                        &v,
                        tensor.shape(),
                        "".to_owned(),
                        0,
                        v.len(),
                    );
                }
                TensorBuffer::U16(v) => {
                    visit_tensor(
                        &mut self.context,
                        &v,
                        tensor.shape(),
                        "".to_owned(),
                        0,
                        v.len(),
                    );
                }
                TensorBuffer::U32(v) => {
                    visit_tensor(
                        &mut self.context,
                        &v,
                        tensor.shape(),
                        "".to_owned(),
                        0,
                        v.len(),
                    );
                }
                TensorBuffer::U64(v) => {
                    visit_tensor(
                        &mut self.context,
                        &v,
                        tensor.shape(),
                        "".to_owned(),
                        0,
                        v.len(),
                    );
                }
                TensorBuffer::I8(v) => {
                    visit_tensor(
                        &mut self.context,
                        &v,
                        tensor.shape(),
                        "".to_owned(),
                        0,
                        v.len(),
                    );
                }
                TensorBuffer::I16(v) => {
                    visit_tensor(
                        &mut self.context,
                        &v,
                        tensor.shape(),
                        "".to_owned(),
                        0,
                        v.len(),
                    );
                }
                TensorBuffer::I32(v) => {
                    visit_tensor(
                        &mut self.context,
                        &v,
                        tensor.shape(),
                        "".to_owned(),
                        0,
                        v.len(),
                    );
                }
                TensorBuffer::I64(v) => {
                    visit_tensor(
                        &mut self.context,
                        &v,
                        tensor.shape(),
                        "".to_owned(),
                        0,
                        v.len(),
                    );
                }
                TensorBuffer::F16(v) => {
                    let v: ArrowBuffer<f32> =
                        v.iter().map(|x| x.to_f32()).collect::<Vec<f32>>().into();
                    visit_tensor(
                        &mut self.context,
                        &v,
                        tensor.shape(),
                        "".to_owned(),
                        0,
                        v.len(),
                    );
                }
                TensorBuffer::F32(v) => {
                    visit_tensor(
                        &mut self.context,
                        &v,
                        tensor.shape(),
                        "".to_owned(),
                        0,
                        v.len(),
                    );
                }
                TensorBuffer::F64(v) => {
                    visit_tensor(&mut context, &v, tensor.shape(), "".to_owned(), 0, v.len());
                }
                //Unsupported types
                TensorBuffer::Jpeg(_) => {}
                TensorBuffer::Nv12(_) => {}
                TensorBuffer::Yuy2(_) => {}
            }

            let context = context.into_json();

            self.context.insert(entity_path, &context);
        }

        Ok(Vec::new())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_fallback_provider(&self) -> &dyn re_viewer_context::ComponentFallbackProvider {
        self
    }
}

re_viewer_context::impl_component_fallback_provider!(LTVSystem => []);

fn visit_tensor<T: Clone + Serialize + std::fmt::Debug>(
    map: &mut tera::Context,
    buffer: &ArrowBuffer<T>,
    remaining_shape: &[TensorDimension],
    prefix_str: String,
    start_index: usize,
    end_index: usize,
) {
    // Format:
    //
    // Eg. shape: [phasor=1, data=2]
    //     Values:
    //     phasor0__data
    //
    // Eg. shape: [2, data=2]
    //      Values:
    //      0__data
    //      1__data

    let name = if let Some(Some(name)) = remaining_shape
        .get(0)
        .map(|s| s.name.as_ref().map(|s| s.to_string()))
    {
        name
    } else {
        "".to_string()
    };

    if remaining_shape.len() == 0 {
        return;
    } else if remaining_shape.len() == 1 {
        let name = format!("{}__{}", prefix_str, name);
        let v = buffer[start_index..end_index].to_vec();
        map.insert(name, &v);
        return;
    }

    let dim = &remaining_shape[0];
    let rest = &remaining_shape[1..];
    let num = dim.size as usize;

    let buf_size = remaining_shape
        .iter()
        .fold(1, |acc, x| acc * x.size as usize);

    //Split buffer into chunks of size and iterate over them
    for i in 0..num {
        let new_prefix = format!("{}__{}{}", prefix_str, name, i);
        let start = start_index + (i * buf_size);
        let end = start + buf_size;
        visit_tensor(map, buffer, rest, new_prefix, start, end);
    }
}
