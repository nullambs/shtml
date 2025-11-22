use pest::{
    Parser,
    iterators::Pairs,
};
use pest_derive::Parser;
use wasm_bindgen::prelude::*;
use web_sys::{Element, console, HtmlElement, window};

#[derive(Debug)]
struct Attribute<'a> {
    name: Option<&'a str>,
    value: Option<&'a str>,
}

#[derive(Debug)]
enum Nodes<'a> {
    Elements {
        name: &'a str,
        id: Option<&'a str>,
        class: Vec<&'a str>,
        children: Vec<Nodes<'a>>,
        attributes: Vec<Attribute<'a>>,
    },
    Text {
        value: &'a str,
    },
}

#[derive(Parser)]
#[grammar = "shtml.pest"]
struct ShtmlParser;

fn process_tag<'a>(pairs: &mut Pairs<'a, Rule>) -> Nodes<'a> {
    let mut name: &str = "";
    let mut id: Option<&str> = None;
    let mut class: Vec<&str> = Vec::new();
    let mut children: Vec<Nodes> = Vec::new();
    let mut attributes: Vec<Attribute> = Vec::new();

    for element in pairs {
        match element.as_rule() {
            Rule::tag_name => name = element.as_str(),
            Rule::tag_id => id = Some(element.as_str()),
            Rule::tag_class => class.push(element.as_str()),
            Rule::tag_attributes => {
                let element_span = element.as_span().as_str();

                for attribute in element.into_inner() {
                    let attribute_span = attribute.as_span().as_str();
                    let mut attr = Attribute { name: None, value: None };

                    for attr_entry in attribute.into_inner() {
                        match attr_entry.as_rule() {
                            Rule::attribute_name => attr.name = Some(attr_entry.as_str()),
                            Rule::attribute_value => attr.value = Some(attr_entry.as_str()),
                            _ => {} 
                        }
                    }

                    if attr.name.is_none() {
                        console::warn_1(&format!(
                            "Failed to parse attribute {} of {}",
                            attribute_span,
                            element_span).into()
                        );
                        continue;
                    }

                    attributes.push(attr);
                }
            }
            Rule::inner_html => {
                for inner in element.into_inner() {
                    match inner.as_rule() {
                        Rule::tag => children.push(process_tag(&mut inner.into_inner())),
                        Rule::inner_text => children.push(Nodes::Text {
                            value: inner.as_str(),
                        }),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Nodes::Elements {
        name,
        id,
        class,
        children,
        attributes,
    }
}

fn process_document<'a>(pairs: &mut Pairs<'a, Rule>) -> Vec<Nodes<'a>> {
    let mut result = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::tag => result.push(process_tag(&mut pair.into_inner())),
            _ => {}
        }
    }

    return result;
}
fn create_elements(tree: &Nodes, attach_to: &Element) {
    let window = window().expect("Could not resolve window");
    let document = window.document().expect("Could not resolve document");

    match tree {
        Nodes::Elements { name, id, class, children, attributes } => {
            let mut element = document.create_element(name.trim()).expect("Failed to create element");
            element.set_class_name(class.join(" ").as_str());
            for attribute in attributes {
                element
                    .set_attribute(attribute.name.unwrap(), attribute.value.unwrap_or(""))
                    .expect("Failed to set tag attribute");
            }
            
            match id {
                Some(v) => element.set_id(v),
                None => {}
            }
            
            children.iter().for_each(|c| {
                create_elements(c, &mut element);
            });

            attach_to.append_child(&element).expect("Failed to attach child");
        },
        Nodes::Text { value } => {
            let element = document.create_text_node(value);
            attach_to.append_child(&element).expect("Failed to attach text node");
        }
    }
}

#[wasm_bindgen]
pub fn parse(shtml: &str, attach_to: &Element) {
    let mut pairs = ShtmlParser::parse(Rule::tag, shtml)
        .unwrap_or_else(|e| {
            console::error_1(&format!("{}", e).into());
            panic!();
        });
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

