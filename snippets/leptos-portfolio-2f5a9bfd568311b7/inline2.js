
export function lenis_scroll_to(selector) {
    const target = document.querySelector(selector);
    if (target && window.__lenis) {
        window.__lenis.scrollTo(target, { offset: -80 });
    }
}
