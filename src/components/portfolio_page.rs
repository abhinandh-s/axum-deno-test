use crate::home_page::Footer;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Portfolio)]
pub fn portfolio_page() -> Html {
    html!(
                            <>
                        <div class="bg-blur">
                            <PfHeader />
    <TestContent />


        /* From Uiverse.io by MuhammadHasann */
        <button class="pf-button3">
          <div class="pf_dots_border3"></div>
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            class="pf_sparkle3"
          >
            <path
              class="path"
              stroke-linejoin="round"
              stroke-linecap="round"
              stroke="black"
              fill="black"
              d="M14.187 8.096L15 5.25L15.813 8.096C16.0231 8.83114 16.4171 9.50062 16.9577 10.0413C17.4984 10.5819 18.1679 10.9759 18.903 11.186L21.75 12L18.904 12.813C18.1689 13.0231 17.4994 13.4171 16.9587 13.9577C16.4181 14.4984 16.0241 15.1679 15.814 15.903L15 18.75L14.187 15.904C13.9769 15.1689 13.5829 14.4994 13.0423 13.9587C12.5016 13.4181 11.8321 13.0241 11.097 12.814L8.25 12L11.096 11.187C11.8311 10.9769 12.5006 10.5829 13.0413 10.0423C13.5819 9.50162 13.9759 8.83214 14.186 8.097L14.187 8.096Z"
            ></path>
            <path
              class="path"
              stroke-linejoin="round"
              stroke-linecap="round"
              stroke="black"
              fill="black"
              d="M6 14.25L5.741 15.285C5.59267 15.8785 5.28579 16.4206 4.85319 16.8532C4.42059 17.2858 3.87853 17.5927 3.285 17.741L2.25 18L3.285 18.259C3.87853 18.4073 4.42059 18.7142 4.85319 19.1468C5.28579 19.5794 5.59267 20.1215 5.741 20.715L6 21.75L6.259 20.715C6.40725 20.1216 6.71398 19.5796 7.14639 19.147C7.5788 18.7144 8.12065 18.4075 8.714 18.259L9.75 18L8.714 17.741C8.12065 17.5925 7.5788 17.2856 7.14639 16.853C6.71398 16.4204 6.40725 15.8784 6.259 15.285L6 14.25Z"
            ></path>
            <path
              class="path"
              stroke-linejoin="round"
              stroke-linecap="round"
              stroke="black"
              fill="black"
              d="M6.5 4L6.303 4.5915C6.24777 4.75718 6.15472 4.90774 6.03123 5.03123C5.90774 5.15472 5.75718 5.24777 5.5915 5.303L5 5.5L5.5915 5.697C5.75718 5.75223 5.90774 5.84528 6.03123 5.96877C6.15472 6.09226 6.24777 6.24282 6.303 6.4085L6.5 7L6.697 6.4085C6.75223 6.24282 6.84528 6.09226 6.96877 5.96877C7.09226 5.84528 7.24282 5.75223 7.4085 5.697L8 5.5L7.4085 5.303C7.24282 5.24777 7.09226 5.15472 6.96877 5.03123C6.84528 4.90774 6.75223 4.75718 6.697 4.5915L6.5 4Z"
            ></path>
          </svg>
          <span class="text_button">{"Generate Site"}</span>
        </button>




                                 /* From Uiverse.io by Quezaquo */
                                <div class="pf-container1">
                                  <span class="pf-hover-me1">{"Hover me !"}</span>
                                  <div class="pf-tooltip1">
                                    <p>{"Heyy👋"}</p>
                                  </div>
                                </div>


                                /* From Uiverse.io by PriyanshuGupta28 */
                        <div class="checkbox-wrapper">
                          <input checked=false type="checkbox" class="check" id="check1-61" />
                          <label for="check1-61" class="label">
                            <svg width="45" height="45" viewBox="0 0 95 95">
                              <rect x="30" y="20" width="50" height="50" stroke="black" fill="none"></rect>
                              <g transform="translate(0,-952.36222)">
                                <path d="m 56,963 c -102,122 6,9 7,9 17,-5 -66,69 -38,52 122,-77 -7,14 18,4 29,-11 45,-43 23,-4" stroke="black" stroke-width="3" fill="none" class="path1"></path>
                              </g>
                            </svg>
                            <span>{"Checkbox"}</span>
                          </label>
                        </div>


                    /* From Uiverse.io by Artahs */
                    <ul class="example-2">
                        <li class="icon-content">
                    <a href="https://www.github.com/" aria-label="GitHub" data-social="github">
                      <div class="filled"></div>
                      <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="16"
                        height="16"
                        fill="currentColor"
                        class="bi bi-github"
                        viewBox="0 0 16 16"
                //        xml:space="preserve"
                      >
                        <path
                          d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27s1.36.09 2 .27c1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8"
                          fill="currentColor"
                        ></path>
                      </svg>
                    </a>
                    <div class="tooltip">{"GitHub"}</div>
                  </li>

                      <li class="icon-content">
                        <a href="https://discord.com/" aria-label="Discord" data-social="discord">
                          <div class="filled"></div>
                          <svg
                            viewBox="0 0 16 16"
                            class="bi bi-discord"
                            fill="currentColor"
                            height="16"
                            width="16"
                            xmlns="http://www.w3.org/2000/svg"
                          >
                            <path
                              d="M13.545 2.907a13.2 13.2 0 0 0-3.257-1.011.05.05 0 0 0-.052.025c-.141.25-.297.577-.406.833a12.2 12.2 0 0 0-3.658 0 8 8 0 0 0-.412-.833.05.05 0 0 0-.052-.025c-1.125.194-2.22.534-3.257 1.011a.04.04 0 0 0-.021.018C.356 6.024-.213 9.047.066 12.032q.003.022.021.037a13.3 13.3 0 0 0 3.995 2.02.05.05 0 0 0 .056-.019q.463-.63.818-1.329a.05.05 0 0 0-.01-.059l-.018-.011a9 9 0 0 1-1.248-.595.05.05 0 0 1-.02-.066l.015-.019q.127-.095.248-.195a.05.05 0 0 1 .051-.007c2.619 1.196 5.454 1.196 8.041 0a.05.05 0 0 1 .053.007q.121.1.248.195a.05.05 0 0 1-.004.085 8 8 0 0 1-1.249.594.05.05 0 0 0-.03.03.05.05 0 0 0 .003.041c.24.465.515.909.817 1.329a.05.05 0 0 0 .056.019 13.2 13.2 0 0 0 4.001-2.02.05.05 0 0 0 .021-.037c.334-3.451-.559-6.449-2.366-9.106a.03.03 0 0 0-.02-.019m-8.198 7.307c-.789 0-1.438-.724-1.438-1.612s.637-1.613 1.438-1.613c.807 0 1.45.73 1.438 1.613 0 .888-.637 1.612-1.438 1.612m5.316 0c-.788 0-1.438-.724-1.438-1.612s.637-1.613 1.438-1.613c.807 0 1.451.73 1.438 1.613 0 .888-.631 1.612-1.438 1.612"
                            ></path>
                          </svg>
                        </a>
                        <div class="tooltip">{"Discord"}</div>
                      </li>
                      <li class="icon-content">
                        <a
                          href="https://store.steampowered.com/"
                          aria-label="Steam"
                          data-social="steam"
                        >
                          <div class="filled"></div>
                          <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="16"
                            height="16"
                            fill="currentColor"
                            class="bi bi-steam"
                            viewBox="0 0 16 16"
                          >
                            <path
                              d="M.329 10.333A8.01 8.01 0 0 0 7.99 16C12.414 16 16 12.418 16 8s-3.586-8-8.009-8A8.006 8.006 0 0 0 0 7.468l.003.006 4.304 1.769A2.2 2.2 0 0 1 5.62 8.88l1.96-2.844-.001-.04a3.046 3.046 0 0 1 3.042-3.043 3.046 3.046 0 0 1 3.042 3.043 3.047 3.047 0 0 1-3.111 3.044l-2.804 2a2.223 2.223 0 0 1-3.075 2.11 2.22 2.22 0 0 1-1.312-1.568L.33 10.333Z"
                            ></path>
                            <path
                              d="M4.868 12.683a1.715 1.715 0 0 0 1.318-3.165 1.7 1.7 0 0 0-1.263-.02l1.023.424a1.261 1.261 0 1 1-.97 2.33l-.99-.41a1.7 1.7 0 0 0 .882.84Zm3.726-6.687a2.03 2.03 0 0 0 2.027 2.029 2.03 2.03 0 0 0 2.027-2.029 2.03 2.03 0 0 0-2.027-2.027 2.03 2.03 0 0 0-2.027 2.027m2.03-1.527a1.524 1.524 0 1 1-.002 3.048 1.524 1.524 0 0 1 .002-3.048"
                            ></path>
                          </svg>
                        </a>
                        <div class="tooltip">{"Steam"}</div>
                      </li>
                      <li class="icon-content">
                        <a
                          href="https://www.instagram.com/"
                          aria-label="Instagram"
                          data-social="instagram"
                        >
                          <div class="filled"></div>
                          <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="16"
                            height="16"
                            fill="currentColor"
                            class="bi bi-instagram"
                            viewBox="0 0 16 16"
                           // xml:space="preserve"
                          >
                            <path
                              d="M8 0C5.829 0 5.556.01 4.703.048 3.85.088 3.269.222 2.76.42a3.9 3.9 0 0 0-1.417.923A3.9 3.9 0 0 0 .42 2.76C.222 3.268.087 3.85.048 4.7.01 5.555 0 5.827 0 8.001c0 2.172.01 2.444.048 3.297.04.852.174 1.433.372 1.942.205.526.478.972.923 1.417.444.445.89.719 1.416.923.51.198 1.09.333 1.942.372C5.555 15.99 5.827 16 8 16s2.444-.01 3.298-.048c.851-.04 1.434-.174 1.943-.372a3.9 3.9 0 0 0 1.416-.923c.445-.445.718-.891.923-1.417.197-.509.332-1.09.372-1.942C15.99 10.445 16 10.173 16 8s-.01-2.445-.048-3.299c-.04-.851-.175-1.433-.372-1.941a3.9 3.9 0 0 0-.923-1.417A3.9 3.9 0 0 0 13.24.42c-.51-.198-1.092-.333-1.943-.372C10.443.01 10.172 0 7.998 0zm-.717 1.442h.718c2.136 0 2.389.007 3.232.046.78.035 1.204.166 1.486.275.373.145.64.319.92.599s.453.546.598.92c.11.281.24.705.275 1.485.039.843.047 1.096.047 3.231s-.008 2.389-.047 3.232c-.035.78-.166 1.203-.275 1.485a2.5 2.5 0 0 1-.599.919c-.28.28-.546.453-.92.598-.28.11-.704.24-1.485.276-.843.038-1.096.047-3.232.047s-2.39-.009-3.233-.047c-.78-.036-1.203-.166-1.485-.276a2.5 2.5 0 0 1-.92-.598 2.5 2.5 0 0 1-.6-.92c-.109-.281-.24-.705-.275-1.485-.038-.843-.046-1.096-.046-3.233s.008-2.388.046-3.231c.036-.78.166-1.204.276-1.486.145-.373.319-.64.599-.92s.546-.453.92-.598c.282-.11.705-.24 1.485-.276.738-.034 1.024-.044 2.515-.045zm4.988 1.328a.96.96 0 1 0 0 1.92.96.96 0 0 0 0-1.92m-4.27 1.122a4.109 4.109 0 1 0 0 8.217 4.109 4.109 0 0 0 0-8.217m0 1.441a2.667 2.667 0 1 1 0 5.334 2.667 2.667 0 0 1 0-5.334"
                              fill="currentColor"
                            ></path>
                          </svg>
                        </a>
                        <div class="tooltip">{"Instagram"}</div>
                      </li>
                    </ul>





                                            <Footer />
                            </div>
                                                </>
                                            )
}
#[function_component(PfHeader)]
pub fn pf_header() -> Html {
    html! {
            <>
                <header>
                    <div class="wrapper">
                        <h1>{ "ABHINANDH S" }<span class="pf-color0">{ "." }</span></h1>
                        <nav>
                            <ul>
                <li>        /* Theme switch button */
            <label class="pf-switch-button2" for="switch">
              <div class="pf-switch-outer2">
                <input id="switch" type="checkbox" />
                <div class="pf-button2">
                  <span class="pf-button-toggle2"></span>
                  <span class="pf-button-indicator2"></span>
                </div>
              </div>
            </label>
    </li>
                                <li><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></li>
                                <li><Link<Route> to={Route::Portfolio}>{ "Portfolio" }</Link<Route>></li>
                                <li><Link<Route> to={Route::ArticlesRoute}>{ "Articles" }</Link<Route>></li>
                                <li><Link<Route> to={Route::Contact}>{ "Contact" }</Link<Route>></li>
                            </ul>
                        </nav>
                    </div>
                </header>
            <div>
                <nav id="mobile-nav">
                    <ul>
                        <li><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></li>
                        <li><Link<Route> to={Route::Portfolio}>{ "Portfolio" }</Link<Route>></li>
                        <li><Link<Route> to={Route::ArticlesRoute}>{ "Articles" }</Link<Route>></li>
                        <li><Link<Route> to={Route::Contact}>{ "Contact" }</Link<Route>></li>
                    </ul>
                </nav>
            </div>
        </>
        }
}

#[function_component(PfFooter)]
pub fn pf_footer() -> Html {
    html! {
        <footer>
            <div class="wrapper">
                <div id="footer-info">
                    <p>{ "Except where otherwise noted, content on this site is licensed under a " }<a href="https://creativecommons.org/publicdomain/zero/1.0/" target="_blank">{ "Creative Commons Zero (CC0) license"}</a></p> // Except where otherwise noted, content on this site is licensed under a <a href="https://creativecommons.org/publicdomain/zero/1.0/">Creative Commons Zero (CC0) license</a>.
                    <a href="https://creativecommons.org/publicdomain/zero/1.0/" target="_blank">
                    <img src="https://licensebuttons.net/p/zero/1.0/88x31.png" alt="CC0" />
                    </a>
                    <p><a href="#">{ "Terms of Service & Privacy" }</a></p>
                </div>
            <div id="footer-links">
                <ul>
                    <li><h5>{ "Links" }</h5></li>
                    <li><a href="https://x.com/Ungraduate_Abhi" target="_blank">{ "X / Twitter" }</a></li>
                    <li><a href="https://github.com/abhi-xyz" target="_blank">{ "Github" }</a></li>
                    <li><a href="https://git.sr.ht/~abhinandh/" target="_blank">{ "Sourcehut" }</a></li>
                </ul>
            </div>
        <div class="clear"></div>
        </div>
    </footer>
    }
}

#[function_component(TestContent)]
pub fn test_content() -> Html {
    html! {
        <>
                    <div id="primary-content">
                        <div class="wrapper">
                            <article>
                                <h3>{ "About me" }</h3>
                                <p>{ "I’ve been writing about some things and other some thing since 2024. I'm a technology enthusiast with a passion for Rust programming and Linux systems. Here, you'll find a variety of content, including tech musings, project updates, and random ideas and discoveries. This space is where I share the less formal, yet intriguing aspects of my work and interests." }</p>
                            </article>
                        </div>
                    </div>



            <div class=".pf_parent_container_button">
       /* From Uiverse.io by MuhammadHasann */
        <button class="pf-button3">
          <div class="pf_dots_border3"></div>
          <span class="text_button">{"Generate Site"}</span>
        </button>
            </div>



        <div id="secondary-content">
            <div class="wrapper">
                    <div class="overlay">
                        <h4>{ "Articles on Rust" }</h4>
                        <p>{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec euismod leo a nibh dignissim tincidunt nam." }</p>
                        <a href="#" class="more_link">{ "Go to index" }</a>
                    </div>
                <div class="clear"></div>
            </div>
        </div>
            <div id="cta">
                <div class="wrapper">
                    <h3>{ "View My Work" }</h3>
                    <p>{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec euismod leo a nibh dignissim tincidunt. Nam ultricies odio ac neque suscipit volutpat. Ut dictum adipiscing felis sed malesuada. Integer porta sem nec nibh hendrerit imperdiet." }</p>
                    <a href="#" class="button-2">{ "Get Started" }</a>
                </div>
            </div>
        </>
    }
}
