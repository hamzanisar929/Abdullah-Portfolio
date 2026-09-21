use leptos::prelude::*;

#[component]
pub fn Resume() -> impl IntoView {
    let (decoded, set_decoded) = signal(false);

    view! {
        <section class="resume-section" id="resume">
            <div class="resume-copy">
                <span class="resume-kicker">"PERSONNEL FILE / 2026"</span>
                <h2>"RESUME" <em>"DECODED."</em></h2>
                <p>"A focused field report covering the systems, products, and measurable outcomes behind the work. Tap the encrypted document to reveal and download the file."</p>
                <div class="resume-signals">
                    <span>"FULL-STACK"</span><span>"AI ENGINEERING"</span><span>"12 PROJECTS"</span>
                </div>
                <a class="resume-download hover-target" href="/resume/Muhammad-Hamza-Resume.pdf" download="Muhammad-Hamza-Resume.pdf">"DOWNLOAD RESUME" <span>"↓"</span></a>
            </div>

            <a class="resume-stage hover-target" class:decoded=move || decoded.get() on:click=move |_| set_decoded.update(|value| *value = !*value) href="/resume/Muhammad-Hamza-Resume.pdf" download="Muhammad-Hamza-Resume.pdf" aria-label="Decrypt and download Muhammad Hamza resume">
                <div class="resume-orbit orbit-one"></div>
                <div class="resume-orbit orbit-two"></div>
                <div class="resume-document">
                    <img src="/resume/resume-preview.png" alt="Muhammad Hamza resume preview" loading="lazy" />
                    <div class="resume-encryption">
                        <span>"ENCRYPTED PERSONNEL FILE"</span>
                        <strong>"MH—26"</strong>
                        <div class="cipher-lines">"01001101 01001000 00101111 00110010 00110110 10110101 01100100 01000101"</div>
                        <small>"CLICK TO DECODE"</small>
                    </div>
                    <div class="resume-scanline"></div>
                    <span class="resume-corner corner-a"></span><span class="resume-corner corner-b"></span>
                </div>
                <span class="resume-status">{move || if decoded.get() { "FILE DECRYPTED / CLICK TO ENCRYPT" } else { "SECURE FILE / ACCESS REQUIRED" }}</span>
            </a>
        </section>
    }
}
