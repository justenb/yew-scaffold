**yew-scaffold** - skeleton project for building web assembly applications in rust, using the [`Yew`](https://github.com/yewstack/yew) framework.
===========================================


This template integrates the following rust projects:

- [`wasmbl`](https://github.com/wasmbl/wasmbl)  - wasm build tool that extends wasm builds with bundled assets, custom build specs, etc..
- [`ybc`](https://github.com/thedodd/ybc/) - a implementation of bulma components for yew
- [`warp`](https://github.com/seanmonstar/warp) - web server framework   


&nbsp;


![](https://assets.codepen.io/269372/template.png)&nbsp;

#### Using this template

***

click the 'use this template' button or clone the repository

```bash
git clone https://github.com/justenb/yew-wasmbl-bulma-scaffold
```

install [`cargo-make`](https://github.com/sagiegurari/cargo-make)

```bash
cargo install --force cargo-make
```



project structure

    .
    ├── frontend      # Yew components 
    ├── backend       # Warp server and wasm bundler 
    ├── common        # Workspace shared libs  
    ├── run           # Custom CLI commands 
    ├── Dockerfile    # Compile targets in a container
    ├── index.html    # 
    ├── Makefile.toml # Build tasks





#### Development

---

Starting the dev server

```bash
cargo make serve

#( will reload if any changes are made to the frontend )
# default listener is 4000
# use  cargo make --env DOCKER_ID="${your_docker_id}"  serve
# change default bind port.
```





#### **yew component layout**

 root component - *frontend/src/app.rs* 

```rust
// frontend/src/containers/base.rs

    fn view(&self) -> Html {
        html! {
            <>
            { BaseContainer::view_nav() }
                {self.props.children.clone()}
            { BaseContainer::view_footer() }
            </>
        }
    }
```



*containers* - parent components that will persist route changes

```rust
// frontend/src/containers/base.rs
    fn view(&self) -> Html {
        html! {
            <>
            { BaseContainer::view_nav() }
                {self.props.children.clone()}
            { BaseContainer::view_footer() }
            </>
        }
    }
```



*pages* - yew router targets

```rust
// frontend/src/app.rs
match switch {
        AppRouter::RootPath => html!{
            <pages::home::HomePage/>
        },
        ...
}
```



*mounting the app* 

```rust
#[wasm_bindgen(start)]
pub fn start_app() {
    let div = document().query_selector("#app").unwrap().unwrap();
    App::<app::App>::new().mount(div);
}
```

```html
<!DOCTYPE html>
<html lang="en">
    <head>
        <base href="{{ base_url }}">
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">

        {{ stylesheet | safe }}

        <title>Yew + Bulma Scaffold</title>
    </head>
    <body>
        <div id="app"></div>
        {{ javascript | safe }}
    </body>
</html>
```





#### Building your project

---

compile for local target

```bash
cargo make build
```



build project as a docker image

```bash
cargo make --env SERVE_PORT="${expose_port}" publish-docker
# SERVE_PORT defaults to 4000
```

- compiles the project in a build container  

- copies the target/release/{build-bin, build/} files from the build to an a alpine container, the current template's final container artifact is < 12MB



#### Publishing your project

---
```bash
cargo make --env DOCKER_ID="${your_docker_id}" publish-docker
```

release and push your build as a docker image to your configured container registry ( this task assumes docker is installed and a valid docker config is set   )


### Todo
---

- [ ] add more router templates
- [ ] add examples of integrating component state
- [ ] add callback examples
- [ ] suggestions?

**I am learning rust and wasm as I go, so please share and contribute!** 🎉