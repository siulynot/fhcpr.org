use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
struct ServiceCardProps {
    title: String,
    children: Children,
}

#[function_component(ServiceCard)]
fn service_card(props: &ServiceCardProps) -> Html {
    html! {
        <div class="col-md-6">
            <div class="card mb-4 shadow animate-on-scroll">
                <div class="card-body">
                    <h3 class="card-title">{&props.title}</h3>
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}

#[function_component(Services)]
pub fn services() -> Html {
    html! {
        <section id="servicios" class="container mt-5 py-5 animate-on-scroll">
            <div class="row">
                <div class="col-12 text-center mb-5">
                    <h2 class="section-title">{"Servicios"}</h2>
                    <p class="lead text-muted">{"Ofrecemos un conjunto completo de servicios de atención médica en el hogar para satisfacer sus necesidades."}</p>
                </div>
            </div>
            
            <div class="row">
                <ServiceCard title="Enfermería">
                    <ul>
                        <li>{"Curación de Úlceras"}</li>
                        <li>{"Manejo de Dolor"}</li>
                        <li>{"Cuidado Post Quirúrgico"}</li>
                        <li>{"Manejo de Medicamentos"}</li>
                        <li>{"Asistencia de Salud en el Hogar"}</li>
                    </ul>
                </ServiceCard>
                
                <ServiceCard title="Terapias">
                    <ul>
                        <li>{"Terapia Física"}</li>
                        <li>{"Terapia del Habla"}</li>
                        <li>{"Terapia Ocupacional"}</li>
                        <li>{"Manejo de Dolor Crónico"}</li>
                        <li>{"Rehabilitación Post Quirúrgica"}</li>
                    </ul>
                </ServiceCard>
            </div>
            
            <div class="row mt-4">
                <ServiceCard title="Trabajo Social Médico">
                    <ul>
                        <li>{"Evaluación de necesidades sociales"}</li>
                        <li>{"Conexión con recursos comunitarios"}</li>
                        <li>{"Apoyo a cuidadores y familiares"}</li>
                        <li>{"Asesoramiento para pacientes"}</li>
                    </ul>
                </ServiceCard>
                
                <ServiceCard title="Nutrición y Dietética">
                    <ul>
                        <li>{"Evaluación nutricional"}</li>
                        <li>{"Planes de alimentación personalizados"}</li>
                        <li>{"Educación sobre dietas especiales"}</li>
                        <li>{"Manejo de condiciones crónicas"}</li>
                    </ul>
                </ServiceCard>
            </div>
        </section>
    }
}
