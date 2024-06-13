use crate::*;
use gloo_console::log;
use patternfly_yew::prelude::{
    ExpandableSection, PageSection, PageSectionSticky, PageSectionType, PageSectionVariant,
};

#[derive(Properties, Clone, PartialEq, serde::Deserialize, Debug)]
pub struct QuestionStruct {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub tags: Option<HashSet<String>>,
    pub answer: Option<String>,
}

impl QuestionStruct {
    pub async fn get_question(key: Option<String>) -> Msg {
        log!("question");
        let request = match &key {
            None => "http://localhost:3000/api/v1/questions".to_string(),
            Some(ref key) => format!("http://localhost:3000/api/v1/questions/{}", key,),
        };
        log!("q2");
        let response = http::Request::get(&request).send().await;
        match response {
            Err(e) => {
                log!("error");
                Msg::GotQuestion(Err(e))
            }
            Ok(data) => {
                log!("OK");
                Msg::GotQuestion(data.json().await)
            }
        }
    }
}

pub fn format_tags(tags: &HashSet<String>) -> String {
    let taglist: Vec<&str> = tags.iter().map(String::as_ref).collect();
    taglist.join(", ")
}

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QuestionProps {
    pub question: Vec<QuestionStruct>,
}

#[function_component(Question)]
pub fn question(questions: &QuestionProps) -> Html {
    let vnodes = questions.question.iter().map(|question| {
        html! {
        <>
            <PageSection>
            <PageSection
                r#type={PageSectionType::Default}
                variant={PageSectionVariant::Light}
                limit_width=true
                sticky={[PageSectionSticky::Top]}
            >

                <ExpandableSection>
                    <div class="question">
                        <span class="teller">{"Question:"}</span><br/>
                        <span class="tellee">{format!("{}", &question.title)}</span><br/>
                        <span class="tellee">{format!("{}", &question.content)}</span><br/><br/>
                        <span class="tellee">{"Answer:"}</span><br/>
                        <span class="tellee">{match &question.answer { Some(res) => { format!("{}.", res) }, None => { "No Answer yet.".to_string() } }}</span><br/>
                    </div>
                    <span class="annotation">
                        {format!("[id: {}", &question.id)}
                        if let Some(ref tags) = question.tags {
                            {format!("; tags: {}", &format_tags(tags))}
                        }
                        {"]"}
                    </span>
                </ExpandableSection>
            </PageSection>
            </PageSection>
        </>
        }
    }).collect::<Vec<_>>();

    vnodes
        .into_iter()
        .fold(html! { <></> }, |accumulator, node| {
            html! {
            <>
                { accumulator }
                { node }
            </>
            }
        })
}
