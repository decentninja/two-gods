#![recursion_limit = "256"]

use std::cell::Cell;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

struct Model {
    paragraphs: Vec<Item>,
    answers: Vec<Answer>,
}

enum Item {
    Paragraph(Vec<ParagraphParts>),
    SpotifyEmbed(String),
}

#[derive(Clone, PartialEq)]
enum Answer {
    Hidden,
    Showing { answer: Option<bool> },
}

enum Msg {
    Show(usize),
    Answer(usize, bool),
}

enum ParagraphParts {
    Text(&'static str),
    Answer(usize, &'static str),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let source = include_str!("../story.txt");
        let n_answers = Cell::new(0); // What? Don't look at me like that!
        let paragraphs = source
            .split("\n\n")
            .map(|text| {
                let spotify = "https://open.spotify.com";
                if text.starts_with(spotify) {
                    let mut link = text.to_owned();
                    link.insert_str(spotify.len(), "/embed");
                    Item::SpotifyEmbed(link)
                } else {
                    Item::Paragraph(
                        text.split(|c| c == '[' || c == ']')
                            .enumerate()
                            .map(|(i, part)| {
                                if i % 2 == 0 {
                                    ParagraphParts::Text(part)
                                } else {
                                    let par = ParagraphParts::Answer(n_answers.get(), part);
                                    n_answers.set(n_answers.get() + 1);
                                    par
                                }
                            })
                            .collect(),
                    )
                }
            })
            .collect();
        Model {
            paragraphs,
            answers: vec![Answer::Hidden; n_answers.get()],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Show(index) => self.answers[index] = Answer::Showing { answer: None },
            Msg::Answer(index, answer) => {
                self.answers[index] = Answer::Showing {
                    answer: Some(answer),
                };
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <title>{ "Two Gods" }</title>
                <link rel="stylesheet" href="gutenberg.css"></link>
                <link rel="stylesheet" href="styles.css"></link>
                <h1>{ "Two Gods" }</h1>
                { for self.paragraphs.iter().map(|item| match item {
                    Item::Paragraph(paragraph) => html! {
                        <p>{ for paragraph.iter().map(|part| self.render_part(part)) }</p>
                    },
                    Item::SpotifyEmbed(link) => html! {
            <iframe src={ link } width="300" height="180" frameborder="0" allowtransparency="true" allow="encrypted-media"></iframe>
                    }
                  }) }
                <p>{format!("How many did you get?: {}/{}", self.correct(), self.total()) }</p>
                <p>{"GG, see you 2019-07-23 18:00!."}</p>
            </div>
        }
    }
}

impl Model {
    fn total(&self) -> usize {
        self.answers.len()
    }

    fn correct(&self) -> usize {
        self.answers
            .iter()
            .filter(|a| a == &&Answer::Showing { answer: Some(true) })
            .count()
    }

    fn render_part(&self, part: &ParagraphParts) -> Html<Self> {
        match part {
            ParagraphParts::Text(t) => html! { <span>{ t }</span> },
            ParagraphParts::Answer(i, answer) => {
                let i = *i;
                match self.answers[i] {
                    Answer::Hidden => html! {
                        <span class="hidden" onclick=|_| Msg::Show(i)>
                            { answer }
                        </span>
                    },
                    Answer::Showing { answer: result } => {
                        let yes = if result == Some(true) { "yes" } else { "" };
                        let no = if result == Some(false) { "no" } else { "" };
                        html! {
                            <span>
                                <span>{ answer }</span>
                                <button class={ yes } onclick=|_| Msg::Answer(i, true)>{ "✓" }</button>
                                <button class={ no } onclick=|_| Msg::Answer(i, false)>{ "✗" }</button>
                            </span>
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
