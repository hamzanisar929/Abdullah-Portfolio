// src/components/projects.rs — Darshan Vichhi Portfolio × Leptos 0.7
//
// ── UI COMPONENT ─────────────────────────────────────────────────────────────
// Fully encapsulated component. Data is injected statically at compile time.
// ─────────────────────────────────────────────────────────────────────────────

use leptos::prelude::*;
use crate::content;

#[component]
pub fn Projects() -> impl IntoView {
    let projects = content::load().projects;
    let (selected, set_selected) = signal(None::<content::Project>);
    view! {
        <section class="work-section" id="work">
            <div class="grid grid-cols-12 border-b">
                <div class="col-span-12 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center border-b">
                    <h2 class="section-heading font-sans text-[0.7rem] font-semibold tracking-[0.25em] uppercase text-muted">"SELECTED ARCHIVES"</h2>
                </div>
            </div>
            <div class="projects-card-grid">
                {projects.into_iter().map(|p| {
                    let open_project = p.clone();
                    let video = p.video.clone();
                    view! {
                        <article class="project-row showcase-card invisible hover-target" data-accent=p.accent.clone() on:click=move |_| set_selected.set(Some(open_project.clone()))>
                            <div class:has-video=p.video.is_some() class="project-media">
                                {video.map(|src| view! { <video class="project-preview" data-src=src playsinline preload="none"></video> })}
                                <img class="project-poster" src=p.poster.clone() alt=format!("{} project poster", p.title) loading="lazy" decoding="async" />
                                <span class="card-index">{p.num.clone()}</span>
                                <span class="card-category">{p.tags.first().cloned().unwrap_or_default()}</span>
                                {p.video.is_some().then(|| view! { <span class="preview-label">"HOVER · SOUND ON"</span> })}
                            </div>
                            <div class="showcase-copy">
                                <p class="showcase-eyebrow">{p.subtitle.clone()}</p>
                                <div class="showcase-title-row"><h3>{p.title.clone()}</h3><span class="card-expand">"+"</span></div>
                                <p class="showcase-summary">{p.desc.clone()}</p>
                                <div class="showcase-tags">{p.tags.iter().take(3).cloned().map(|t| view! { <span>{t}</span> }).collect_view()}</div>
                                {(!p.href.is_empty() && p.href != "#").then(|| view! {
                                    <a class="card-visit" href=p.href.clone() target="_blank" rel="noopener noreferrer" on:click=|ev| ev.stop_propagation()>"VIEW WEBSITE" <span>"↗"</span></a>
                                })}
                            </div>
                        </article>
                    }
                }).collect_view()}
            </div>
        </section>

        {move || selected.get().map(|p| {
            let close_button = set_selected;
            let close_backdrop = set_selected;
            view! {
                <div class="project-modal" role="dialog" aria-modal="true" aria-label=format!("{} project details", p.title) on:click=move |_| close_backdrop.set(None)>
                    <div class="project-modal-panel" on:click=|ev| ev.stop_propagation()>
                        <button class="modal-close hover-target" aria-label="Close project details" on:click=move |_| close_button.set(None)>"CLOSE ×"</button>
                        <div class="modal-media">
                            {match p.video.clone() { Some(src) => view! { <video src=src poster=p.poster.clone() controls autoplay prop:muted=false loop playsinline></video> }.into_any(), None => view! { <img src=p.poster.clone() alt=format!("{} project poster", p.title) /> }.into_any() }}
                        </div>
                        <div class="modal-copy">
                            <div class="modal-heading"><span>{p.num.clone()}</span><h3>{p.title.clone()}</h3><p>{p.subtitle.clone()}</p></div>
                            <div class="modal-description"><p>{p.desc.clone()}</p><div class="modal-tags">{p.tags.iter().cloned().map(|t| view! { <span>{t}</span> }).collect_view()}</div>{(!p.href.is_empty() && p.href != "#").then(|| view! { <a class="visit-site hover-target" href=p.href.clone() target="_blank" rel="noopener noreferrer">"VISIT WEBSITE ↗"</a> })}</div>
                        </div>
                    </div>
                </div>
            }
        })}
    }
}
