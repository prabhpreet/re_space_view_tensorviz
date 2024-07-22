use std::f64::consts::PI;

use ndarray::Array;
use re_space_view_tensorviz::types::archetypes::TensorViz;
use re_types::archetypes::Tensor;
use re_viewer::external::{re_log, re_memory};

#[global_allocator]
static GLOBAL: re_memory::AccountingAllocator<mimalloc::MiMalloc> =
    re_memory::AccountingAllocator::new(mimalloc::MiMalloc);

pub struct CustomSink {
    channel: re_smart_channel::Sender<re_sdk::log::LogMsg>,
}

impl CustomSink {
    pub fn new(channel: re_smart_channel::Sender<re_sdk::log::LogMsg>) -> Self {
        Self { channel }
    }
}

impl re_sdk::sink::LogSink for CustomSink {
    fn send(&self, msg: re_sdk::log::LogMsg) {
        self.channel.send(msg).unwrap();
    }

    fn flush_blocking(&self) {
        self.channel.flush_blocking().unwrap();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let svg_file = include_str!("phasor.svg");

    re_log::setup_logging();
    re_crash_handler::install_crash_handlers(re_viewer::build_info());

    let (rec, rx) = re_smart_channel::smart_channel(
        re_smart_channel::SmartMessageSource::Sdk,
        re_smart_channel::SmartChannelSource::Sdk,
    );

    let startup_options = re_viewer::StartupOptions {
        hide_welcome_screen: true,
        ..Default::default()
    };

    let app_env = re_viewer::AppEnvironment::Custom("tensor example".to_string());

    let handle = std::thread::spawn(move || {
        let application_id = "tensor_example".to_string();
        let sink = CustomSink::new(rec);
        let rec = re_sdk::RecordingStream::new(
            re_sdk::new_store_info(application_id),
            re_sdk::log::ChunkBatcherConfig::DEFAULT,
            Box::new(sink),
        )
        .unwrap();

        rec.log_static("/V", &TensorViz::new_svg(svg_file)).unwrap();

        let path = ["/V/A", "/V/B", "/V/C"];
        let phase_offset = [0.0, 120.0, 240.0];
        let label_mag = [1.0, 0.5, 1.25];

        for x in 0..360 {
            for i in 0..3 {
                let mag = label_mag[i];
                let phase = (x as f64 + phase_offset[i]) * PI / 180.0;
                let data = Array::from_shape_vec((1, 2), vec![mag, phase]).unwrap();
                rec.log(
                    path[i],
                    &Tensor::try_from(data)
                        .unwrap()
                        .with_dim_names(["phasor", "data"]),
                )
                .unwrap();
            }
        }
    });

    re_viewer::run_native_app(
        Box::new(move |cc| {
            let mut app = re_viewer::App::new(
                re_viewer::build_info(),
                &app_env,
                startup_options,
                cc.egui_ctx.clone(),
                cc.storage,
            );
            app.add_receiver(rx);

            app.add_space_view_class::<re_space_view_tensorviz::TVSpaceView>()
                .unwrap();

            Box::new(app)
        }),
        None,
    )?;

    handle.join().unwrap();
    Ok(())
}
