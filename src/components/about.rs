use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <>
            // ===== Stats Section =====
            <div class="stats-section">
                <div class="container">
                    <div class="row text-center align-items-center">
                        <div class="col-6 col-md-3 stat-card animate-on-scroll stagger-1">
                            <div class="stat-number">{"42"}</div>
                            <div class="stat-label">{"Municipios"}</div>
                        </div>
                        <div class="d-none d-md-block col-md-auto stat-sep" style="height:80px;"></div>
                        <div class="col-6 col-md-3 stat-card animate-on-scroll stagger-2">
                            <div class="stat-number">{"5"}</div>
                            <div class="stat-label">{"Especialidades"}</div>
                        </div>
                        <div class="d-none d-md-block col-md-auto stat-sep" style="height:80px;"></div>
                        <div class="col-6 col-md-3 stat-card animate-on-scroll stagger-3">
                            <div class="stat-number">{"7"}</div>
                            <div class="stat-label">{"Planes Aceptados"}</div>
                        </div>
                        <div class="d-none d-md-block col-md-auto stat-sep" style="height:80px;"></div>
                        <div class="col-6 col-md-3 stat-card animate-on-scroll stagger-4">
                            <div class="stat-number">{"3"}</div>
                            <div class="stat-label">{"Regiones Cubiertas"}</div>
                        </div>
                    </div>
                </div>
            </div>

            // ===== About Section =====
            <section id="sobre-nosotros" class="py-5">
                <div class="container">
                    <div class="row">
                        <div class="col-12 text-center mb-5">
                            <span class="section-badge" style="background:rgba(255,255,255,0.15);color:rgba(255,255,255,0.9);border-color:rgba(255,255,255,0.25);">
                                <i class="bi bi-building-heart-fill"></i>
                                {"Sin Fines de Lucro · Certificados"}
                            </span>
                            <h2 class="section-title" style="color:white;">{"Sobre Nosotros"}</h2>
                            <div class="section-divider"></div>
                        </div>
                    </div>

                    <div class="row mb-5">
                        <div class="col-md-10 mx-auto">
                            <div class="about-card animate-on-scroll">
                                <div class="about-card-inner">
                                    <h3 class="about-subtitle">{"Nuestra Misión"}</h3>
                                    <p class="about-text lead">
                                        {"Bienvenidos a First Home Care Center, una organización sin ánimo de lucro, dedicada al bienestar de nuestra comunidad. Contamos con la acreditación del Departamento de Salud del Estado Libre Asociado de Puerto Rico y del Seguro Social Federal, reflejando nuestro compromiso con los más altos estándares de cuidado."}
                                    </p>
                                    <p class="about-text">
                                        {r#"Nuestro principal objetivo es brindar servicios de atención médica de alta calidad en el hogar o lugar de residencia a los pacientes beneficiarios de "Medicare" y "Medicare Advantage", siempre respetando y cumpliendo con los requisitos establecidos."#}
                                    </p>
                                    <div class="about-quote">
                                        <blockquote>
                                            {"Creemos firmemente que cada paciente merece recibir una atención personalizada y de calidad en la comodidad de su hogar."}
                                        </blockquote>
                                    </div>
                                    <p class="about-text">
                                        {"Nuestro equipo de profesionales de la salud está comprometido a proporcionar un servicio excelente y un trato humano a cada uno de nuestros pacientes."}
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>

                    // ===== Contact Section =====
                    <div id="contacto" class="row g-4">
                        <div class="col-md-6">
                            <div class="contact-card animate-on-scroll stagger-1">
                                <div class="contact-card-header">
                                    <h3>
                                        <i class="bi bi-telephone-fill me-2"></i>
                                        {"Información de Contacto"}
                                    </h3>
                                </div>
                                <div class="card-body">
                                    <address class="contact-info">
                                        <div class="contact-item">
                                            <i class="bi bi-building"></i>
                                            <div>
                                                <strong style="color:var(--text-primary);">{"First Home Care Center, Inc."}</strong><br />
                                                {"#96 Oeste, Calle de la Candelaria"}<br />
                                                {"Mayagüez, 00680"}
                                            </div>
                                        </div>
                                        <div class="contact-item">
                                            <i class="bi bi-telephone-fill"></i>
                                            <div>
                                                {"Teléfono: "}<a href="tel:+17878342295" class="contact-link">{"787-834-2295"}</a>
                                            </div>
                                        </div>
                                        <div class="contact-item">
                                            <i class="bi bi-printer-fill"></i>
                                            <div>{"Fax: 787-265-7090"}</div>
                                        </div>
                                        <div class="contact-item">
                                            <i class="bi bi-envelope-fill"></i>
                                            <div>
                                                {"Email: "}<a href="mailto:admin@fhcpr.org" class="contact-link">{"admin@fhcpr.org"}</a>
                                            </div>
                                        </div>
                                    </address>

                                    <div class="hours-container">
                                        <div class="hours-title">
                                            <i class="bi bi-clock-fill"></i>
                                            {"Horario de Atención"}
                                        </div>
                                        <div class="hours-item">
                                            <span class="hours-day">{"Lunes a Viernes"}</span>
                                            <span class="hours-time">{"8:00 AM – 5:00 PM"}</span>
                                        </div>
                                        <div class="hours-item">
                                            <span class="hours-day">{"Sábados y Domingos"}</span>
                                            <span class="hours-time">{"Cerrado"}</span>
                                        </div>
                                    </div>
                                </div>

                                <div class="contact-buttons">
                                    <a href="tel:+17878342295" class="btn btn-primary contact-btn">
                                        <i class="bi bi-telephone-fill"></i>{"Llamar Ahora"}
                                    </a>
                                    <a href="mailto:admin@fhcpr.org" class="btn btn-outline-primary contact-btn">
                                        <i class="bi bi-envelope-fill"></i>{"Enviar Email"}
                                    </a>
                                </div>
                            </div>
                        </div>

                        <div class="col-md-6">
                            <div class="map-card animate-on-scroll stagger-2">
                                <iframe
                                    src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d3790.15558375304!2d-67.14784742403374!3d18.20277277880308!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x8c02b24a643a98c9%3A0xd2d508cf8063a935!2sFirst%20Home%20Care%20Center!5e0!3m2!1sen!2spr!4v1684716821354!5m2!1sen!2spr"
                                    width="100%"
                                    height="100%"
                                    style="border:0; min-height:420px; display:block;"
                                    allowfullscreen=true
                                    loading="lazy"
                                    referrerpolicy="no-referrer-when-downgrade"
                                    title="First Home Care Center Location"
                                ></iframe>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </>
    }
}
