use domatt::attributes::aria::AriaAtomic;
use domatt::attributes::button::ButtonTypeOption;
use domatt::attributes::button::Disabled;
use domatt::attributes::button::Type;
use domatt::attributes::global::CustomAttribute;
use domatt::attributes::Attribute;
use domatt::events::Click;
use std::rc::Rc;
use web_sys::MouseEvent;
use yew::{html, Callback, Component, Context, Html};
use yew_clay::{
    ButtonDisplayType, ClayButton, ClayButtonGroup, ClayButtonProps, ClayButtonWithIcon,
};
use yew_dom_attributes::button_props::ButtonProps;
use yew_dom_attributes::DomInjector;

pub enum Msg {
    ToggleDisabled,
    RemoveListener(String),
}

struct Model {
    btn_disabled: bool,
    button_primary_props: ButtonProps,
    button_warning_props: ButtonProps,
    icon_button_props: ClayButtonProps,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut button_primary_props = ButtonProps::new();

        button_primary_props.add_attribute(Box::new(AriaAtomic::new(true)));
        button_primary_props.add_attribute(Box::new(Type::new(ButtonTypeOption::Submit)));

        let remove_listener_cb: Callback<MouseEvent> = ctx.link().callback(move |_ev| {
            gloo_console::log!("KKKKK");
            Msg::RemoveListener("click-event".into())
        });

        let button_warning_props = ButtonProps::new();

        let callback: Callback<MouseEvent> = ctx.link().callback(move |_ev| Msg::ToggleDisabled);

        button_primary_props.add_listener("my-disabled", Rc::new(Click::from(callback)));
        button_primary_props.add_listener("click-event", Rc::new(Click::from(remove_listener_cb)));

        let icon_button_props = ClayButtonProps {
            borderless: true,
            ..Default::default()
        };

        Self {
            btn_disabled: false,
            button_primary_props,
            button_warning_props,
            icon_button_props,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleDisabled => {
                self.btn_disabled = !self.btn_disabled;
                if self.btn_disabled {
                    self.button_warning_props.add_attribute(Box::new(Disabled));
                } else {
                    self.button_warning_props
                        .remove_attribute(Disabled.get_key());
                }
                let my_attribute = CustomAttribute::new("my-custom-attribute", Some("lalalala"));
                self.button_warning_props
                    .add_attribute(Box::new(my_attribute));
                true
            }
            Msg::RemoveListener(key) => {
                self.button_primary_props.remove_listener(key);
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let spritemap = "static/icons.svg";

        html! {
            <div>
                <h1>{ "Yew Component Library With Dynamic Props Demo" }</h1>
                <ClayButtonGroup spaced={true} class={"stupid-class"}>
                    <ClayButton
                        display_type={ButtonDisplayType::Info}
                        button_html_attributes={Some(self.button_primary_props.clone())}
                        >
                    {"Click Me"}
                    </ClayButton>
                    <ClayButton
                        display_type={ButtonDisplayType::Warning}
                        button_html_attributes={Some(self.button_warning_props.clone())}
                         >
                    {"Other Button"}
                    </ClayButton>
                    <ClayButtonWithIcon clay_button_props={self.icon_button_props.clone()} spritemap={spritemap} symbol={"add-cell"} />
                </ClayButtonGroup>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
