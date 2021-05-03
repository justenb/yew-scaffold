use yew::prelude::*;

pub struct BaseContainer {
    props: BaseContainerProps,
}

impl BaseContainer {
    fn view_footer() -> Html {
        html! {
            <ybc::Footer classes="has-text-grey-lighter has-background-grey-darker">
            <ybc::Content classes="has-text-grey-lighter has-text-centered">
                <p>{"Footer Example, Content goes here..."} </p>
            </ybc::Content>
            </ybc::Footer>
        }
    }

    fn view_nav() -> Html {
        html! {
          <ybc::Navbar
            classes="is-transparent"
            navburger=true
            navbrand=html!{
                    <ybc::NavbarItem tag=html_nested!{ybc::NavbarItemTag::A} href="" >
                        <img src="https://bulma.io/images/bulma-logo.png" width="112" height="28"/>
                    </ybc::NavbarItem>
            }
            navstart=html!{
                <>
                    <ybc::NavbarItem tag=html_nested!{ybc::NavbarItemTag::A}> {"Home"} </ybc::NavbarItem>
                    <ybc::NavbarItem has_dropdown=true tag=html_nested!{ybc::NavbarItemTag::A}>
                        <ybc::NavbarDropdown boxed=true hoverable=true navlink=html!{{"About Us"}}>
                            <ybc::NavbarItem tag=html_nested!{ybc::NavbarItemTag::A} href="/contact"> {"Contact"}</ybc::NavbarItem>
                        </ybc::NavbarDropdown>
                    </ybc::NavbarItem>
                </>
            }
            navend=html!{
                    <ybc::NavbarItem tag=html_nested!{ybc::NavbarItemTag::Div}>
                        <ybc::Buttons>
                            <ybc::Button classes="is-primary">{"Sign Up"}</ybc::Button>
                            <ybc::Button>{"Login"}</ybc::Button>
                        </ybc::Buttons>
                    </ybc::NavbarItem>
            }
            />
        }
    }
}

#[derive(Debug, Properties, Clone)]
pub struct BaseContainerProps {
    children: Children,
}

impl Component for BaseContainer {
    type Properties = BaseContainerProps;
    type Message = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
    /* .. snip .. */
    fn view(&self) -> Html {
        html! {
            <>
            { BaseContainer::view_nav() }
                {self.props.children.clone()}
            { BaseContainer::view_footer() }
            </>
        }
    }
}
