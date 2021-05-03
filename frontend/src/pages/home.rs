use ybc::*;
use yew::prelude::*;

pub struct HomePage;
impl HomePage {
    fn view_tiles() -> Html {
        html! {
            <ybc::Tile ctx=ybc::TileCtx::Ancestor>
                <ybc::Tile vertical=true size=ybc::TileSize::Eight>

                    <ybc::Tile>

                        <ybc::Tile ctx=ybc::TileCtx::Parent vertical=true>
                            <ybc::Tile ctx=ybc::TileCtx::Child tag="article" classes="is-primary notification">
                                <p class="title">{"Vertical..."}</p>
                                <p class="subtitle">{"Top tile"}</p>
                            </ybc::Tile>
                            <ybc::Tile ctx=ybc::TileCtx::Child tag="article" classes="is-warning notification">
                                <p class="title">{"...tiles"}</p>
                                <p class="subtitle">{"Bottom tile"}</p>
                            </ybc::Tile>
                        </ybc::Tile>

                        <ybc::Tile ctx=ybc::TileCtx::Parent>
                            <ybc::Tile ctx=ybc::TileCtx::Child tag="article" classes="is-info notification">
                                <p class="title">{"Middle tile"}</p>
                                <p class="subtitle">{"With an image"}</p>
                                <figure class="image is-4by3">
                                    <img src="https://bulma.io/images/placeholders/640x480.png"/>
                                </figure>
                            </ybc::Tile>
                        </ybc::Tile>
                    </ybc::Tile>

                    <ybc::Tile ctx=ybc::TileCtx::Parent>
                        <ybc::Tile ctx=ybc::TileCtx::Child tag="article" classes="is-danger notification">
                            <p class="title">{"Wide tile"}</p>
                            <p class="subtitle">{"Aligned with the right tile"}</p>
                            <ybc::Content>
                            {"Lorem ipsum dolor sit amet, consectetur adipiscing elit.
                            Proin ornare magna eros, eu pellentesque tortor vestibulum ut. 
                            Maecenas non massa sem. Etiam finibus odio quis feugiat facilisis."}
                            </ybc::Content>
                        </ybc::Tile>
                    </ybc::Tile>
                </ybc::Tile>
                <ybc::Tile ctx=ybc::TileCtx::Parent>
                    <ybc::Tile tag="article" ctx=ybc::TileCtx::Child classes="notification is-success">
                            <p class="title">{"Tall tile"}</p>
                            <p class="subtitle"></p>
                        <ybc::Content>
                            {"Lorem ipsum dolor sit amet, consectetur adipiscing elit.
                            Etiam semper diam at erat pulvinar, at pulvinar felis blandit. 
                            Vestibulum volutpat tellus diam, consequat gravida libero rhoncus ut.
                            Morbi maximus, leo sit amet vehicula eleifend, nunc dui porta orci, quis semper odio felis ut quam.
                            Suspendisse varius ligula in molestie lacinia. Maecenas varius eget ligula a sagittis. Pellentesque interdum, nisl
                            nec interdum maximus, augue diam porttitor lorem, et sollicitudin felis neque sit amet erat. 
                            Maecenas imperdiet felis nisi, fringilla luctus felis hendrerit sit amet. 
                            Aenean vitae gravida diam, finibus dignissim turpis. Sed eget varius ligula, at volutpat tortor.
                            Integer sollicitudin, tortor a mattis commodo, velit urna rhoncus erat, vitae congue lectus dolor consequat libero.
                            Donec leo ligula, maximus et pellentesque sed, gravida a metus. Cras ullamcorper a nunc ac porta. 
                            Aliquam ut aliquet lacus, quis faucibus libero. Quisque non semper leo."}
                        </ybc::Content>
                    </ybc::Tile>
                </ybc::Tile>
            </ybc::Tile>
        }
    }
    fn view_levels() -> Html {
        html! {
            <ybc::Level tag="section" classes="is-medium">
                <ybc::LevelItem classes="has-text-centered">
                    <p class="heading">{"Tweets"}</p>
                    <p class="title">{"3,456"}</p>
                </ybc::LevelItem>
                <ybc::LevelItem classes="has-text-centered">
                    <p class="heading">{"Following"}</p>
                    <p class="title">{"123"}</p>
                </ybc::LevelItem>
                <ybc::LevelItem classes="has-text-centered">
                    <p class="heading">{"Followers"}</p>
                    <p class="title">{"456K"}</p>
                </ybc::LevelItem>
                <ybc::LevelItem classes="has-text-centered">
                    <p class="heading">{"Likes"}</p>
                    <p class="title">{"789"}</p>
                </ybc::LevelItem>
            </ybc::Level>
        }
    }

    fn view_page() -> Html {
        html! {
            <>
            <ybc::Hero size={HeroSize::Medium} classes="is-primary"
                body=html!{
                    <ybc::Container fluid=true>
                        <h1 class="title has-text-centered">{"Hero Section"}</h1>
                        <h2 class="subtitle has-text-centered">{"Basic Layout Skeleton using Bulma CSS Framework"}</h2>
                    </ybc::Container>
                }
            />
            <ybc::Section classes="has-background-white-bis">
                <ybc::Container fluid=true>
                    <h1 class="title has-text-centered">{"Section Example"}</h1>
                    <h2 class="subtitle has-text-centered">{"A simple container to divide your page into "}<strong>{"sections"}</strong>{",like the one you're currently reading"}</h2>
                </ybc::Container>
            </ybc::Section>

            <ybc::Section size={SectionSize::Medium}>
                <ybc::Container fluid=true>
                    <h1 class="title has-text-centered">{"Levels Example"}</h1>
                {HomePage::view_levels()}
                </ybc::Container>
            </ybc::Section>


            <ybc::Section classes="has-background-white-er">
                <ybc::Container fluid=true classes="pb-6">
                    <h1 class="title has-text-centered">{"Tiles Example"}</h1>
                    {HomePage::view_tiles()}
                </ybc::Container>
            </ybc::Section>
            </>
        }
    }
}

impl Component for HomePage {
    type Properties = ();
    type Message = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
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
          {HomePage::view_page()}
        </>
        }
    }
}
