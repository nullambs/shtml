use pest::{
    Parser,
    iterators::Pairs,
};
use pest_derive::Parser;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement, window};

#[derive(Debug)]
enum Elements<'a> {
    Tag {
        name: &'a str,
        id: Option<&'a str>,
        class: Vec<&'a str>,
        children: Vec<Elements<'a>>,
    },
    Text {
        value: &'a str,
    },
}

#[derive(Parser)]
#[grammar = "shtml.pest"]
struct ShtmlParser;

fn process_tag<'a>(pairs: &mut Pairs<'a, Rule>) -> Elements<'a> {
    let mut name: &str = "";
    let mut id: Option<&str> = None;
    let mut class: Vec<&str> = Vec::new();
    let mut children: Vec<Elements> = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::tag_name => name = pair.as_str(),
            Rule::tag_id => id = Some(pair.as_str()),
            Rule::tag_class => class.push(pair.as_str()),
            Rule::inner_html => {
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::tag => children.push(process_tag(&mut inner.into_inner())),
                        Rule::inner_text => children.push(Elements::Text {
                            value: inner.as_str(),
                        }),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Elements::Tag {
        name,
        id,
        class,
        children,
    }
}

fn process_document<'a>(pairs: &mut Pairs<'a, Rule>) -> Vec<Elements<'a>> {
    let mut result = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::tag => result.push(process_tag(&mut pair.into_inner())),
            _ => {}
        }
    }

    return result;
}
fn create_elements(tree: &Elements, attach_to: &Element) {
    let window = window().expect("Could not resolve window");
    let document = window.document().expect("Could not resolve document");

    match tree {
        Elements::Tag { name, id, class, children } => {
            let mut element = document.create_element(name.trim()).expect("Failed to create element");
            class.iter().for_each(|c| {
                element.set_class_name(format!("{} {}", element.class_name(), c).as_str());
            });
            
            match id {
                Some(v) => element.set_id(v),
                None => {}
            }
            
            children.iter().for_each(|c| {
                create_elements(c, &mut element);
            });

            attach_to.append_child(&element).expect("Failed to attach child");
        },
        Elements::Text { value } => {
            let element = document.create_text_node(value);
            attach_to.append_child(&element).expect("Failed to attach text node");
        }
    }
}

#[wasm_bindgen]
pub fn parse(shtml: &str, attach_to: &Element) {
    let mut pairs = ShtmlParser::parse(Rule::tag, shtml).unwrap_or_else(|e| panic!("{}", e));
    let elements = process_document(&mut pairs);

    for el in &elements {
        create_elements(&el, &attach_to);
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let window = window().expect("Could not resolve window");
    let document = window.document().expect("Could not resolve document");

    let nodes = document.query_selector_all("[shtml]").expect("Failed to query shtml nodes");
    for x in 0..nodes.length() {
        let node = nodes.item(x).expect("Failed to iterate through shtml nodes");
        if let Some(element) = node.dyn_ref::<HtmlElement>() {
            let inner_text = element.inner_text();
            element.set_inner_html("");
            parse(&inner_text, &element);
        }
    }
}

