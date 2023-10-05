use dioxus::prelude::*;
use dioxus_signals::Signal;
use uuid::Uuid;
use crate::data::*;
use crate::colours::*;

fn text_colour_from_bg((r, g, b): Rgb) -> Colour {
  if (u16::from(r) + u16::from(g) + u16::from(b)) >= (255 * 3 / 2) {
    Colour::Colour((0, 0, 0))
  } else {
    Colour::Colour((255, 255, 255))
  }
}

#[component]
pub fn MessageBox(cx: Scope, msgs: Signal<Messages>) -> Element {
    cx.render(rsx! {
        div { class: "flex flex-col flex-grow border rounded-xl p-4 w-full max-w-2xl gap-2 overflow-y-scroll",
            for (i , msg) in msgs.read().msgs.iter().enumerate() {
                // PersonaMessage { msg: msg.msg.clone(), persona: msg.persona.clone() }
                div { key: "{msg.uuid}", class: if i == 0 { "flex-col gap-2 mt-auto" } else { "flex-col gap-2" },
                    if i == 0 || !msg.persona.eq(&msgs.read().msgs.get(i-1).unwrap().persona) {
                        rsx! {
                            div { 
                            class: "flex items-center",
                            PersonaIcon { colour: msg.persona.colour }
                            span { "{msg.persona.name}" }
                            }
                        }
                    }
                    div {
                        class: "rounded-lg px-2 py-1 w-fit",
                        style: "{Colour::BgColour(msg.persona.colour)} {text_colour_from_bg(msg.persona.colour)}",
                        onmounted: move |cx2| {
                            cx2.inner().scroll_to(ScrollBehavior::Smooth);
                        },
                        span { "{msg.msg}" }
                    }
                }
            }
        }
    })
}

#[component]
pub fn PersonaButton<'a>(
    cx: Scope,
    name: String,
    colour: Rgb,
    onclick: EventHandler<'a, MouseEvent>,
) -> Element {
    cx.render(rsx! {
        div { class: "flex flex-col items-center w-auto h-auto leading-none",
            button { onclick: move |evt| onclick.call(evt), PersonaIcon { colour: *colour } }
            p { class: "text-xs whitespace-nowrap", "{name}" }
        }
    })
}

pub fn PersonaMessage(cx: Scope<Message>) -> Element {
let text_colour = {
  let (r, g, b) = cx.props.persona.colour;
  if (u16::from(r) + u16::from(g) + u16::from(b)) >= (255 * 3 / 2) {
    Colour::Colour((0, 0, 0))
  } else {
    Colour::Colour((255, 255, 255))
  }
};
  cx.render(rsx! {
    div { class: "flex-col gap-2",
        div { class: "flex items-center",
            PersonaIcon { colour: cx.props.persona.colour }
            span { "{cx.props.persona.name}" }
        }
        div {
            class: "rounded-lg px-2 py-1 w-fit",
            style: r"
              {Colour::BgColour(cx.props.persona.colour)};
              {text_colour};",
            span { "{cx.props.msg}" }
        }
    }
})
}

#[component]
fn PersonaIcon(cx: Scope, colour: Rgb) -> Element {
    cx.render(rsx! {
        div {
            svg {
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "currentColor",
                class: "w-9 h-9",
                style: "{Colour::Colour(*colour)}",
                path {
                    clip_rule: "evenodd",
                    fill_rule: "evenodd",
                    d: "M4.848 2.771A49.144 49.144 0 0112 2.25c2.43 0 4.817.178 7.152.52 1.978.292 3.348 2.024 3.348 3.97v6.02c0 1.946-1.37 3.678-3.348 3.97a48.901 48.901 0 01-3.476.383.39.39 0 00-.297.17l-2.755 4.133a.75.75 0 01-1.248 0l-2.755-4.133a.39.39 0 00-.297-.17 48.9 48.9 0 01-3.476-.384c-1.978-.29-3.348-2.024-3.348-3.97V6.741c0-1.946 1.37-3.68 3.348-3.97z"
                }
            }
        }
    })
}


#[component]
pub fn AddPersonaButton<'a>(cx: Scope, onclick: EventHandler<'a, MouseEvent>) -> Element {
    cx.render(rsx! {
        div { class: "flex flex-col items-center text-black justify-end w-auto h-auto leading-none mr-3",
            button { class: "leading-none", onclick: |evt| onclick.call(evt), AddPersonaIcon {} }
            span { class: "text-xs", "Add" }
        }
    })
}

fn AddPersonaIcon(cx: Scope) -> Element {
    cx.render(rsx! {
        svg { class: "w-9 h-9", view_box: "0 0 24 24",
            svg {
                view_box: "0 0 24 24",
                stroke: "currentColor",
                stroke_width: "1.5",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                class: "w-6 h-6",
                path {
                    d: "M2.25 12.76c0 1.6 1.123 2.994 2.707 3.227 1.068.157 2.148.279 3.238.364.466.037.893.281 1.153.671L12 21l2.652-3.978c.26-.39.687-.634 1.153-.67 1.09-.086 2.17-.208 3.238-.365 1.584-.233 2.707-1.626 2.707-3.228V6.741c0-1.602-1.123-2.995-2.707-3.228A48.394 48.394 0 0012 3c-2.392 0-4.744.175-7.043.513C3.373 3.746 2.25 5.14 2.25 6.741v6.018z",
                    stroke_linecap: "round",
                    stroke_linejoin: "round"
                }
            }
            svg {
                view_box: "0 0 20 20",
                y: "-1.5",
                fill: "currentColor",
                xmlns: "http://www.w3.org/2000/svg",
                class: "w-5 h-5",
                path { d: "M10.75 6.75a.75.75 0 00-1.5 0v2.5h-2.5a.75.75 0 000 1.5h2.5v2.5a.75.75 0 001.5 0v-2.5h2.5a.75.75 0 000-1.5h-2.5v-2.5z" }
            }
        }
    })
}

pub fn SendIcon(cx: Scope) -> Element {
  cx.render(rsx! {
    svg {
        xmlns: "http://www.w3.org/2000/svg",
        fill: "none",
        view_box: "0 0 24 24",
        stroke_width: "1.5",
        stroke: "currentColor",
        class: "w-8 h-8",
        path {
            stroke_linecap: "round",
            stroke_linejoin: "round",
            d: "M6 12L3.269 3.126A59.768 59.768 0 0121.485 12 59.77 59.77 0 013.27 20.876L5.999 12zm0 0h7.5"
        }
    }
})
}
