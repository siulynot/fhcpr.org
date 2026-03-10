use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    let current_year = 2026;

    html! {
        <footer>
            <div class="container">
                <div class="row g-5">
                    // Brand column
                    <div class="col-lg-4 col-md-12 footer-brand">
                        <div class="d-flex align-items-center gap-3 mb-3">
                            <img src="images/logo.png" alt="First Home Care Center" style="height:48px;width:auto;filter:brightness(1.1);" />
                            <h5 class="mb-0">{"First Home Care Center, Inc."}</h5>
                        </div>
                        <p class="footer-desc">
                            {"Una agencia sin fines de lucro certificada y dedicada a ofrecer servicios de salud de calidad en la comodidad de tu hogar."}
                        </p>
                        <div>
                            <span class="footer-cert-badge">
                                <i class="bi bi-patch-check-fill"></i>
                                {"Dept. Salud PR Certificado"}
                            </span>
                            <span class="footer-cert-badge">
                                <i class="bi bi-shield-fill-check"></i>
                                {"Medicare Certificado"}
                            </span>
                        </div>
                    </div>

                    // Quick links
                    <div class="col-lg-2 col-sm-6">
                        <h5>{"Navegación"}</h5>
                        <ul>
                            <li><a href="#">{"Inicio"}</a></li>
                            <li><a href="#servicios">{"Servicios"}</a></li>
                            <li><a href="#requisitos">{"Requisitos"}</a></li>
                            <li><a href="#sobre-nosotros">{"Sobre Nosotros"}</a></li>
                            <li><a href="#contacto">{"Contacto"}</a></li>
                        </ul>
                    </div>

                    // Services list
                    <div class="col-lg-3 col-sm-6">
                        <h5>{"Servicios"}</h5>
                        <ul>
                            <li><a href="#servicios">{"Enfermería Profesional"}</a></li>
                            <li><a href="#servicios">{"Terapias Especializadas"}</a></li>
                            <li><a href="#servicios">{"Trabajo Social Médico"}</a></li>
                            <li><a href="#servicios">{"Nutrición y Dietética"}</a></li>
                            <li><a href="#servicios">{"Asistencia en el Hogar"}</a></li>
                        </ul>
                    </div>

                    // Contact info
                    <div class="col-lg-3 col-md-12">
                        <h5>{"Contacto"}</h5>
                        <div class="footer-contact-item">
                            <i class="bi bi-geo-alt-fill"></i>
                            <span>{"#96 Oeste, Calle de la Candelaria, Mayagüez, PR 00680"}</span>
                        </div>
                        <div class="footer-contact-item">
                            <i class="bi bi-telephone-fill"></i>
                            <a href="tel:+17878342295" style="color:rgba(255,255,255,0.55);">{"787-834-2295"}</a>
                        </div>
                        <div class="footer-contact-item">
                            <i class="bi bi-envelope-fill"></i>
                            <a href="mailto:admin@fhcpr.org" style="color:rgba(255,255,255,0.55);">{"admin@fhcpr.org"}</a>
                        </div>
                        <div class="footer-contact-item">
                            <i class="bi bi-clock-fill"></i>
                            <span>{"Lun–Vie: 8:00 AM – 5:00 PM"}</span>
                        </div>
                    </div>
                </div>
            </div>

            // Bottom bar
            <div class="footer-bottom-bar">
                <div class="container">
                    <div class="footer-bottom">
                        <span>
                            {format!("© {} First Home Care Center, Inc. Todos los derechos reservados.", current_year)}
                        </span>
                        <div class="footer-legal-links">
                            <a href="#">{"Política de Privacidad"}</a>
                            <a href="#">{"Términos de Servicio"}</a>
                        </div>
                    </div>
                </div>
            </div>
        </footer>
    }
}
