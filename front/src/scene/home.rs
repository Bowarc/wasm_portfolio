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
               <h2>{ "À propos de moi" }</h2>
               <p>{ "Développeur autodidacte de " }<component::Age />{ " ans, spécialisé dans le développement logiciel et backend." }</p>
               <p>{ "J'ai commencé avec Python avant de me concentrer principalement sur Rust, un langage dans lequel je vois un potentiel considérable pour ses performances et sa sécurité." }</p>
               <p>{ "Je suis familier avec les environnements Windows et Linux, ce qui me permet de concevoir des solutions robustes et polyvalentes." }</p>
               <p>{ "Mon expertise s'étend à la construction et à la maintenance d'applications de bureau ainsi qu'à la mise en place de solutions côté serveur." }</p>
               <p>{ "En outre, je suis compétent dans le développement front-end, notamment avec " }<a href="https://yew.rs/" class="link">{ "Yew" }</a>{ " via " }<a href="https://webassembly.org/" class="link">{ "WebAssembly" }</a>{ ", ce qui me permet de créer des expériences utilisateur riches et réactives." }</p>
               <p>{ "J'ai eu l'occasion de travailler sur divers projets, dont " }<a href="https://github.com/Bowarc/chess_game" class="link">{ "un jeu d'échec en multijoueur" }</a>{ ", démontrant ma capacité à livrer des solutions de haute qualité dans des environnements variés." }</p>
               <p>{ "Je suis passionné par l'innovation technologique et je m'efforce constamment d'approfondir mes compétences pour relever de nouveaux défis." }</p>
            </section>

            <section id="skill_list">
               <h2>{ "Compétences Techniques" }</h2>
               <ul>
               <li><div class="skill_card">
                  <h3><a href="https://www.rust-lang.org" target="_blank" class="link">{ "Rust" }</a></h3>
                  <img src="./resources/rust.webp" alt="Rust" class="skill_logo" />
                  <p>{ "Langage de programmation orienté vers la performance et la sécurité." }</p>
               </div></li>

               <li><div class="skill_card">
                  <h3><a href="https://www.python.org" target="_blank" class="link">{ "Python" }</a></h3>
                  <img src="./resources/python.webp" alt="Python" class="skill_logo" />
                  <p>{ "Langage de programmation polyvalent et facile à apprendre." }</p>
               </div></li>

               <li><div class="skill_card">
                  <h3><a href="https://www.jetbrains.com/kotlin/" target="_blank" class="link">{ "Kotlin" }</a></h3>
                  <img src="./resources/kotlin.webp" alt="Kotlin" class="skill_logo" />
                  <p>{ "Langage de programmation basé sur java, principalement utilisé pour des application android." }</p>
               </div></li>

               <li><div class="skill_card">
                  <h3><a href="https://www.java.com" target="_blank" class="link">{ "Java" }</a></h3>
                  <img src="./resources/java.webp" alt="Java" class="skill_logo" />
                  <p>{ "Langage de programmation orienté objet largement utilisé." }</p>
               </div></li>

               <li><div class="skill_card">
                  <h3><a href="https://fr.wikipedia.org/wiki/Hypertext_Markup_Language" target="_blank" class="link">{ "html" }</a></h3>
                  <img src="./resources/html.webp" alt="html" class="skill_logo" />
                  <p>{ "Langage de balisage conçu pour représenter les pages web." }</p>
               </div></li>

               <li><div class="skill_card">
                  <h3><a href="https://en.wikipedia.org/wiki/CSS" target="_blank" class="link">{ "css" }</a></h3>
                  <img src="./resources/css.webp" alt="css" class="skill_logo" />
                  <p>{ "Langage informatique qui décrit la présentation des documents HTML et XML." }</p>
               </div></li>

               <li><div class="skill_card">
                  <h3><a href="https://en.wikipedia.org/wiki/JavaScript" target="_blank" class="link">{ "javascript" }</a></h3>
                  <img src="./resources/javascript.webp" alt="js" class="skill_logo" />
                  <p>{ "Langage de programmation de scripts principalement employé dans les pages web." }</p>
               </div></li>

               <li><div class="skill_card">
                  <h3><a href="https://www.php.net" target="_blank" class="link">{ "PHP" }</a></h3>
                  <img src="./resources/php.webp" alt="PHP" class="skill_logo" />
                  <p>{ "Langage de script côté serveur pour le développement web." }</p>
               </div></li>

               <li><div class="skill_card">
                  <h3><a href="https://en.wikipedia.org/wiki/C_(programming_language)" target="_blank" class="link">{ "C" }</a></h3>
                  <img src="./resources/c.webp" alt="C" class="skill_logo" />
                  <p>{ "Langage de programmation généraliste très performant." }</p>
               </div></li>

               <li><div class="skill_card">
                  <h3><a href="https://isocpp.org/" target="_blank" class="link">{ "C++" }</a></h3>
                  <img src="./resources/cpp.webp" alt="C++" class="skill_logo" />
                  <p>{ "Extension du langage C avec des fonctionnalités orientées objet." }</p>
               </div></li>

               <li><div class="skill_card">
                  <h3><a href="https://learn.microsoft.com/en-us/dotnet/csharp/" target="_blank" class="link">{ "C#" }</a></h3>
                  <img src="./resources/csharp.webp" alt="C#" class="skill_logo" />
                  <p>{ "Langage de programmation moderne et simple à utiliser." }</p>
               </div></li>

               <li><div class="skill_card">
                  <h3><a href="https://ziglang.org/" target="_blank" class="link">{ "Zig" }</a></h3>
                  <img src="./resources/zig.webp" alt="Zig" class="skill_logo" />
                  <p>{ "Langage de programmation orienté vers la sécurité et la performance." }</p>
               </div></li>

               <li><div class="skill_card">
                  <h3><a href="https://www.gnu.org/software/bash/" target="_blank" class="link">{ "Bash" }</a></h3>
                  <img src="./resources/bash.webp" alt="Bash" class="skill_logo" />
                  <p>{ "Shell Unix et langage de commande." }</p>
               </div></li>

               <li><div class="skill_card">
                  <h3><a href="https://docs.microsoft.com/en-us/powershell/" target="_blank" class="link">{ "PowerShell" }</a></h3>
                  <img src="./resources/pwsh2.webp" alt="PowerShell" class="skill_logo" />
                  <p>{ "Automatisation des tâches et gestion de configuration." }</p>
               </div></li>

               <li><div class="skill_card">
                  <h3><a href="https://www.ssh.com/ssh/" target="_blank" class="link">{ "SSH" }</a></h3>
                  <img src="./resources/ssh.webp" alt="SSH" class="skill_logo" />
                  <p>{ "Protocole pour accéder de manière sécurisée à un ordinateur à distance." }</p>
               </div></li>

               <li><div class="skill_card">
                  <h3><a href="https://filezilla-project.org/" target="_blank" class="link">{ "FTP/SFTP" }</a></h3>
                  <img src="./resources/ftp.webp" alt="FTP/SFTP" class="skill_logo" />
                  <p>{ "Protocoles pour le transfert de fichiers." }</p>
               </div></li>

               <li><div class="skill_card">
                  <h3><a href="https://git-scm.com/" target="_blank" class="link">{ "Git" }</a></h3>
                  <img src="./resources/git.webp" alt="Git" class="skill_logo" />
                  <p>{ "Système de contrôle de version distribué." }</p>
               </div></li>

               <li><div class="skill_card">
                  <h3><a href="https://www.mysql.com/" target="_blank" class="link">{ "SQL" }</a></h3>
                  <img src="./resources/sql.webp" alt="SQL" class="skill_logo" />
                  <p>{ "Langage de programmation pour gérer et manipuler les bases de données." }</p>
               </div></li>
               </ul>
           </section>


        </>}
    }
}
