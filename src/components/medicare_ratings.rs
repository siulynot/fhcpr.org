use yew::prelude::*;

/// Renders 5 stars given a rating 0.0–5.0, supporting half stars.
fn star_row(rating: f32) -> Html {
    let mut stars: Vec<Html> = Vec::with_capacity(5);
    for i in 1..=5 {
        let f = i as f32;
        let icon = if rating >= f {
            "bi bi-star-fill"
        } else if rating >= f - 0.5 {
            "bi bi-star-half"
        } else {
            "bi bi-star"
        };
        stars.push(html! { <i class={icon}></i> });
    }
    html! {
        <div class="mc-stars" aria-label={format!("{} de 5 estrellas", rating)}>
            { for stars }
        </div>
    }
}

#[function_component(MedicareRatings)]
pub fn medicare_ratings() -> Html {
    // Datos oficiales — Medicare Care Compare, CCN 407020 (First Home Care Center)
    let quality_rating: f32 = 4.5;
    let survey_rating:  f32 = 4.0;

    html! {
        <section id="medicare-ratings" class="medicare-ratings-section">
            <div class="container">

                // Section header
                <div class="text-center mb-5 animate-on-scroll">
                    <span class="section-badge">
                        <i class="bi bi-award-fill"></i>
                        {"Calificación Oficial de Medicare"}
                    </span>
                    <h2 class="section-title">{"Reconocidos por Medicare Care Compare"}</h2>
                    <p class="section-subtitle">
                        {"Nuestras calificaciones provienen del programa oficial "}
                        <strong>{"Care Compare"}</strong>
                        {" de los "}
                        <em>{"Centers for Medicare & Medicaid Services"}</em>
                        {" (CMS) — una evaluación independiente basada en datos clínicos y encuestas a pacientes reales."}
                    </p>
                </div>

                // Two rating cards side by side
                <div class="row g-4 justify-content-center">

                    // Card 1 — Quality of patient care
                    <div class="col-md-6 col-lg-5 animate-on-scroll stagger-1">
                        <div class="mc-rating-card mc-rating-card--quality h-100">
                            <div class="mc-rating-card-header">
                                <div class="mc-rating-icon">
                                    <i class="bi bi-heart-pulse-fill"></i>
                                </div>
                                <div>
                                    <div class="mc-rating-eyebrow">{"Calidad del Cuidado al Paciente"}</div>
                                    <div class="mc-rating-label">{"Quality of Patient Care"}</div>
                                </div>
                            </div>
                            <div class="mc-rating-body">
                                { star_row(quality_rating) }
                                <div class="mc-rating-score">
                                    <span class="mc-rating-number">{"4.5"}</span>
                                    <span class="mc-rating-out">{"/ 5"}</span>
                                </div>
                                <p class="mc-rating-desc">
                                    {"Mide qué tan efectivamente tratamos a nuestros pacientes con base en resultados clínicos: mejoría en movilidad, manejo de dolor, recuperación de hospitalizaciones y más."}
                                </p>
                            </div>
                        </div>
                    </div>

                    // Card 2 — Patient survey
                    <div class="col-md-6 col-lg-5 animate-on-scroll stagger-2">
                        <div class="mc-rating-card mc-rating-card--survey h-100">
                            <div class="mc-rating-card-header">
                                <div class="mc-rating-icon">
                                    <i class="bi bi-chat-heart-fill"></i>
                                </div>
                                <div>
                                    <div class="mc-rating-eyebrow">{"Encuesta a Pacientes (HHCAHPS)"}</div>
                                    <div class="mc-rating-label">{"Patient Survey Rating"}</div>
                                </div>
                            </div>
                            <div class="mc-rating-body">
                                { star_row(survey_rating) }
                                <div class="mc-rating-score">
                                    <span class="mc-rating-number">{"4.0"}</span>
                                    <span class="mc-rating-out">{"/ 5"}</span>
                                </div>
                                <p class="mc-rating-desc">
                                    {"Calificación otorgada directamente por nuestros pacientes — qué tan bien comunicamos, qué tan profesional es nuestro equipo, y si nos recomendarían a sus seres queridos."}
                                </p>
                            </div>
                        </div>
                    </div>

                </div>

                // Verification strip
                <div class="mc-verify-strip animate-on-scroll stagger-3">
                    <div class="mc-verify-content">
                        <div class="mc-verify-item">
                            <i class="bi bi-shield-fill-check"></i>
                            <div>
                                <strong>{"CMS Certification Number"}</strong>
                                <span>{"407020"}</span>
                            </div>
                        </div>
                        <div class="mc-verify-divider"></div>
                        <div class="mc-verify-item">
                            <i class="bi bi-calendar-check-fill"></i>
                            <div>
                                <strong>{"Certificados por Medicare desde"}</strong>
                                <span>{"1984"}</span>
                            </div>
                        </div>
                        <div class="mc-verify-divider"></div>
                        <div class="mc-verify-item">
                            <i class="bi bi-geo-alt-fill"></i>
                            <div>
                                <strong>{"Sede"}</strong>
                                <span>{"Mayagüez, Puerto Rico"}</span>
                            </div>
                        </div>
                    </div>
                    <a
                        href="https://www.medicare.gov/care-compare/details/home-health/407020?city=Mayag%C3%BCez&state=PR&zipcode=00681"
                        class="mc-verify-cta"
                        target="_blank"
                        rel="noopener noreferrer"
                        aria-label="Ver perfil completo en Medicare.gov (abre en pestaña nueva)"
                    >
                        <i class="bi bi-box-arrow-up-right me-2"></i>
                        {"Verificar en Medicare.gov"}
                    </a>
                </div>

                // Services note — affirmative listing, no contradiction
                <div class="mc-services-note animate-on-scroll stagger-4">
                    <div class="mc-services-note-icon">
                        <i class="bi bi-info-circle-fill"></i>
                    </div>
                    <div class="mc-services-note-body">
                        <strong>{"Nota sobre nuestros servicios:"}</strong>
                        {" Nuestro plan integral de cuidado en el hogar incluye "}
                        <strong>{"Enfermería"}</strong>{", "}
                        <strong>{"Terapia Física"}</strong>{", "}
                        <strong>{"Terapia Ocupacional"}</strong>{", "}
                        <strong>{"Terapia del Habla"}</strong>{", "}
                        <strong>{"Trabajo Social Médico"}</strong>{", "}
                        <strong>{"Consultoría de Nutrición"}</strong>
                        {" y "}
                        <strong>{"Asistencia de Salud en el Hogar (Home Health Aide)"}</strong>
                        {". Si tu plan de cuidado lo requiere, coordinamos cualquier servicio cubierto. "}
                        <a href="tel:+17878342295" class="mc-inline-cta">
                            <i class="bi bi-telephone-fill me-1"></i>
                            {"Llámanos al 787-834-2295"}
                        </a>
                        {" para confirmar tu elegibilidad."}
                    </div>
                </div>

            </div>
        </section>
    }
}
