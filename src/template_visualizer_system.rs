use re_data_store::LatestAtQuery;
use re_space_view::DataResultQuery;
use re_viewer_context::{IdentifiedViewSystem, VisualizerQueryInfo, VisualizerSystem};

#[derive(Default, Debug)]
pub struct TVSystem {
    pub template: String,
}

impl IdentifiedViewSystem for TVSystem {
    fn identifier() -> re_viewer_context::ViewSystemIdentifier {
        "TemplateVisualizerSystem".into()
    }
}

impl VisualizerSystem for TVSystem {
    fn visualizer_query_info(&self) -> re_viewer_context::VisualizerQueryInfo {
        //let mut query = VisualizerQueryInfo::from_archetype::<re_types::archetypes::Tensor>();
        //let mut query = VisualizerQueryInfo::empty();
        let query = VisualizerQueryInfo::from_archetype::<crate::types::archetypes::TensorViz>();

        query
    }

    fn execute(
        &mut self,
        ctx: &re_viewer_context::ViewContext<'_>,
        query: &re_viewer_context::ViewQuery<'_>,
        context_systems: &re_viewer_context::ViewContextCollection,
    ) -> Result<Vec<re_renderer::QueueableDrawData>, re_viewer_context::SpaceViewSystemExecutionError>
    {
        let timeline_query = LatestAtQuery::new(query.timeline, query.latest_at);

        for data_result in query.iter_visible_data_results(ctx, Self::identifier()) {
            let svg_results = data_result
                .latest_at_with_blueprint_resolved_data::<crate::types::archetypes::TensorViz>(
                    ctx,
                    &timeline_query,
                );

            if let Some(svg) = svg_results.get_required_mono::<crate::types::components::TeraSVG>()
            {
                self.template = svg.0.to_string();
            }
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

re_viewer_context::impl_component_fallback_provider!(TVSystem => []);
