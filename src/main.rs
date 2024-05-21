#![allow(non_snake_case)]

use dioxus::prelude::*;
use std::collections::HashMap;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/database_connect")]
    DatabaseConnect {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let mut values: Signal<HashMap<String, FormValue>> = use_signal(HashMap::new);
    rsx! {
        div {
            class:"flex",
            div{
                class:"w-1/2 h-auto overflow-scroll block h-screen bg-gradient-to-r from-blue-100 via-purple-100 to-pink-100 p-4 flex items-center justify-center",
                div{
                    class:"bg-white py-6 px-10 sm:max-w-md w-full",
                    div{
                        class:"sm:text-3xl text-2xl font-semibold text-center text-sky-600  mb-12",
                        "Database Values"
                    },
                    form{
                        action:"database_connect",
                        method:"POST",
                        oninput: move |ev| {
                            println!("Input event: {:#?}", ev);
                            values.set(ev.values());
                        },
                        /*
                        * SUBMIT and Route(?)
                        onsubmit: move |ev| {
                            println!("Submit event: {:#?}", ev);
                            submitted_values.set(ev.values());
                        },
                        */
                        div {
                            class:"",
                            div{
                                input{
                                    r#type:"text",
                                    class:"focus:outline-none border-b w-full pb-2 border-sky-400 placeholder-gray-500",
                                    placeholder:"Username ",
                                    name: "username"
                                }
                            },
                            div{
                                input{
                                    r#type:"password",
                                    class:"focus:outline-none border-b w-full pb-2 border-sky-400 placeholder-gray-500 my-8",
                                    placeholder:"Password ",
                                    name: "password"
                                }
                            },
                            div{
                                input{
                                    r#type:"text",
                                    class:"focus:outline-none border-b w-full pb-2 border-sky-400 placeholder-gray-500 mb-8",
                                    placeholder:"Hostname or IP adress",
                                    name: "hostname"
                                }
                            },
                            div{
                                input{
                                    r#type:"number",
                                    class:"focus:outline-none border-b w-full pb-2 border-sky-400 placeholder-gray-500 mb-8",
                                    placeholder:"Port ",
                                    name: "port"
                                }
                            },
                            div{
                                input{
                                    r#type:"text",
                                    class:"focus:outline-none border-b w-full pb-2 border-sky-400 placeholder-gray-500 mb-8",
                                    placeholder:"Database ",
                                    name: "database"
                                }
                            },
                            div{
                                class:"flex justify-center my-6",
                                button{
                                    r#type:"submit",
                                    class:"rounded-full  p-3 w-full sm:w-56   bg-gradient-to-r from-sky-600  to-teal-300 text-white text-lg font-semibold",
                                    "Try to Connect"
                                }
                            }
                        }
                    }
                }
            },
            div {
                class:"w-1/2",
                h1 { "Oninput Values" },
                pre { "{values:#?}" }
            }
        }
    }
}

#[component]
fn DatabaseConnect() -> Element {
    rsx! {}
}
