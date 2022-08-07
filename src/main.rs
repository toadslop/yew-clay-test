use std::rc::Rc;

use web_sys::MouseEvent;
use yew::{html, Callback, Component, Context, Html};
use yew_clay::button::{
    ClayButton, ClayButtonGroup, ClayButtonProps, ClayButtonWithIcon, DisplayType,
};
use yew_clay::icon::ClayIcon;
use yew_dom_attributes::attributes::aria_attributes::AriaAttributes;
use yew_dom_attributes::attributes::button_html_attributes::ButtonHtmlAttributes;
use yew_dom_attributes::events::events::{EventType, MouseEvents};
use yew_dom_attributes::props::aria_props::AriaPropsHandler;
use yew_dom_attributes::props::button_props::{ButtonProps, ButtonPropsHandler};
use yew_dom_attributes::props::custom_attributes::{CustomAttribute, CustomPropsHandler};
use yew_dom_attributes::props::DomInjector;

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
        let on_btn_props_update: Callback<Rc<ButtonProps>> = ctx
            .link()
            .callback(move |btn_props: Rc<ButtonProps>| Msg::UpdateBtnPrimaryProps(btn_props));

        let mut button_primary_props = Rc::new(ButtonProps::new(on_btn_props_update));

        Rc::make_mut(&mut button_primary_props).add_aria_prop(AriaAttributes::AriaAtomic(true));

        let callback: Callback<MouseEvent> = ctx
            .link()
            .callback(move |_ev| Msg::RemoveListener("click-event".into()));

        Rc::make_mut(&mut button_primary_props).add_listener(
            "click-event".into(),
            EventType::MouseEvent(MouseEvents::Click(callback)),
        );

        let on_btn_warning_props_update: Callback<Rc<ButtonProps>> = ctx
            .link()
            .callback(move |btn_props: Rc<ButtonProps>| Msg::UpdateBtnWarningProps(btn_props));

        let button_warning_props = Rc::new(ButtonProps::new(on_btn_warning_props_update));

        let callback: Callback<MouseEvent> = ctx.link().callback(move |_ev| Msg::ToggleDisabled);

        Rc::make_mut(&mut button_primary_props).add_listener(
            "set-disabled".into(),
            EventType::MouseEvent(MouseEvents::Click(callback)),
        );
        button_warning_props
            .get_props_update_callback()
            .emit(button_warning_props.clone());

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
                    Rc::make_mut(&mut self.button_warning_props)
                        .add_button_prop(ButtonHtmlAttributes::Disabled);
                } else {
                    Rc::make_mut(&mut self.button_warning_props)
                        .remove_button_prop(ButtonHtmlAttributes::Disabled);
                }

                Rc::make_mut(&mut self.button_warning_props).add_custom_prop(CustomAttribute::new(
                    "my-custom-attribute".into(),
                    "lalalala".into(),
                ));
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
