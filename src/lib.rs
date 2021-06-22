// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports, clippy::non_ascii_literal)]

use seed::{prelude::*, *};
use wacolor::WaColor;

mod wacolor;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        view_color: WaColor::random_color(),
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    view_color: WaColor,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    RandomClicked,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::RandomClicked => {
            model.view_color = WaColor::random_color();
        }
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    // log!(model.view_color.luminance());

    div![
        C!["main-container"],
        style! {St::Background => model.view_color.color_code()},
        div![
            id!["operation-box"],
            button![
                "無作為選択",
                id!["random-button"],
                ev(Ev::Click, |_| Msg::RandomClicked),
            ],
        ],
        div![
            id!["color-data"],
            C![if model.view_color.luminance() < 0.6 {
                "light-color"
            } else {
                "dark-color"
            }],
            div![id!["yomigana"], model.view_color.yomi(),],
            div![id!["kanji"], model.view_color.name(),],
            div![
                id!["color-code"],
                model.view_color.color_code().to_uppercase(),
                br!(),
                model.view_color.rgb_code(),
            ],
        ],
        div![
            id!["footer"],
            a![
                attrs! {
                    At::Href => "https://github.com/trisef/wanoiro-seed/",
                    At::Target => "_blank"
                },
                "リポジトリ"
            ]
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
