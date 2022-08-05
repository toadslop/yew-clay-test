use std::rc::Rc;

use gloo_console::log;
use material_yew::MatButton;
use yew::prelude::*;
use yew_clay::button::ButtonGroup;
use yew_clay::button::{ClayButton, DisplayType};
use yew_clay::icon::ClayIcon;
use yew_dom_attributes::attributes::aria_attributes::AriaAttributes;
use yew_dom_attributes::attributes::button_html_attributes::ButtonHtmlAttributes;
use yew_dom_attributes::events::events::{EventType, MouseEvents};
use yew_dom_attributes::props::aria_props::AriaPropsHandler;
use yew_dom_attributes::props::button_props::{ButtonProps2, ButtonPropsHandler};
use yew_dom_attributes::props::custom_attributes::{CustomAttribute, CustomPropsHandler};
use yew_dom_attributes::props::DomInjector;

pub enum Msg {
    AddOne,
    SetDisabled(bool),
    RemoveListener(String),
    UpdateButtonProps(Rc<ButtonProps2>),
}

struct Model {
    value: i64,
    btn_disabled: bool,
    button_props: Rc<ButtonProps2>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let on_btn_props_update: Callback<Rc<ButtonProps2>> =
            ctx.link().callback(move |btn_props: Rc<ButtonProps2>| {
                gloo_console::log!("CALLBACK EMITTED");
                Msg::UpdateButtonProps(btn_props)
            });
        let mut button_props = Rc::new(ButtonProps2::new(on_btn_props_update));
        Rc::make_mut(&mut button_props).add_aria_prop(AriaAttributes::AriaAtomic(true));
        gloo_console::log!("create ran");

        let callback: Callback<MouseEvent> = ctx.link().callback(move |_ev| {
            log!("something");
            Msg::RemoveListener("click-event".into())
        });
        Rc::make_mut(&mut button_props).add_listener(
            "click-event".into(),
            EventType::MouseEvent(MouseEvents::Click(callback)),
        );

        Self {
            value: 0,
            btn_disabled: false,
            button_props,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::SetDisabled(is_disabled) => {
                log!("set disabled ran");
                Rc::make_mut(&mut self.button_props)
                    .add_button_prop(ButtonHtmlAttributes::Disabled);
                self.btn_disabled = is_disabled;
                Rc::make_mut(&mut self.button_props).add_custom_prop(CustomAttribute::new(
                    "my-custom-attribute".into(),
                    "lalalala".into(),
                ));
                true
            }
            Msg::RemoveListener(key) => {
                gloo_console::log!("REMOVING IT");
                Rc::make_mut(&mut self.button_props).remove_listener(key);
                true
            }
            Msg::UpdateButtonProps(new_props) => {
                gloo_console::log!("update props");
                self.button_props = new_props;
                false
            }
        }
    }

    // fn changed(&mut self, ctx: &Context<Self>) -> bool {}

    fn view(&self, ctx: &Context<Self>) -> Html {
        gloo_console::log!("view ran");
        let link = ctx.link();
        let is_disabled = self.btn_disabled.to_owned();

        let spritemap = "https://cdn.jsdelivr.net/npm/@clayui/css/lib/images/icons/icons.svg";

        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p data-key={"something"} > { self.value }</p>
                <MatButton label="Click me!"  />
                <ButtonGroup spaced={true} class={"stupid-class"}>
                    <ClayButton
                        display_type={DisplayType::Info}
                        button_html_attributes={Some(self.button_props.clone())}
                        >
                    {"Click Me"}
                    </ClayButton>
                    <ClayButton
                        display_type={DisplayType::Warning}
                        >
                    {"Other Button"}
                    </ClayButton>

                </ButtonGroup>
                <ClayIcon spritemap={spritemap} symbol={"add-cell"}></ClayIcon>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
