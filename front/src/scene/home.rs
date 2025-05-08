use crate::scene::Scene;
use crate::utils::age;

use yew::{function_component, html, Callback, Html};

#[derive(yew::Properties, std::cmp::PartialEq)]
pub struct Props {
    pub current_scene: yew::UseStateHandle<crate::scene::Scene>,
}

#[function_component]
pub fn Home(props: &Props) -> Html {
    use i18nrs::yew::use_translation;

    let (i18n, _) = use_translation();

    html! { <>
        <header>
            // <h1>{ "Welcome to my Portfolio" }</h1>
            <p class="warning">{ i18n.t("home.construction_warning") }</p>
        </header>

        <section id="description">
        <h2 class="hidable_element">{ i18n.t("home.description.intro.title") }</h2>
        <p class="hidable_element">
            { i18n.t("home.description.intro.content.1") }
            { age() }
            { i18n.t("home.description.intro.content.2") }
        </p>

        <h2 class="hidable_element">{ i18n.t("home.description.journey.title") }</h2>
        <p class="hidable_element">
            { i18n.t("home.description.journey.content.1") }
            { age()-16 }
            { i18n.t("home.description.journey.content.2") }
        </p>
        <p class="hidable_element">{ i18n.t("home.description.journey.content.3") }</p>

        <h2 class="hidable_element">{ i18n.t("home.description.frontend.title") }</h2>
        <p class="hidable_element">{ i18n.t("home.description.frontend.content.1") }</p>
        <p class="hidable_element">{ i18n.t("home.description.frontend.content.2") }</p>

        <h2 class="hidable_element">{ i18n.t("home.description.open_source.title") }</h2>
        <p class="hidable_element">{ i18n.t("home.description.open_source.content.1") }</p>
        <p class="hidable_element">{ i18n.t("home.description.open_source.content.2") }</p>

        <h2 class="hidable_element">{ i18n.t("home.description.experience.title") }</h2>
        <p class="hidable_element">{ i18n.t("home.description.experience.content") }</p>

        <h2 class="hidable_element">{ i18n.t("home.description.personal_statement.title") }</h2>
        <p class="hidable_element">{ i18n.t("home.description.personal_statement.content") }</p>

        <h2 class="hidable_element">{ i18n.t("home.description.connect.title") }</h2>
        <p class="hidable_element">
            { i18n.t("home.description.connect.content.1") }
            <button onclick={
                let current_scene_clone = props.current_scene.clone();
                Callback::from(move |_| current_scene_clone.set(Scene::Contact))
            }>{ i18n.t("home.description.connect.button_text") }</button>
            { i18n.t("home.description.connect.content.2") }
        </p>

        </section>
        <section id="skill_list" class="hidable_element">
            <h2>{ i18n.t("home.skills.title") }</h2>
            <ul>
            <li><div class="skill_card hidable_element">
                <h3><a href="https://www.rust-lang.org" target="_blank" class="link">{ "Rust" }</a></h3>
                <img src="./resources/rust.webp" alt="logo" class="skill_logo" />
            </div></li>

            <li><div class="skill_card hidable_element">
                <h3><a href="https://www.python.org" target="_blank" class="link">{ "Python" }</a></h3>
                <img src="./resources/python.webp" alt="logo" class="skill_logo" />
            </div></li>

            <li><div class="skill_card hidable_element">
                <h3><a href="https://en.wikipedia.org/wiki/C_(programming_language)" target="_blank" class="link">{ "C" }</a></h3>
                <img src="./resources/c.webp" alt="logo" class="skill_logo" />
            </div></li>

            <li><div class="skill_card hidable_element">
                <h3><a href="https://ziglang.org/" target="_blank" class="link">{ "Zig" }</a></h3>
                <img src="./resources/zig.webp" alt="logo" class="skill_logo" />
            </div></li>

            <li><div class="skill_card hidable_element">
                <h3><a href="https://en.wikipedia.org/wiki/C%2B%2B" target="_blank" class="link">{ "C++" }</a></h3>
                <img src="./resources/cpp.webp" alt="logo" class="skill_logo" />
            </div></li>

            <li><div class="skill_card hidable_element">
                <h3><a href="https://www.ssh.com/ssh/" target="_blank" class="link">{ "SSH" }</a></h3>
                <img src="./resources/ssh.webp" alt="logo" class="skill_logo" />
            </div></li>

            <li><div class="skill_card hidable_element">
                <h3><a href="https://www.docker.com/" target="_blank" class="link">{ "Docker" }</a></h3>
                <img src="./resources/docker.webp" alt="logo" class="skill_logo" />
            </div></li>

            <li><div class="skill_card hidable_element">
                <h3><a href="https://www.jetbrains.com/kotlin/" target="_blank" class="link">{ "Kotlin" }</a></h3>
                <img src="./resources/kotlin.webp" alt="logo" class="skill_logo" />
            </div></li>

            <li><div class="skill_card hidable_element">
                <h3><a href="https://www.java.com" target="_blank" class="link">{ "Java" }</a></h3>
                <img src="./resources/java.webp" alt="logo" class="skill_logo" />
            </div></li>

            <li><div class="skill_card hidable_element">
                <h3><a href="https://en.wikipedia.org/wiki/Hypertext_Markup_Language" target="_blank" class="link">{ "HTML" }</a></h3>
                <img src="./resources/html.webp" alt="logo" class="skill_logo" />
            </div></li>

            <li><div class="skill_card hidable_element">
                <h3><a href="https://en.wikipedia.org/wiki/CSS" target="_blank" class="link">{ "CSS" }</a></h3>
                <img src="./resources/css.webp" alt="logo" class="skill_logo" />
            </div></li>

            <li><div class="skill_card hidable_element">
                <h3><a href="https://en.wikipedia.org/wiki/JavaScript" target="_blank" class="link">{ "JavaScript" }</a></h3>
                <img src="./resources/javascript.webp" alt="logo" class="skill_logo" />
            </div></li>

            <li><div class="skill_card hidable_element">
                <h3><a href="https://www.php.net" target="_blank" class="link">{ "PHP" }</a></h3>
                <img src="./resources/php.webp" alt="logo" class="skill_logo" />
            </div></li>

            <li><div class="skill_card hidable_element">
                <h3><a href="https://en.wikipedia.org/wiki/C_Sharp_(programming_language)" target="_blank" class="link">{ "C#" }</a></h3>
                <img src="./resources/csharp.webp" alt="logo" class="skill_logo" />
            </div></li>

            <li><div class="skill_card hidable_element">
                <h3><a href="https://www.gnu.org/software/bash/" target="_blank" class="link">{ "Bash" }</a></h3>
                <img src="./resources/bash.webp" alt="logo" class="skill_logo" />
            </div></li>

            <li><div class="skill_card hidable_element">
                <h3><a href="https://en.wikipedia.org/wiki/PowerShell" target="_blank" class="link">{ "PowerShell" }</a></h3>
                <img src="./resources/pwsh2.webp" alt="logo" class="skill_logo" />
            </div></li>
            // <li><div class="skill_card hidable_element">
            //     <h3><a href="https://filezilla-project.org/" target="_blank" class="link">{ "FTP/SFTP" }</a></h3>
            //     <img src="./resources/ftp.webp" alt="logo" class="skill_logo" />
            // </div></li>
            <li><div class="skill_card hidable_element">
                <h3><a href="https://git-scm.com/" target="_blank" class="link">{ "Git" }</a></h3>
                <img src="./resources/git.webp" alt="logo" class="skill_logo" />
            </div></li>
            <li><div class="skill_card hidable_element">
                <h3><a href="https://www.mysql.com/" target="_blank" class="link">{ "SQL" }</a></h3>
                <img src="./resources/sql.webp" alt="logo" class="skill_logo" />
            </div></li>
            </ul>
        </section>


    </>}
}
