use gloo_console::log;
use material_yew::MatButton;
use yew::prelude::*;
use yew_clay::button::ButtonGroup;
use yew_clay::button::{ClayButton, DisplayType};
use yew_clay::icon::ClayIcon;
use yew_dom_attributes::aria_attributes::AriaAttributeReceiver;
use yew_dom_attributes::aria_attributes::AriaAttributes;
use yew_dom_attributes::button_html_attributes::ButtonHtmlAttributes;
use yew_dom_attributes::events::EventPropsReceiver;
use yew_dom_attributes::events::MouseEvents;
use yew_dom_attributes::misc_attributes::CustomAttributeReceiver;
use yew_dom_attributes::props::button_props::ButtonProps;

enum Msg {
    AddOne,
    SetDisabled(bool),
}

struct Model {
    value: i64,
    btn_disabled: bool,
    button_props: ButtonProps,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut button_props = ButtonProps::new();
        button_props.add_aria_attribute(AriaAttributes::AriaAtomic(true));

        Self {
            value: 0,
            btn_disabled: false,
            button_props: button_props,
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
                self.button_props
                    .add_btn_attribute(ButtonHtmlAttributes::Disabled);
                self.btn_disabled = is_disabled;
                self.button_props
                    .add_attribute("my-custom-attr".into(), "lalalala".into());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let is_disabled = self.btn_disabled.to_owned();

        let callback: Callback<MouseEvent> = link.callback(move |_ev| {
            log!("something");
            Msg::SetDisabled(!is_disabled)
        });

        let mut button_props = self.button_props.clone();
        button_props.add_mouse_event_listener(MouseEvents::Click(callback));

        let spritemap = "https://cdn.jsdelivr.net/npm/@clayui/css/lib/images/icons/icons.svg";

        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p data-key={"something"} > { self.value }</p>
                <MatButton label="Click me!"  />
                <ButtonGroup spaced={true} class={"stupid-class"}>
                    <ClayButton
                       // misc_attrs={self.btn_1_misc_attrs.clone()}

                        display_type={DisplayType::Info}
                        // onclick={on_click}
                        button_html_attributes={Some(button_props)}
                        //aria={aria}
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
