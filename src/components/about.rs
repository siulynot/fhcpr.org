use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <section id="sobre-nosotros" class="py-5" style="background-color: #5c9abe;">
            <div class="container text-white animate-on-scroll">
                <div class="row">
                    <div class="col-12 text-center mb-5">
                        <h2 class="section-title text-white">{"Sobre Nosotros"}</h2>
                        <div class="section-divider"></div>
                    </div>
                </div>
                
                <div class="row mb-5">
                    <div class="col-md-10 mx-auto">
                        <div class="about-card">
                            <div class="about-card-inner">
                                <h3 class="about-subtitle">{"Nuestra Misión"}</h3>
                                <p class="lead about-text">
                                    {"Bienvenidos a First Home Care, una organización sin ánimo de lucro, dedicada al bienestar de nuestra comunidad. Contamos con la acreditación del Departamento de Salud del Estado Libre Asociado de Puerto Rico y del Seguro Social Federal, reflejando nuestro compromiso con los más altos estándares de cuidado."}
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
                
                <div id="contacto" class="row">
                    <div class="col-md-6 mb-4">
                        <div class="card contact-card shadow h-100">
                            <div class="card-body">
                                <h3 class="card-title text-dark text-center">{"Contacto"}</h3>
                                <div class="contact-divider"></div>
                                <address class="text-dark contact-info">
                                    <div class="contact-item">
                                        <i class="bi bi-building"></i>
                                        <div>
                                            <strong>{"First Home Care Center, Inc."}</strong><br />
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
                                        <div>
                                            {"Fax: 787-265-7090"}
                                        </div>
                                    </div>
                                    <div class="contact-item">
                                        <i class="bi bi-envelope-fill"></i>
                                        <div>
                                            {"Email: "}<a href="mailto:admin@fhcpr.org" class="contact-link">{"admin@fhcpr.org"}</a>
                                        </div>
                                    </div>
                                </address>
                                
                                <div class="hours-container">
                                    <h4 class="text-dark text-center mt-4">{"Horario de Atención"}</h4>
                                    <div class="hours-divider"></div>
                                    <div class="hours-item">
                                        <span class="hours-day">{"Lunes a Viernes:"}</span>
                                        <span class="hours-time">{"8:00 AM - 5:00 PM"}</span>
                                    </div>
                                    <div class="hours-item">
                                        <span class="hours-day">{"Sábados y Domingos:"}</span>
                                        <span class="hours-time">{"Cerrado"}</span>
                                    </div>
                                </div>
                                
                                <div class="contact-buttons mt-4">
                                    <a href="tel:+17878342295" class="btn btn-primary contact-btn me-2">
                                        <i class="bi bi-telephone-fill me-2"></i>{"Llamar Ahora"}
                                    </a>
                                    <a href="mailto:admin@fhcpr.org" class="btn btn-outline-primary contact-btn">
                                        <i class="bi bi-envelope-fill me-2"></i>{"Enviar Email"}
                                    </a>
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    <div class="col-md-6 mb-4">
                        <div class="card map-card shadow h-100">
                            <div class="card-body p-0">
                                <iframe
                                    src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d3790.15558375304!2d-67.14784742403374!3d18.20277277880308!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x8c02b24a643a98c9%3A0xd2d508cf8063a935!2sFirst%20Home%20Care%20Center!5e0!3m2!1sen!2spr!4v1684716821354!5m2!1sen!2spr"
                                    width="100%"
                                    height="100%"
                                    style="border:0; min-height: 350px;"
                                    allowfullscreen=true
                                    loading="lazy"
                                    referrerpolicy="no-referrer-when-downgrade"
                                    title="First Home Care Center Location"
                                ></iframe>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
