
export function init_lenis() {
    window.__lenis = new Lenis({
        duration: 1.2,
        easing: (t) => Math.min(1, 1.001 - Math.pow(2, -10 * t)),
        touchMultiplier: 2,
    });
    window.__lenis.on('scroll', ScrollTrigger.update);
    gsap.ticker.add((time) => window.__lenis.raf(time * 1000));
    gsap.ticker.lagSmoothing(0);
}
