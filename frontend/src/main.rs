mod finder;
mod question;

use finder::*;
use gloo_console::log;
use question::*;

use patternfly_yew::prelude::Pagination;
use std::collections::HashSet;

extern crate serde;
use gloo_net::http;
extern crate wasm_bindgen_futures;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

pub type QuestionResult = Result<Vec<QuestionStruct>, gloo_net::Error>;

struct App {
    question: QuestionResult,
}

pub enum Msg {
    GotQuestion(QuestionResult),
    GetQuestion(Option<String>),
}

impl App {
    fn refresh_question(ctx: &Context<Self>, key: Option<String>) {
        log!("create");
        let got_question = QuestionStruct::get_question(key);
        ctx.link().send_future(got_question);
        log!("create2");
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        App::refresh_question(ctx, None);
        let question = Err(gloo_net::Error::GlooError("Loading Question…".to_string()));
        Self { question }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        log!("Update");
        match msg {
            Msg::GotQuestion(question) => {
                self.question = question;
                true
            }
            Msg::GetQuestion(key) => {
                App::refresh_question(ctx, key);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log!("view");
        let question = &self.question;
        log!("view2");
        html! {
        <>
            <h1>{ "Questions Unlimited" }</h1>

            if let Ok(question_res) = question {
                <div>
                    <Question question={question_res.clone()}/>
                </div>
            } else if let Err(ref error) = question {
                <div>
                    <span class="error">{format!("Server Error: {error}")}</span>
                </div>
            }
            <Finder on_find={ctx.link().callback(Msg::GetQuestion)}/>
            <Pagination/>
        </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
