
export function init_cursor() {
    const cursor = document.getElementById('blend-cursor');
    if (!cursor) return;
    let mx = -100, my = -100, cx = -100, cy = -100;

    document.addEventListener('mousemove', (e) => { mx = e.clientX; my = e.clientY; });

    function tick() {
        cx += (mx - cx) * 0.15;
        cy += (my - cy) * 0.15;
        cursor.style.transform = 'translate3d(' + cx + 'px,' + cy + 'px,0)';
        requestAnimationFrame(tick);
    }
    requestAnimationFrame(tick);

    document.querySelectorAll('.hover-target').forEach(el => {
        el.addEventListener('mouseenter', () => cursor.classList.add('hovering'));
        el.addEventListener('mouseleave', () => cursor.classList.remove('hovering'));
    });
}
