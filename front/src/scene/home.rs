use crate::component;
use yew::html;

pub struct Home;

impl yew::Component for Home {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        html! { <>
            <header>
                <h1>{ "Bienvenue sur mon Portfolio" }</h1>
                // <p class="warning">{ "This is still under construction and nowhere close to done" }</p>
            </header>

            <section id="description">
                <h2>{ "About Me" }</h2>
                <p>{ "Self-taught " }<component::Age />{ " years old developer, focusing on software and backend development." }</p>
                <p>{ "I started with Python but now mainly work with Rust because I love its performance, safety, and solid type system." }</p>

                <p>{ "I also do front-end development, primarily using " }<a href="https://yew.rs/" class="link">{ "Yew" }</a>{ " and " }<a href="https://webassembly.org/" class="link">{ "WebAssembly" }</a>
                { "." }
                <br />
                { "While I have experience with React and Next.js, I find Yew to be a more enjoyable framework to work with." }</p>
                <p>{ "Maintainer of the " }<a href="https://ggez.rs/" class="link">{ "ggez" }</a>{ " 2D game framework." }</p>
            </section>
            <section id="skill_list">
                <h2>{ "Technical Skills" }</h2>
                <ul>
                <li><div class="skill_card">
                    <h3><a href="https://www.rust-lang.org" target="_blank" class="link">{ "Rust" }</a></h3>
                    <img src="./resources/rust.webp" alt="Rust" class="skill_logo" />
                    <p>{ "A programming language focused on performance and safety." }</p>
                </div></li>

                <li><div class="skill_card">
                    <h3><a href="https://www.python.org" target="_blank" class="link">{ "Python" }</a></h3>
                    <img src="./resources/python.webp" alt="Python" class="skill_logo" />
                    <p>{ "A versatile and easy-to-learn programming language." }</p>
                </div></li>

                <li><div class="skill_card">
                    <h3><a href="https://www.jetbrains.com/kotlin/" target="_blank" class="link">{ "Kotlin" }</a></h3>
                    <img src="./resources/kotlin.webp" alt="Kotlin" class="skill_logo" />
                    <p>{ "A programming language based on Java, primarily used for Android applications." }</p>
                </div></li>

                <li><div class="skill_card">
                    <h3><a href="https://www.java.com" target="_blank" class="link">{ "Java" }</a></h3>
                    <img src="./resources/java.webp" alt="Java" class="skill_logo" />
                    <p>{ "A widely used object-oriented programming language." }</p>
                </div></li>

                <li><div class="skill_card">
                    <h3><a href="https://en.wikipedia.org/wiki/Hypertext_Markup_Language" target="_blank" class="link">{ "HTML" }</a></h3>
                    <img src="./resources/html.webp" alt="HTML" class="skill_logo" />
                    <p>{ "A markup language designed to represent web pages." }</p>
                </div></li>

                <li><div class="skill_card">
                    <h3><a href="https://en.wikipedia.org/wiki/CSS" target="_blank" class="link">{ "CSS" }</a></h3>
                    <img src="./resources/css.webp" alt="CSS" class="skill_logo" />
                    <p>{ "A stylesheet language that describes the presentation of HTML and XML documents." }</p>
                </div></li>

                <li><div class="skill_card">
                    <h3><a href="https://en.wikipedia.org/wiki/JavaScript" target="_blank" class="link">{ "JavaScript" }</a></h3>
                    <img src="./resources/javascript.webp" alt="JavaScript" class="skill_logo" />
                    <p>{ "A scripting programming language primarily used in web pages." }</p>
                </div></li>

                <li><div class="skill_card">
                    <h3><a href="https://www.php.net" target="_blank" class="link">{ "PHP" }</a></h3>
                    <img src="./resources/php.webp" alt="PHP" class="skill_logo" />
                    <p>{ "A server-side scripting language for web development." }</p>
                </div></li>

                <li><div class="skill_card">
                    <h3><a href="https://en.wikipedia.org/wiki/C_(programming_language)" target="_blank" class="link">{ "C" }</a></h3>
                    <img src="./resources/c.webp" alt="C" class="skill_logo" />
                    <p>{ "A general-purpose programming language known for its performance." }</p>
                </div></li>

                <li><div class="skill_card">
                    <h3><a href="https://en.wikipedia.org/wiki/C%2B%2B" target="_blank" class="link">{ "C++" }</a></h3>
                    <img src="./resources/cpp.webp" alt="C++" class="skill_logo" />
                    <p>{ "An extension of the C language with object-oriented features." }</p>
                </div></li>

                <li><div class="skill_card">
                    <h3><a href="https://en.wikipedia.org/wiki/C_Sharp_(programming_language)" target="_blank" class="link">{ "C#" }</a></h3>
                    <img src="./resources/csharp.webp" alt="C#" class="skill_logo" />
                    <p>{ "A modern programming language that is easy to use." }</p>
                </div></li>

                <li><div class="skill_card">
                    <h3><a href="https://ziglang.org/" target="_blank" class="link">{ "Zig" }</a></h3>
                    <img src="./resources/zig.webp" alt="Zig" class="skill_logo" />
                    <p>{ "A programming language focused on safety and performance." }</p>
                </div></li>

                <li><div class="skill_card">
                    <h3><a href="https://www.gnu.org/software/bash/" target="_blank" class="link">{ "Bash" }</a></h3>
                    <img src="./resources/bash.webp" alt="Bash" class="skill_logo" />
                    <p>{ "A Unix shell and command language." }</p>
                </div></li>

                <li><div class="skill_card">
                    <h3><a href="https://en.wikipedia.org/wiki/PowerShell" target="_blank" class="link">{ "PowerShell" }</a></h3>
                    <img src="./resources/pwsh2.webp" alt="PowerShell" class="skill_logo" />
                    <p>{ "A task automation and configuration management framework." }</p>
                </div></li>

                <li><div class="skill_card">
                    <h3><a href="https://www.ssh.com/ssh/" target="_blank" class="link">{ "SSH" }</a></h3>
                    <img src="./resources/ssh.webp" alt="SSH" class="skill_logo" />
                    <p>{ "A protocol for securely accessing a remote computer." }</p>
                </div></li>

                <li><div class="skill_card">
                    <h3><a href="https://filezilla-project.org/" target="_blank" class="link">{ "FTP/SFTP" }</a></h3>
                    <img src="./resources/ftp.webp" alt="FTP/SFTP" class="skill_logo" />
                    <p>{ "Protocols for transferring files." }</p>
                </div></li>

                <li><div class="skill_card">
                    <h3><a href="https://git-scm.com/" target="_blank" class="link">{ "Git" }</a></h3>
                    <img src="./resources/git.webp" alt="Git" class="skill_logo" />
                    <p>{ "A distributed version control system." }</p>
                </div></li>

                <li><div class="skill_card">
                    <h3><a href="https://www.mysql.com/" target="_blank" class="link">{ "SQL" }</a></h3>
                    <img src="./resources/sql.webp" alt="SQL" class="skill_logo" />
                    <p>{ "A programming language for managing and manipulating databases." }</p>
                </div></li>
                </ul>
            </section>


        </>}
    }
}
