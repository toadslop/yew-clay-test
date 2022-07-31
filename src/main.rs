use material_yew::MatButton;
use web_sys::console;
use yew::prelude::*;
use yew_clay::button::Group;
use yew_clay::button::{ClayButton, DisplayType};
use yew_clay::icon::ClayIcon;
use yew_dom_attributes::aria_attributes::{
    AriaAttributes, AriaAutocomplete, AriaChecked, AriaHasPopup,
};
use yew_dom_attributes::misc_attributes::MiscAttrs;

enum Msg {
    AddOne,
    SetDisabled(bool),
}

struct Model {
    value: i64,
    btn_disabled: bool,
    btn_1_misc_attrs: MiscAttrs,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut misc_attrs = MiscAttrs::new();
        misc_attrs.add_attribute("id".into(), "my-id".into());
        misc_attrs.add_attribute("data-cy".into(), "clay-button".into());
        Self {
            value: 0,
            btn_disabled: false,
            btn_1_misc_attrs: misc_attrs,
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
                self.btn_disabled = is_disabled;
                self.btn_1_misc_attrs
                    .add_attribute("i-disabled-it".into(), "woo".into());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let is_disabled = self.btn_disabled.to_owned();
        let on_click: Callback<MouseEvent> = link.callback(move |_ev| {
            console::log_1(&"Hello using web-sys".into());
            Msg::SetDisabled(!is_disabled)
        });

        let spritemap = "https://cdn.jsdelivr.net/npm/@clayui/css/lib/images/icons/icons.svg";
        let aria = AriaAttributes {
            aria_activedescendant: Some("hi".into()),
            aria_atomic: Some(false),
            aria_autocomplete: Some(AriaAutocomplete::List),
            aria_busy: Some(true),
            aria_checked: Some(AriaChecked::Mixed),
            aria_colcount: Some(8),
            aria_colindex: Some(8),
            aria_haspopup: Some(AriaHasPopup::Dialog),
            ..Default::default()
        };
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p data-key={"something"} > { self.value }</p>
                <MatButton label="Click me!"  />
                <Group spaced={true} class={"stupid-class"}>
                    <ClayButton
                        misc_attrs={self.btn_1_misc_attrs.clone()}
                        disabled={self.btn_disabled}
                        display_type={DisplayType::Info}
                        onclick={on_click}
                        aria={aria}
                        >
                    {"Click Me"}
                    </ClayButton>
                    <ClayButton
                        display_type={DisplayType::Warning}
                        >
                    {"Other Button"}
                    </ClayButton>

                </Group>
                <ClayIcon spritemap={spritemap} symbol={"add-cell"}></ClayIcon>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
