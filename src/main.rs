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
use yew_clay::{ClayButton, ClayButtonGroup, ClayButtonProps, ClayButtonWithIcon, DisplayType};
use yew_dom_attributes::button_props::ButtonProps;
use yew_dom_attributes::DomInjector;

pub enum Msg {
    ToggleDisabled,
    RemoveListener(String),
    UpdateBtnPrimaryProps(Rc<ButtonProps>),
    UpdateBtnWarningProps(Rc<ButtonProps>),
}

struct Model {
    btn_disabled: bool,
    button_primary_props: Rc<ButtonProps>,
    button_warning_props: Rc<ButtonProps>,
    icon_button_props: ClayButtonProps,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let update_func = |btn_props: Rc<ButtonProps>| Msg::UpdateBtnPrimaryProps(btn_props);
        let mut button_primary_props = ButtonProps::with_update_callback(ctx, update_func);

        button_primary_props.add_attribute(Box::new(AriaAtomic::new(true)));
        button_primary_props.add_attribute(Box::new(Type::new(ButtonTypeOption::Submit)));

        let remove_listener_cb: Callback<MouseEvent> = ctx
            .link()
            .callback(move |_ev| Msg::RemoveListener("click-event".into()));

        button_primary_props.add_listener("click-event", Rc::new(Click::from(remove_listener_cb)));

        let btn_warning_update_func =
            |btn_props: Rc<ButtonProps>| Msg::UpdateBtnWarningProps(btn_props);

        let button_warning_props = ButtonProps::with_update_callback(ctx, btn_warning_update_func);

        let callback: Callback<MouseEvent> = ctx.link().callback(move |_ev| Msg::ToggleDisabled);

        button_primary_props.add_listener("my-disabled", Rc::new(Click::from(callback)));

        let icon_button_props = ClayButtonProps {
            borderless: true,
            ..Default::default()
        };

        Self {
            btn_disabled: false,
            button_primary_props: Rc::new(button_primary_props),
            button_warning_props: Rc::new(button_warning_props),
            icon_button_props,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleDisabled => {
                self.btn_disabled = !self.btn_disabled;

                if self.btn_disabled {
                    Rc::make_mut(&mut self.button_warning_props).add_attribute(Box::new(Disabled));
                } else {
                    Rc::make_mut(&mut self.button_warning_props)
                        .remove_attribute(Disabled.get_key());
                }
                let my_attribute = CustomAttribute::new("my-custom-attribute", Some("lalalala"));
                Rc::make_mut(&mut self.button_warning_props).add_attribute(Box::new(my_attribute));
                true
            }
            Msg::RemoveListener(key) => {
                Rc::make_mut(&mut self.button_primary_props).remove_listener(key);
                true
            }
            Msg::UpdateBtnPrimaryProps(new_props) => {
                self.button_primary_props = new_props;
                false
            }
            Msg::UpdateBtnWarningProps(new_props) => {
                self.button_warning_props = new_props;
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
                        display_type={DisplayType::Info}
                        button_html_attributes={Some(self.button_primary_props.clone())}
                        >
                    {"Click Me"}
                    </ClayButton>
                    <ClayButton
                        display_type={DisplayType::Warning}
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
