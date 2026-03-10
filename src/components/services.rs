use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
struct ServiceCardProps {
    title: String,
    icon: String,
    stagger: u8,
    children: Children,
}

#[function_component(ServiceCard)]
fn service_card(props: &ServiceCardProps) -> Html {
    let stagger_class = format!("stagger-{}", props.stagger);
    html! {
        <div class="col-md-6 mb-4">
            <div class={classes!("service-card", "animate-on-scroll", stagger_class)}>
                <div class="service-icon">
                    <i class={props.icon.clone()}></i>
                </div>
                <h3 class="card-title">{&props.title}</h3>
                { for props.children.iter() }
            </div>
        </div>
    }
}

#[function_component(Services)]
pub fn services() -> Html {
    html! {
        <section id="servicios">
            <div class="container">
                <div class="row">
                    <div class="col-12 text-center mb-5 animate-on-scroll">
                        <span class="section-badge">
                            <i class="bi bi-heart-pulse-fill"></i>
                            {"Atención Especializada"}
                        </span>
                        <h2 class="section-title">{"Nuestros Servicios"}</h2>
                        <p class="section-subtitle">
                            {"Ofrecemos un conjunto completo de servicios de atención médica en el hogar, adaptados a las necesidades individuales de cada paciente."}
                        </p>
                    </div>
                </div>

                <div class="row">
                    <ServiceCard
                        title="Enfermería Profesional"
                        icon="bi bi-clipboard2-pulse-fill"
                        stagger={1}
                    >
                        <ul>
                            <li>{"Curación de Úlceras"}</li>
                            <li>{"Manejo de Dolor"}</li>
                            <li>{"Cuidado Post Quirúrgico"}</li>
                            <li>{"Manejo de Medicamentos"}</li>
                            <li>{"Asistencia de Salud en el Hogar"}</li>
                        </ul>
                    </ServiceCard>

                    <ServiceCard
                        title="Terapias Especializadas"
                        icon="bi bi-person-walking"
                        stagger={2}
                    >
                        <ul>
                            <li>{"Terapia Física"}</li>
                            <li>{"Terapia del Habla"}</li>
                            <li>{"Terapia Ocupacional"}</li>
                            <li>{"Manejo de Dolor Crónico"}</li>
                            <li>{"Rehabilitación Post Quirúrgica"}</li>
                        </ul>
                    </ServiceCard>
                </div>

                <div class="row">
                    <ServiceCard
                        title="Trabajo Social Médico"
                        icon="bi bi-people-fill"
                        stagger={3}
                    >
                        <ul>
                            <li>{"Evaluación de necesidades sociales"}</li>
                            <li>{"Conexión con recursos comunitarios"}</li>
                            <li>{"Apoyo a cuidadores y familiares"}</li>
                            <li>{"Asesoramiento para pacientes"}</li>
                        </ul>
                    </ServiceCard>

                    <ServiceCard
                        title="Nutrición y Dietética"
                        icon="bi bi-heart-fill"
                        stagger={4}
                    >
                        <ul>
                            <li>{"Evaluación nutricional"}</li>
                            <li>{"Planes de alimentación personalizados"}</li>
                            <li>{"Educación sobre dietas especiales"}</li>
                            <li>{"Manejo de condiciones crónicas"}</li>
                        </ul>
                    </ServiceCard>
                </div>
            </div>
        </section>
    }
}
