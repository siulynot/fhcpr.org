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
            <div class="card shadow-sm h-100" style="border-radius:var(--border-radius); border:1px solid var(--gray-200); overflow:hidden; transition:var(--transition);">
                <div class="card-body" style="padding:1.75rem;">
                    <h4 class="region-title">
                        <i class={props.icon.clone()}></i>
                        {&props.title}
                    </h4>
                    <ul class="region-list">
                        {props.cities.iter().map(|city| {
                            html! { <li>{city}</li> }
                        }).collect::<Html>()}
                    </ul>
                </div>
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
                <div class="text-center mb-5">
                    <span class="section-badge">
                        <i class="bi bi-shield-check-fill"></i>
                        {"Elegibilidad y Cobertura"}
                    </span>
                    <h2 class="section-title">{"Requisitos"}</h2>
                </div>

                // Certification intro
                <div class="process-card mb-5">
                    <div class="card-body">
                        <h3 class="sub-heading mb-3">
                            <i class="bi bi-award-fill me-2" style="color:var(--teal);"></i>
                            {"Servicios de Salud en el Hogar"}
                        </h3>
                        <p class="lead mb-0" style="color:var(--text-secondary);">
                            {r#"First Home Care Center, Inc. es una agencia sin fines de lucro certificada por el Departamento de Salud del Estado Libre Asociado de Puerto Rico y por el Seguro Social Federal. Nuestro propósito es proveer servicios de salud de calidad en el hogar o lugar de residencia a pacientes beneficiarios de "Medicare" y "Medicare Advantage", que cumplan con los requisitos establecidos."#}
                        </p>
                    </div>
                </div>

                // How to get services — step cards
                <section class="mb-5">
                    <h3 class="sub-heading mb-4">{"¿Cómo obtener nuestros servicios?"}</h3>
                    <div class="row g-3">
                        <div class="col-md-4">
                            <div class="d-flex align-items-start gap-3 p-3 bg-white rounded-3 border h-100" style="border-color:var(--gray-200) !important; transition:var(--transition);">
                                <div style="width:44px;height:44px;background:var(--gradient-primary);border-radius:50%;display:flex;align-items:center;justify-content:center;color:white;font-weight:800;font-size:1.1rem;flex-shrink:0;font-family:var(--font-display);">
                                    {"1"}
                                </div>
                                <div>
                                    <h5 style="font-size:0.9375rem;font-weight:700;color:var(--text-primary);margin-bottom:0.375rem;">{"Referido Médico"}</h5>
                                    <p style="font-size:0.875rem;margin:0;">{"Su médico de cabecera lo refiere a nuestra agencia si considera que amerita cualquiera de nuestros servicios."}</p>
                                </div>
                            </div>
                        </div>
                        <div class="col-md-4">
                            <div class="d-flex align-items-start gap-3 p-3 bg-white rounded-3 border h-100" style="border-color:var(--gray-200) !important; transition:var(--transition);">
                                <div style="width:44px;height:44px;background:var(--gradient-primary);border-radius:50%;display:flex;align-items:center;justify-content:center;color:white;font-weight:800;font-size:1.1rem;flex-shrink:0;font-family:var(--font-display);">
                                    {"2"}
                                </div>
                                <div>
                                    <h5 style="font-size:0.9375rem;font-weight:700;color:var(--text-primary);margin-bottom:0.375rem;">{"Evaluación Inicial"}</h5>
                                    <p style="font-size:0.875rem;margin:0;">{"La agencia realiza una visita de evaluación para determinar si es elegible para nuestros servicios y suple los formularios necesarios."}</p>
                                </div>
                            </div>
                        </div>
                        <div class="col-md-4">
                            <div class="d-flex align-items-start gap-3 p-3 bg-white rounded-3 border h-100" style="border-color:var(--gray-200) !important; transition:var(--transition);">
                                <div style="width:44px;height:44px;background:var(--gradient-primary);border-radius:50%;display:flex;align-items:center;justify-content:center;color:white;font-weight:800;font-size:1.1rem;flex-shrink:0;font-family:var(--font-display);">
                                    {"3"}
                                </div>
                                <div>
                                    <h5 style="font-size:0.9375rem;font-weight:700;color:var(--text-primary);margin-bottom:0.375rem;">{"Inicio de Servicios"}</h5>
                                    <p style="font-size:0.875rem;margin:0;">{r#"Para calificar, debe cumplir con los requisitos de CMS. En casos de "Medicare Advantage" se solicita aprobación del plan médico."#}</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                // Services offered
                <section class="mb-5">
                    <div class="process-card">
                        <div class="card-body">
                            <h3 class="sub-heading mb-3">
                                <i class="bi bi-list-check me-2" style="color:var(--teal);"></i>
                                {"¿Qué servicios ofrecemos?"}
                            </h3>
                            <div class="row">
                                <div class="col-md-6">
                                    <ul style="list-style:none;padding:0;margin:0;">
                                        <li class="d-flex align-items-center gap-2 mb-2" style="font-size:0.9375rem;color:var(--text-secondary);">
                                            <span style="width:8px;height:8px;border-radius:50%;background:var(--gradient-primary);flex-shrink:0;"></span>
                                            {"Enfermería Profesional"}
                                        </li>
                                        <li class="d-flex align-items-center gap-2 mb-2" style="font-size:0.9375rem;color:var(--text-secondary);">
                                            <span style="width:8px;height:8px;border-radius:50%;background:var(--gradient-primary);flex-shrink:0;"></span>
                                            {"Asistencia de Salud"}
                                        </li>
                                        <li class="d-flex align-items-center gap-2 mb-2" style="font-size:0.9375rem;color:var(--text-secondary);">
                                            <span style="width:8px;height:8px;border-radius:50%;background:var(--gradient-primary);flex-shrink:0;"></span>
                                            {"Terapia Física, Ocupacional y del Habla"}
                                        </li>
                                    </ul>
                                </div>
                                <div class="col-md-6">
                                    <ul style="list-style:none;padding:0;margin:0;">
                                        <li class="d-flex align-items-center gap-2 mb-2" style="font-size:0.9375rem;color:var(--text-secondary);">
                                            <span style="width:8px;height:8px;border-radius:50%;background:var(--gradient-primary);flex-shrink:0;"></span>
                                            {"Trabajo Médico Social"}
                                        </li>
                                        <li class="d-flex align-items-center gap-2 mb-2" style="font-size:0.9375rem;color:var(--text-secondary);">
                                            <span style="width:8px;height:8px;border-radius:50%;background:var(--gradient-primary);flex-shrink:0;"></span>
                                            {"Consultoría de Nutrición"}
                                        </li>
                                        <li class="d-flex align-items-center gap-2 mb-2" style="font-size:0.9375rem;color:var(--text-secondary);">
                                            <span style="width:8px;height:8px;border-radius:50%;background:var(--gradient-primary);flex-shrink:0;"></span>
                                            {"Coordinación de Equipo Médico"}
                                        </li>
                                    </ul>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                // Insurance section
                <section class="mb-5">
                    <h3 class="sub-heading mb-3">
                        <i class="bi bi-credit-card-2-front-fill me-2" style="color:var(--teal);"></i>
                        {"¿Quién paga por nuestros servicios?"}
                    </h3>
                    <div class="process-card mb-4">
                        <div class="card-body">
                            <p class="mb-0" style="color:var(--text-secondary);">
                                {r#"Cualquiera de los planes "Medicare" o "Medicare Advantage" pagan o ayudan a pagar los servicios ofrecidos a sus beneficiarios. Nosotros nos encargamos de facturar directamente a estos planes."#}
                            </p>
                        </div>
                    </div>
                    <h4 class="sub-heading mb-3" style="font-size:1rem;">{"Planes médicos aceptados:"}</h4>
                    <div class="insurance-grid">
                        {insurances.into_iter().map(|(logo, name)| {
                            html! {
                                <InsuranceCard logo={logo.to_string()} name={name.to_string()} />
                            }
                        }).collect::<Html>()}
                    </div>
                </section>

                // Coverage regions
                <section>
                    <h3 class="sub-heading mb-4">
                        <i class="bi bi-geo-alt-fill me-2" style="color:var(--teal);"></i>
                        {"Áreas de Cobertura"}
                    </h3>
                    <div class="row">
                        <RegionCard title={"Región Oeste".to_string()} icon={"bi bi-compass-fill".to_string()} cities={oeste} />
                        <RegionCard title={"Región Sur".to_string()}  icon={"bi bi-sun-fill".to_string()}     cities={sur}   />
                        <RegionCard title={"Región Norte".to_string()} icon={"bi bi-cloud-fill".to_string()}  cities={norte} />
                    </div>
                </section>
            </div>
        </div>
    }
}
