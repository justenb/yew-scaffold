use crate::containers;
use crate::pages;
use yew::prelude::*;
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};

#[derive(Switch, Debug, Clone)]
pub enum AppRouter {
    #[to = "/!"]
    RootPath,
    #[to = "/contact!"]
    ContactPath,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}

#[derive(Debug)]
pub struct App {
    link: ComponentLink<Self>,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { link }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
          <>
            <containers::base::BaseContainer>
                <Router<AppRouter, ()>
                    render = Router::render(|switch: AppRouter | {
                        match switch {
                            AppRouter::RootPath => html!{
                                <pages::home::HomePage/>
                            },
                            AppRouter::ContactPath => html!{
                                html!{<strong>{"Contact"}</strong>}
                            },
                            AppRouter::PageNotFound(Permissive(None)) => html!{"Page not found"},
                            AppRouter::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                        }
                    })
                    redirect = Router::redirect(|route: Route<()>| {
                        AppRouter::PageNotFound(Permissive(Some(route.route)))
                    })
                />
            </containers::base::BaseContainer>
          </>
        }
    }
}
