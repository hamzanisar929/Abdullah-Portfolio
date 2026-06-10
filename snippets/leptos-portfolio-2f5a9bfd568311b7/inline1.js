
export function play_splash() {
    // 1) First hide elements that will be animated in
    gsap.set('.hero-title .line', { opacity: 0, y: 50 });
    gsap.set('.hero-body', { opacity: 0, y: 30 });
    gsap.set('.stat-block', { opacity: 0, x: 40 });
    
    // Also hide header and ticker elements
    gsap.set('.ed-header > div', { opacity: 0, y: -20 });
    gsap.set('.ticker-wrap', { opacity: 0 });

    const tl = gsap.timeline({
        onComplete: () => {
            document.body.classList.remove('is-loading');
            animate_on_scroll_inner();
        }
    });

    tl.to('.stamp-1', { opacity:1, y:0, scaleY:1, duration:0.3, ease:'back.out(3)' }, 0.3);
    tl.to('.stamp-2', { opacity:1, y:0, scaleY:1, duration:0.3, ease:'back.out(3)' }, 0.5);
    tl.to('.stamp-3', { opacity:1, y:0, scaleY:1, duration:0.3, ease:'back.out(3)' }, 0.7);
    tl.to('.stamp-4', { opacity:1, y:0, scaleY:1, duration:0.3, ease:'back.out(3)' }, 0.9);

    tl.to('.ring-fill', {
        strokeDashoffset: 0,
        duration: 1.5,
        ease: 'power2.inOut'
    }, 0.3);

    tl.to('.stamp-container', {
        opacity: 0,
        y: -40,
        duration: 0.4,
        ease: 'power2.in'
    }, '+=0.3');

    tl.to('.splash-progress-ring', {
        opacity: 0,
        scale: 0.5,
        duration: 0.3,
        ease: 'power2.in'
    }, '-=0.3');

    tl.to('#splash-screen', {
        yPercent: -100,
        duration: 0.7,
        ease: 'power3.inOut',
        onComplete: () => {
            const s = document.getElementById('splash-screen');
            if (s) s.style.display = 'none';
            // Start hero animation immediately after splash is gone
            animate_hero_inner();
        }
    });
}

function animate_hero_inner() {
    const tl = gsap.timeline({ defaults: { ease: 'power4.out' } });
    
    // Animate Header in First
    tl.to('.ed-header > div', { opacity: 1, y: 0, duration: 0.8, stagger: 0.1 });
    // Then animate ticker and hero title together
    tl.to('.ticker-wrap', { opacity: 1, duration: 0.8 }, '-=0.4');
    tl.to('.hero-title .line', { opacity: 1, y: 0, duration: 1.4, stagger: 0.15 }, '-=0.6');
    // Then body text
    tl.to('.hero-body',  { opacity: 1, y: 0, duration: 1.0 }, '-=1.0');
    // Finally stats
    tl.to('.stat-block', { opacity: 1, x: 0, duration: 0.8, stagger: 0.1 }, '-=0.8');
}

function animate_on_scroll_inner() {
    // Refresh ScrollTrigger to ensure positions are correct after Hero has rendered completely
    ScrollTrigger.refresh();

    gsap.utils.toArray('.project-row').forEach((row, i) => {
        gsap.fromTo(row,
            { autoAlpha: 0, x: -40 },
            {
                scrollTrigger: { trigger: row, start: 'top 88%', toggleActions: 'play none none none' },
                autoAlpha: 1, x: 0, duration: 0.8, delay: i * 0.1, ease: 'expo.out'
            }
        );
    });

    gsap.from('.philosophy-section .hero-title', {
        scrollTrigger: { trigger: '.philosophy-section', start: 'top 70%', toggleActions: 'play none none none' },
        opacity:0, scale:0.9, duration:1.2, ease:'expo.out'
    });

    gsap.from('.geometric-deco', {
        scrollTrigger: { trigger: '.philosophy-section', start: 'top 70%', toggleActions: 'play none none none' },
        opacity:0, x:40, rotation:-30, duration:1, ease:'expo.out'
    });

    gsap.utils.toArray('.cv-row').forEach((row, i) => {
        gsap.from(row, {
            scrollTrigger: { trigger: row, start: 'top 88%', toggleActions: 'play none none none' },
            opacity:0, y:30, duration:0.7, delay: i * 0.12, ease:'expo.out'
        });
    });

    gsap.utils.toArray('.skills-section .col-3').forEach((b, i) => {
        gsap.from(b, {
            scrollTrigger: { trigger: '.skills-section', start: 'top 80%', toggleActions: 'play none none none' },
            opacity:0, y:40, duration:0.7, delay: i * 0.1, ease:'expo.out'
        });
    });

    gsap.utils.toArray('.rec-item').forEach((item, i) => {
        gsap.from(item, {
            scrollTrigger: { trigger: item, start: 'top 90%', toggleActions: 'play none none none' },
            opacity:0, x:-20, duration:0.6, delay: i * 0.1, ease:'expo.out'
        });
    });

    gsap.utils.toArray('.article-row').forEach((row, i) => {
        gsap.from(row, {
            scrollTrigger: { trigger: row, start: 'top 90%', toggleActions: 'play none none none' },
            opacity:0, y:20, duration:0.6, delay: i * 0.08, ease:'expo.out'
        });
    });

    gsap.from('.contact-hero .hero-title', {
        scrollTrigger: { trigger: '.contact-section', start: 'top 75%', toggleActions: 'play none none none' },
        opacity:0, y:50, duration:1.2, ease:'expo.out'
    });

    gsap.utils.toArray('.section-heading').forEach(h => {
        gsap.from(h, {
            scrollTrigger: { trigger: h, start: 'top 90%', toggleActions: 'play none none none' },
            opacity:0, x:-20, duration:0.8, ease:'expo.out'
        });
    });
}
