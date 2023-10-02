use leptos::*;

fn main() {
    mount_to_body(|cx| view! {cx, <App/>})
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <body>
            <div class="container">
                <header>
                    <h1 id="branding">"ProcKiller"</h1>
                </header>
                <div class="main">
                    <div class="process-info">
                        <div class="process-count">
                            <h2>"Number of Processes: "<span id="process-count">0</span></h2>
                        </div>
                        <div class="filter">
                            <input type="text" id="filter-input" placeholder="Filter by name" />
                        </div>
                        <div class="port-search">
                            <input type="text" id="port-input" placeholder="Find by port number" />
                            <button id="port-search-button">"Search"</button>
                        </div>
                    </div>
                    <div class="process-list">
                        <header>
                            <div class="header-item">"Select"</div>
                            <div class="header-item">"Name"</div>
                            <div class="header-item">"PID"</div>
                            <div class="header-item">"Parent PID"</div>
                        </header>
                    </div>
                    <div class="actions">
                        <button class="kill-button" id="kill-button">"Kill Selected Process"</button>
                    </div>
                </div>
            </div>
        </body>
    }
}
