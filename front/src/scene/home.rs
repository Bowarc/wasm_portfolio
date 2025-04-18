use crate::component;
use yew::{function_component, html, Html};

#[function_component]
pub fn Home() -> Html {
    use i18nrs::yew::use_translation;

    let (i18n, _) = use_translation();

    html! { <>
        <header>
            // <h1>{ "Welcome to my Portfolio" }</h1>
            <p class="warning">{ i18n.t("home.construction_warning") }</p>
        </header>

        <section id="description">
            <p>{ "I'm a self-taught " }<component::Age />{ " years old developer, focusing on software and backend development." }</p>
            <p>{ "I started with Python but now mainly work with Rust because I love its performance, safety, and solid type system." }</p>

            <p>{ "I also do front-end development, primarily using " }<a href="https://yew.rs/" class="link">{ "Yew" }</a>{ " and " }<a href="https://webassembly.org/" class="link">{ "WebAssembly" }</a>
            { "." }
            <br />
            { "While I have experience with React and Next.js, I find Yew to be a more enjoyable framework to work with." }</p>
            <p>{ "Maintainer of the " }<a href="https://ggez.rs/" class="link">{ "ggez" }</a>{ " 2D game framework." }</p>
        </section>
        <section id="skill_list">
            <h2>{ i18n.t("home.skills.title") }</h2>
            <ul>
            <li><div class="skill_card">
                <h3><a href="https://www.rust-lang.org" target="_blank" class="link">{ "Rust" }</a></h3>
                <img src="./resources/rust.webp" alt="Rust" class="skill_logo" />
            </div></li>

            <li><div class="skill_card">
                <h3><a href="https://www.python.org" target="_blank" class="link">{ "Python" }</a></h3>
                <img src="./resources/python.webp" alt="Python" class="skill_logo" />
            </div></li>

            <li><div class="skill_card">
                <h3><a href="https://en.wikipedia.org/wiki/C_(programming_language)" target="_blank" class="link">{ "C" }</a></h3>
                <img src="./resources/c.webp" alt="C" class="skill_logo" />
            </div></li>

            <li><div class="skill_card">
                <h3><a href="https://ziglang.org/" target="_blank" class="link">{ "Zig" }</a></h3>
                <img src="./resources/zig.webp" alt="Zig" class="skill_logo" />
            </div></li>

            <li><div class="skill_card">
                <h3><a href="https://en.wikipedia.org/wiki/C%2B%2B" target="_blank" class="link">{ "C++" }</a></h3>
                <img src="./resources/cpp.webp" alt="C++" class="skill_logo" />
            </div></li>

            <li><div class="skill_card">
                <h3><a href="https://www.ssh.com/ssh/" target="_blank" class="link">{ "SSH" }</a></h3>
                <img src="./resources/ssh.webp" alt="SSH" class="skill_logo" />
            </div></li>

            <li><div class="skill_card">
                <h3><a href="https://www.docker.com/" target="_blank" class="link">{ "Docker" }</a></h3>
                <img src="./resources/docker.webp" alt="docker" class="skill_logo" />
            </div></li>

            <li><div class="skill_card">
                <h3><a href="https://www.jetbrains.com/kotlin/" target="_blank" class="link">{ "Kotlin" }</a></h3>
                <img src="./resources/kotlin.webp" alt="Kotlin" class="skill_logo" />
            </div></li>

            <li><div class="skill_card">
                <h3><a href="https://www.java.com" target="_blank" class="link">{ "Java" }</a></h3>
                <img src="./resources/java.webp" alt="Java" class="skill_logo" />
            </div></li>

            <li><div class="skill_card">
                <h3><a href="https://en.wikipedia.org/wiki/Hypertext_Markup_Language" target="_blank" class="link">{ "HTML" }</a></h3>
                <img src="./resources/html.webp" alt="HTML" class="skill_logo" />
            </div></li>

            <li><div class="skill_card">
                <h3><a href="https://en.wikipedia.org/wiki/CSS" target="_blank" class="link">{ "CSS" }</a></h3>
                <img src="./resources/css.webp" alt="CSS" class="skill_logo" />
            </div></li>

            <li><div class="skill_card">
                <h3><a href="https://en.wikipedia.org/wiki/JavaScript" target="_blank" class="link">{ "JavaScript" }</a></h3>
                <img src="./resources/javascript.webp" alt="JavaScript" class="skill_logo" />
            </div></li>

            <li><div class="skill_card">
                <h3><a href="https://www.php.net" target="_blank" class="link">{ "PHP" }</a></h3>
                <img src="./resources/php.webp" alt="PHP" class="skill_logo" />
            </div></li>

            <li><div class="skill_card">
                <h3><a href="https://en.wikipedia.org/wiki/C_Sharp_(programming_language)" target="_blank" class="link">{ "C#" }</a></h3>
                <img src="./resources/csharp.webp" alt="C#" class="skill_logo" />
            </div></li>

            <li><div class="skill_card">
                <h3><a href="https://www.gnu.org/software/bash/" target="_blank" class="link">{ "Bash" }</a></h3>
                <img src="./resources/bash.webp" alt="Bash" class="skill_logo" />
            </div></li>

            <li><div class="skill_card">
                <h3><a href="https://en.wikipedia.org/wiki/PowerShell" target="_blank" class="link">{ "PowerShell" }</a></h3>
                <img src="./resources/pwsh2.webp" alt="PowerShell" class="skill_logo" />
            </div></li>


            // <li><div class="skill_card">
            //     <h3><a href="https://filezilla-project.org/" target="_blank" class="link">{ "FTP/SFTP" }</a></h3>
            //     <img src="./resources/ftp.webp" alt="FTP/SFTP" class="skill_logo" />
            // </div></li>

            <li><div class="skill_card">
                <h3><a href="https://git-scm.com/" target="_blank" class="link">{ "Git" }</a></h3>
                <img src="./resources/git.webp" alt="Git" class="skill_logo" />
            </div></li>

            <li><div class="skill_card">
                <h3><a href="https://www.mysql.com/" target="_blank" class="link">{ "SQL" }</a></h3>
                <img src="./resources/sql.webp" alt="SQL" class="skill_logo" />
            </div></li>
            </ul>
        </section>


    </>}
}
