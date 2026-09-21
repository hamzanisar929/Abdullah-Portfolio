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
    let (expanded, set_expanded) = signal(None::<String>);
    view! {
        <section class="work-section" id="work">
            <div class="grid grid-cols-12 border-b">
                <div class="col-span-12 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center border-b">
                    <h2 class="section-heading font-sans text-[0.7rem] font-semibold tracking-[0.25em] uppercase text-muted">"SELECTED ARCHIVES"</h2>
                </div>
            </div>
            <div class="projects-card-grid">
                {projects.into_iter().map(|p| {
                    let project_key = p.title.clone();
                    let click_key = project_key.clone();
                    let class_key = project_key.clone();
                    let video = p.video.clone();
                    view! {
                        <article
                            class="project-row showcase-card invisible hover-target"
                            class:expanded=move || expanded.get().as_ref() == Some(&class_key)
                            data-accent=p.accent.clone()
                            on:click=move |_| {
                                let next = if expanded.get().as_ref() == Some(&click_key) { None } else { Some(click_key.clone()) };
                                set_expanded.set(next);
                            }
                        >
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
                                <div class="expanded-details">
                                    <span>"PROJECT FILE / " {p.num.clone()}</span>
                                    <p>{p.desc.clone()}</p>
                                    <p>"Explore the complete product flow in the preview, then open the live build to review its responsive interface and core workflows."</p>
                                </div>
                                {(!p.href.is_empty() && p.href != "#").then(|| view! {
                                    <a class="card-visit" href=p.href.clone() target="_blank" rel="noopener noreferrer" on:click=|ev| ev.stop_propagation()>"VIEW WEBSITE" <span>"↗"</span></a>
                                })}
                            </div>
                        </article>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}
