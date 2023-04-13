use client::App;

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    #[cfg(feature = "ssr")]
    yew::Renderer::<App>::new().hydrate();
    #[cfg(not(feature = "ssr"))]
    yew::Renderer::<App>::new().render();
}