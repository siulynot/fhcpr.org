use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
struct RegionCardProps {
    title: String,
    icon: String,
    cities: Vec<String>,
}

#[function_component(RegionCard)]
fn region_card(props: &RegionCardProps) -> Html {
    html! {
        <div class="col-md-4 mb-4">
            <div class="req-region-card h-100">
                <h4 class="region-title">
                    <i class={props.icon.clone()}></i>
                    {&props.title}
                </h4>
                <ul class="region-list">
                    {props.cities.iter().map(|city| html! { <li>{city}</li> }).collect::<Html>()}
                </ul>
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct InsuranceCardProps {
    logo: String,
    name: String,
}

#[function_component(InsuranceCard)]
fn insurance_card(props: &InsuranceCardProps) -> Html {
    html! {
        <div class="insurance-card">
            <img src={props.logo.clone()} alt={props.name.clone()} />
        </div>
    }
}

#[function_component(Requirements)]
pub fn requirements() -> Html {
    let oeste = vec![
        "Aguada", "Aguadilla", "Añasco", "Cabo Rojo", "Hormigueros",
        "Isabela", "Lajas", "Las Marías", "Maricao", "Mayagüez",
        "Moca", "Rincón", "Sabana Grande", "San Germán", "San Sebastián",
    ].into_iter().map(String::from).collect::<Vec<_>>();

    let sur = vec![
        "Adjuntas", "Arroyo", "Coamo", "Guayama", "Guayanilla",
        "Guánica", "Jayuya", "Juana Díaz", "Patillas", "Ponce",
        "Peñuelas", "Salinas", "Santa Isabel", "Villalba", "Yauco",
    ].into_iter().map(String::from).collect::<Vec<_>>();

    let norte = vec![
        "Arecibo", "Barceloneta", "Camuy", "Ciales", "Florida",
        "Hatillo", "Lares", "Manatí", "Morovis", "Quebradillas",
        "Utuado", "Vega Baja",
    ].into_iter().map(String::from).collect::<Vec<_>>();

    let insurances = vec![
        ("images/mmm.png",      "MMM"),
        ("images/mcs.png",      "MCS"),
        ("images/ssbv.png",     "SSBV"),
        ("images/medicare.png", "Medicare"),
        ("images/humana.png",   "Humana"),
        ("images/pmc.png",      "PMC"),
        ("images/prossam.png",  "Prossam"),
    ];

    html! {
        <div id="requisitos">
            <div class="container requirements-section animate-on-scroll">

                // Section header
                <div class="text-center mb-5">
                    <span class="section-badge">
                        <i class="bi bi-shield-check-fill"></i>
                        {"Elegibilidad y Cobertura"}
                    </span>
                    <h2 class="section-title">{"¿Cómo Funciona?"}</h2>
                    <p class="section-subtitle">
                        {"Todo lo que necesitas saber para empezar a recibir atención médica en tu hogar."}
                    </p>
                </div>

                // ── 3-column answer cards ──────────────────────────────
                <div class="row g-4 mb-5">

                    // Card 1 — Who qualifies
                    <div class="col-lg-4 animate-on-scroll stagger-1">
                        <div class="answer-card h-100">
                            <div class="answer-card-icon" style="background:linear-gradient(135deg,#D6EFF5,#EBF8F5);">
                                <i class="bi bi-person-check-fill" style="color:var(--primary);"></i>
                            </div>
                            <h3 class="answer-card-title">{"¿Quién Califica?"}</h3>
                            <ul class="answer-list">
                                <li>
                                    <i class="bi bi-check-circle-fill"></i>
                                    {"Beneficiarios de Medicare (Original)"}
                                </li>
                                <li>
                                    <i class="bi bi-check-circle-fill"></i>
                                    {"Beneficiarios de Medicare Advantage"}
                                </li>
                                <li>
                                    <i class="bi bi-check-circle-fill"></i>
                                    {"Pacientes con referido de su médico"}
                                </li>
                                <li>
                                    <i class="bi bi-check-circle-fill"></i>
                                    {"Residentes en nuestras áreas de servicio"}
                                </li>
                            </ul>
                            <div class="answer-card-footer">
                                <i class="bi bi-info-circle me-1"></i>
                                {"Tu médico determina si calificas durante tu próxima visita."}
                            </div>
                        </div>
                    </div>

                    // Card 2 — What's covered
                    <div class="col-lg-4 animate-on-scroll stagger-2">
                        <div class="answer-card answer-card--featured h-100">
                            <div class="answer-card-icon" style="background:rgba(255,255,255,0.2);">
                                <i class="bi bi-heart-pulse-fill" style="color:white;"></i>
                            </div>
                            <h3 class="answer-card-title">{"¿Qué Cubre Tu Seguro?"}</h3>
                            <ul class="answer-list">
                                <li>
                                    <i class="bi bi-check-circle-fill"></i>
                                    {"Enfermería Profesional"}
                                </li>
                                <li>
                                    <i class="bi bi-check-circle-fill"></i>
                                    {"Terapia Física, Ocupacional y del Habla"}
                                </li>
                                <li>
                                    <i class="bi bi-check-circle-fill"></i>
                                    {"Trabajo Médico Social"}
                                </li>
                                <li>
                                    <i class="bi bi-check-circle-fill"></i>
                                    {"Consultoría de Nutrición"}
                                </li>
                                <li>
                                    <i class="bi bi-check-circle-fill"></i>
                                    {"Asistencia de Salud en el Hogar"}
                                </li>
                            </ul>
                            <div class="answer-card-footer answer-card-footer--light">
                                <i class="bi bi-shield-fill-check me-1"></i>
                                {"Sin costo adicional para ti — facturamos directamente a tu plan."}
                            </div>
                        </div>
                    </div>

                    // Card 3 — How to start
                    <div class="col-lg-4 animate-on-scroll stagger-3">
                        <div class="answer-card h-100">
                            <div class="answer-card-icon" style="background:linear-gradient(135deg,#D6EFF5,#EBF8F5);">
                                <i class="bi bi-play-circle-fill" style="color:var(--primary);"></i>
                            </div>
                            <h3 class="answer-card-title">{"¿Cómo Empezar?"}</h3>
                            <div class="answer-steps">
                                <div class="answer-step">
                                    <div class="answer-step-num">{"1"}</div>
                                    <div>
                                        <strong>{"Habla con tu médico"}</strong>
                                        <p>{"Pídele que te refiera a First Home Care Center."}</p>
                                    </div>
                                </div>
                                <div class="answer-step">
                                    <div class="answer-step-num">{"2"}</div>
                                    <div>
                                        <strong>{"Recibe una evaluación"}</strong>
                                        <p>{"Te visitamos en casa para confirmar tu elegibilidad."}</p>
                                    </div>
                                </div>
                                <div class="answer-step">
                                    <div class="answer-step-num">{"3"}</div>
                                    <div>
                                        <strong>{"Comienza tu cuidado"}</strong>
                                        <p>{"Nuestro equipo empieza a visitarte en tu hogar."}</p>
                                    </div>
                                </div>
                            </div>
                            <a href="#contacto" class="answer-card-cta">
                                <i class="bi bi-telephone-fill me-2"></i>{"Contáctanos"}
                            </a>
                        </div>
                    </div>
                </div>

                // ── Insurance logos ────────────────────────────────────
                <div class="req-divider-section animate-on-scroll">
                    <div class="req-divider-label">
                        <i class="bi bi-credit-card-2-front-fill"></i>
                        {"Planes médicos que aceptamos"}
                    </div>
                    <div class="insurance-grid">
                        {insurances.into_iter().map(|(logo, name)| html! {
                            <InsuranceCard logo={logo.to_string()} name={name.to_string()} />
                        }).collect::<Html>()}
                    </div>
                </div>

                // ── Coverage regions ───────────────────────────────────
                <div class="animate-on-scroll mt-5">
                    <div class="req-divider-label mb-4">
                        <i class="bi bi-geo-alt-fill"></i>
                        {"Ofrecemos servicios en 42 municipios de Puerto Rico"}
                    </div>
                    <div class="row">
                        <RegionCard title={"Región Oeste".to_string()} icon={"bi bi-compass-fill".to_string()} cities={oeste} />
                        <RegionCard title={"Región Sur".to_string()}  icon={"bi bi-sun-fill".to_string()}     cities={sur}   />
                        <RegionCard title={"Región Norte".to_string()} icon={"bi bi-cloud-fill".to_string()}  cities={norte} />
                    </div>
                </div>

            </div>
        </div>
    }
}
