use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use re_space_view::controls;
use re_types::View;
use re_viewer_context::{
    SpaceViewClass, SpaceViewSpawnHeuristics, SpaceViewState, SpaceViewStateExt,
};
use tera::Tera;

use crate::{template_visualizer_system::TVSystem, tensors_context_visualizer_system::LTVSystem};

#[derive(Clone, Default)]
pub struct TVSpaceViewState {
    tera: Tera,
    /// Reset has been performed (from a new view or state)
    reset_done: bool,
}

impl SpaceViewState for TVSpaceViewState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct TVView;

impl re_types::SizeBytes for TVView {
    fn heap_size_bytes(&self) -> u64 {
        0
    }

    fn is_pod() -> bool {
        true
    }
}

impl re_types::View for TVView {
    #[inline]
    fn identifier() -> re_types::SpaceViewClassIdentifier {
        "TensorVis".into()
    }
}

type ViewType = TVView;

#[derive(Default, Debug, Clone)]
pub struct TVSpaceViewDrawError(String);

impl std::fmt::Display for TVSpaceViewDrawError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TVSpaceViewDrawError: {}", self.0)
    }
}

impl Error for TVSpaceViewDrawError {}

#[derive(Default)]
pub struct TVSpaceView;

impl SpaceViewClass for TVSpaceView {
    fn identifier() -> re_types::SpaceViewClassIdentifier
    where
        Self: Sized,
    {
        ViewType::identifier()
    }

    fn display_name(&self) -> &'static str {
        "TVSpaceView"
    }

    fn help_text(&self, egui_ctx: &egui::Context) -> egui::WidgetText {
        let mut layout = re_ui::LayoutJobBuilder::new(egui_ctx);

        layout.add("Pan by dragging, or scroll (+ ");
        layout.add(controls::HORIZONTAL_SCROLL_MODIFIER);
        layout.add(" for horizontal).\n");

        layout.add("Zoom with pinch gesture or scroll + ");
        layout.add(controls::ZOOM_SCROLL_MODIFIER);
        layout.add(".\n");

        layout.add("Scroll + ");
        layout.add(controls::ASPECT_SCROLL_MODIFIER);
        layout.add(" to zoom only the temporal axis while holding the y-range fixed.\n");

        layout.add("Drag ");
        layout.add(controls::SELECTION_RECT_ZOOM_BUTTON);
        layout.add(" to zoom in/out using a selection.\n");

        layout.add_button_text(controls::RESET_VIEW_BUTTON_TEXT);
        layout.add(" to reset the view.\n");

        layout.add(egui::Modifiers {
            shift: true,
            ..Default::default()
        });
        layout.add("+ ");
        layout.add(egui::PointerButton::Primary);
        layout.add(" to set the timeline cursor.\n");

        layout.add(egui::Modifiers {
            shift: true,
            ..Default::default()
        });
        layout.add("+ ");
        layout.add(egui::PointerButton::Secondary);
        layout.add(" to set a secondary timeline marker.\n");

        layout.add(egui::Modifiers {
            ctrl: true,
            ..Default::default()
        });
        layout.add("+ ");
        layout.add(egui::PointerButton::Primary);
        layout.add(" to select multiple waveforms.\n");

        layout.add(egui::Modifiers {
            ctrl: true,
            ..Default::default()
        });
        layout.add("+ ");
        layout.add(egui::Key::Enter);
        layout.add(" to toggle selected mode once waveforms have been selected.\n");

        layout.layout_job.into()
    }

    fn on_register(
        &self,
        system_registry: &mut re_viewer_context::SpaceViewSystemRegistrator<'_>,
    ) -> Result<(), re_viewer_context::SpaceViewClassRegistryError> {
        system_registry.register_visualizer::<TVSystem>()?;
        system_registry.register_visualizer::<LTVSystem>()?;

        Ok(())
    }

    fn new_state(&self) -> Box<dyn SpaceViewState> {
        let mut state = Box::<TVSpaceViewState>::default();
        tera_math::register_f64_math_functions(&mut state.tera);
        tera_math::register_f64_math_filters(&mut state.tera);
        state
    }

    fn layout_priority(&self) -> re_viewer_context::SpaceViewClassLayoutPriority {
        re_viewer_context::SpaceViewClassLayoutPriority::High
    }

    fn spawn_heuristics(
        &self,
        ctx: &re_viewer_context::ViewerContext<'_>,
    ) -> re_viewer_context::SpaceViewSpawnHeuristics {
        SpaceViewSpawnHeuristics::root()
        //re_space_view::suggest_space_view_for_each_entity::<TVSystem>(ctx, self)
    }

    fn ui(
        &self,
        ctx: &re_viewer_context::ViewerContext<'_>,
        ui: &mut egui::Ui,
        state: &mut dyn SpaceViewState,
        query: &re_viewer_context::ViewQuery<'_>,
        system_output: re_viewer_context::SystemExecutionOutput,
    ) -> Result<(), re_viewer_context::SpaceViewSystemExecutionError> {
        egui_extras::install_image_loaders(&ctx.egui_ctx);

        let TVSpaceViewState { reset_done, tera } = state.downcast_mut::<TVSpaceViewState>()?;

        //Global inputs
        let (mut current_timeline_marker, time_type, timeline) = {
            // Avoid holding the lock for long
            let time_ctrl = ctx.rec_cfg.time_ctrl.read();
            let current_timeline_marker = time_ctrl.time_i64();
            let time_type = time_ctrl.time_type();
            let timeline = *time_ctrl.timeline();
            (current_timeline_marker, time_type, timeline)
        };

        let mut reset_view = ui.input(|i| {
            i.pointer
                .button_double_clicked(egui::PointerButton::Primary)
        }) || !(*reset_done);

        *reset_done = true;

        let ctrl_pressed = ui.ctx().input(|i| i.modifiers.ctrl);
        let shift_pressed = ui.ctx().input(|i| i.modifiers.shift);
        let space_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::Space));

        // Global effects from inputs

        let mut hovered_entity_paths = HashSet::new();
        let mut selected_entity_paths = HashSet::new();

        ctx.hovered().iter().for_each(|(item, _item_space_ctx)| {
            if let Some(entity_path) = item.entity_path() {
                hovered_entity_paths.insert(entity_path.clone());
            }
        });

        ctx.selection().iter().for_each(|(item, _item_space_ctx)| {
            if let Some(entity_path) = item.entity_path() {
                selected_entity_paths.insert(entity_path.clone());
            }
        });

        let TVSystem { template } = system_output.view_systems.get::<TVSystem>()?;
        let LTVSystem { context } = system_output.view_systems.get::<LTVSystem>()?;

        let render_str = tera.render_str(template, context);

        match render_str {
            Ok(render_str) => {
                let bytes: Vec<_> = render_str.bytes().collect();

                let current_timeline_marker = current_timeline_marker.unwrap();
                ui.add(
                    egui::widgets::Image::from_bytes(
                        format!("bytes://{}.svg", current_timeline_marker),
                        bytes,
                    )
                    .fit_to_exact_size(ui.available_size()),
                );
            }
            Err(e) => {
                ui.vertical_centered(|ui| {
                    ui.label(format!("Error: {}: {:?}", e, e.source()));
                    ui.label(format!("Context: {:?}", context));
                });
            }
        }

        Ok(())
    }
}
