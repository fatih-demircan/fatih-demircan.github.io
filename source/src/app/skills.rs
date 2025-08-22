use super::*;
use gloo_net::http::Request;
use std::collections::BTreeMap;

async fn fetch_logos() -> BTreeMap<String, BTreeMap<String, String>> {
    Request::get("public/logos.json")
        .send()
        .await
        .unwrap()
        .json::<BTreeMap<String, BTreeMap<String, String>>>()
        .await
        .unwrap()
}

#[component]
fn Li(title: String, entries: BTreeMap<String, String>) -> impl IntoView {
    view! {
        <li class="list-row font-semibold">
            {title}
            <div class="list-col-wrap flex flex-row flex-wrap gap-1 pt-1 pb-2">
                {entries
                    .into_iter()
                    .map(|entry| {
                        view! { <Badge entry=entry /> }
                    })
                    .collect_view()}
            </div>
        </li>
    }
}

#[component]
fn Badge(entry: (String, String)) -> impl IntoView {
    view! {
        <div class="badge badge-neutral badge-soft badge-sm indent-0 font-semibold">
            <img src=entry.1 class="h-9/10 aspect-auto" alt=entry.0.clone() />
            {entry.0}
        </div>
    }
}

#[component]
pub fn Skills() -> impl IntoView {
    let logos = LocalResource::new(|| fetch_logos());
    view! {
        <Card title="Skills".to_string()>
            <ul>
                {move || {
                    let result = logos.get().unwrap_or_default();
                    let app = result.get("app").unwrap_or(&BTreeMap::new()).clone();
                    let lang = result.get("lang").unwrap_or(&BTreeMap::new()).clone();
                    let pkg = result.get("pkg").unwrap_or(&BTreeMap::new()).clone();
                    view! {
                        <Li title="Applications".to_string() entries=app />
                        <Li title="Languages".to_string() entries=lang />
                        <Li title="Packages & Frameworks".to_string() entries=pkg />
                    }
                }}
            </ul>
        </Card>
    }
}
