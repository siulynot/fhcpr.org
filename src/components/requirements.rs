use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
struct RegionCardProps {
    title: String,
    cities: Vec<String>,
}

#[function_component(RegionCard)]
fn region_card(props: &RegionCardProps) -> Html {
    html! {
        <div class="col-md-4 mb-4">
            <div class="card shadow h-100">
                <div class="card-body">
                    <h4 class="region-title">{&props.title}</h4>
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
        <div class="col-6 col-sm-4 col-md-3 mb-4">
            <div class="card insurance-card shadow h-100">
                <img src={props.logo.clone()} class="card-img-top p-3" alt={props.name.clone()} />
            </div>
        </div>
    }
}

#[function_component(Requirements)]
pub fn requirements() -> Html {
    // Region data
    let oeste = vec![
        "Aguada", "Aguadilla", "Añasco", "Cabo Rojo", "Hormigueros", 
        "Isabela", "Lajas", "Las Marías", "Maricao", "Mayagüez", 
        "Moca", "Rincón", "Sabana Grande", "San Germán", "San Sebastián"
    ].into_iter().map(String::from).collect::<Vec<_>>();
    
    let sur = vec![
        "Adjuntas", "Arroyo", "Coamo", "Guayama", "Guayanilla", 
        "Guánica", "Jayuya", "Juana Díaz", "Patillas", "Ponce", 
        "Peñuelas", "Salinas", "Santa Isabel", "Villalba", "Yauco"
    ].into_iter().map(String::from).collect::<Vec<_>>();
    
    let norte = vec![
        "Arecibo", "Barceloneta", "Camuy", "Ciales", "Florida", 
        "Hatillo", "Lares", "Manatí", "Morovis", "Quebradillas", 
        "Utuado", "Vega Baja"
    ].into_iter().map(String::from).collect::<Vec<_>>();
    
    // Insurance data
    let insurances = vec![
        ("images/mmm.png", "MMM"),
        ("images/mcs.png", "MCS"),
        ("images/ssbv.png", "SSBV"),
        ("images/medicare.png", "Medicare"),
        ("images/humana.png", "Humana"),
        ("images/pmc.png", "PMC"),
        ("images/prossam.png", "Prossam"),
    ];

    html! {
        <div id="requisitos" class="bg-light py-5">
            <div class="container requirements-section animate-on-scroll">
                <h2 class="text-center section-title mb-5">{"Requisitos"}</h2>
                
                <section class="mb-5">
                    <h3 class="sub-heading mb-4">{"Servicios de Salud en el Hogar"}</h3>
                    <p class="lead">
                        {r#"First Home Care Center, Inc. es una agencia sin fines de lucro certificada por el Departamento de Salud del Estado Libre Asociado de Puerto Rico y por el Seguro Social Federal. Nuestro propósito es proveer servicios de salud de calidad en el hogar o lugar de residencia a pacientes beneficiarios de "Medicare" y "Medicare Advantage", que cumplan con los requisitos establecidos."#}
                    </p>
                </section>
                
                <section class="mb-5">
                    <h3 class="sub-heading mb-4">{"¿Cómo obtener nuestros servicios?"}</h3>
                    <div class="card shadow">
                        <div class="card-body">
                            <p>
                                {"Su médico de cabecera lo refiere a nuestra agencia si considera que amerita cualquiera de nuestros servicios. La agencia suple los formularios necesarios para el plan de tratamiento y realiza una visita de evaluación para determinar si es elegible o no para nuestros servicios."}
                            </p>
                            <p>
                                {r#"Para cualificar necesita cumplir con los requisitos de participación establecidos por CMS (Center for Medicare Services). Se le hará una notificación escrita a su médico sobre la determinación de su caso. En los casos de "Medicare Advantage", es necesario solicitar aprobación del plan médico para los distintos servicios."#}
                            </p>
                        </div>
                    </div>
                </section>
                
                <section class="mb-5">
                    <h3 class="sub-heading mb-4">{"¿Qué servicios ofrece First Home Care Center, Inc.?"}</h3>
                    <div class="card shadow">
                        <div class="card-body">
                            <p>{"En First Home Care Center, Inc. ofrecemos servicios de:"}</p>
                            <ul>
                                <li>{"Enfermería Profesional"}</li>
                                <li>{"Asistencia de Salud"}</li>
                                <li>{"Terapia Física, Ocupacional y del Habla"}</li>
                                <li>{"Trabajo Médico Social"}</li>
                                <li>{"Consultoría de Nutrición"}</li>
                            </ul>
                            <p>
                                {"Además, ofrecemos todos los procedimientos de Enfermería de acuerdo a la Ley Reguladora de la Profesión de Enfermería y las políticas de Medicare. El alquiler de equipo médico es ofrecido a pacientes que así lo ameriten en coordinación con el doctor de cabecera y otras agencias suplidoras de equipo médico de la comunidad. Siempre se requerirá la orden del médico a cargo del paciente para cualquier servicio."}
                            </p>
                        </div>
                    </div>
                </section>
                
                <section class="mb-5">
                    <h3 class="sub-heading mb-4">{"¿Quién paga por nuestros servicios?"}</h3>
                    <div class="card shadow mb-4">
                        <div class="card-body">
                            <p>
                                {r#"Cualquiera de los planes "Medicare" o "Medicare Advantage" pagan o ayudan a pagar los servicios que se ofrecen a los beneficiarios de estos seguros. Nosotros nos encargamos de facturar directamente a estos planes por los servicios ofrecidos."#}
                            </p>
                        </div>
                    </div>
                    
                    <h4 class="sub-heading mb-4">{"Aceptamos los siguientes planes médicos:"}</h4>
                    <div class="row">
                        {insurances.into_iter().map(|(logo, name)| {
                            html! {
                                <InsuranceCard logo={logo.to_string()} name={name.to_string()} />
                            }
                        }).collect::<Html>()}
                    </div>
                </section>
                
                <section>
                    <h3 class="sub-heading mb-4">{"Ofrecemos nuestros servicios en las siguientes regiones:"}</h3>
                    <div class="row">
                        <RegionCard title={"Oeste".to_string()} cities={oeste} />
                        <RegionCard title={"Sur".to_string()} cities={sur} />
                        <RegionCard title={"Norte".to_string()} cities={norte} />
                    </div>
                </section>
            </div>
        </div>
    }
}
